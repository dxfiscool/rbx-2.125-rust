//! core shard lx — 100 core stubs EA-sorted, next uncovered fallback gap filler (lowest unstubbed EA first).
//! Source: ida/export.json (85545 funcs) global EA asc not yet stubbed in any crate — next 100 uncovered sorted asc.
//! Preserves IDA ea + mangled + demangled for rg; uses rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_FMOD_eatwhite")]
// 0xe2cfc — _FMOD_eatwhite
pub fn stub_0xe2cfc() {
    // IDA 0xe2cfc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_strdup")]
// 0xe2d34 — _FMOD_strdup
pub fn stub_0xe2d34() {
    // IDA 0xe2d34: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_strlenW")]
// 0xe2da0 — _FMOD_strlenW
pub fn stub_0xe2da0() {
    // IDA 0xe2da0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_strncpyW")]
// 0xe2dcc — _FMOD_strncpyW
pub fn stub_0xe2dcc() {
    // IDA 0xe2dcc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_tolowerW")]
// 0xe2dfc — _FMOD_tolowerW
pub fn stub_0xe2dfc() {
    // IDA 0xe2dfc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_strnicmpW")]
// 0xe2e20 — _FMOD_strnicmpW
pub fn stub_0xe2e20() {
    // IDA 0xe2e20: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_wtoa")]
// 0xe2e88 — _FMOD_wtoa
pub fn stub_0xe2e88() {
    // IDA 0xe2e88: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System19setReverbPropertiesEPK22FMOD_REVERB_PROPERTIES")]
// 0xe2ec4 — __ZN4FMOD6System19setReverbPropertiesEPK22FMOD_REVERB_PROPERTIES
pub fn stub_0xe2ec4() {
    // IDA 0xe2ec4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System9playSoundE17FMOD_CHANNELINDEXPNS_5SoundEbPPNS_7ChannelE")]
// 0xe2f00 — __ZN4FMOD6System9playSoundE17FMOD_CHANNELINDEXPNS_5SoundEbPPNS_7ChannelE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xe2f00() {
    // IDA 0xe2f00: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System16createSoundGroupEPKcPPNS_10SoundGroupE")]
// 0xe2f4c — __ZN4FMOD6System16createSoundGroupEPKcPPNS_10SoundGroupE
// type: _DWORD __fastcall(FMOD::System *__hidden this, const char *, FMOD::SoundGroup **)
pub fn stub_0xe2f4c() {
    // IDA 0xe2f4c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System18createChannelGroupEPKcPPNS_12ChannelGroupE")]
// 0xe2f88 — __ZN4FMOD6System18createChannelGroupEPKcPPNS_12ChannelGroupE
// type: _DWORD __fastcall(FMOD::System *__hidden this, const char *, FMOD::ChannelGroup **)
pub fn stub_0xe2f88() {
    // IDA 0xe2f88: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System11createSoundEPKcjP22FMOD_CREATESOUNDEXINFOPPNS_5SoundE")]
// 0xe2fc4 — __ZN4FMOD6System11createSoundEPKcjP22FMOD_CREATESOUNDEXINFOPPNS_5SoundE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xe2fc4() {
    // IDA 0xe2fc4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System11getCPUUsageEPfS1_S1_S1_S1_")]
// 0xe3010 — __ZN4FMOD6System11getCPUUsageEPfS1_S1_S1_S1_
// type: _DWORD __fastcall(FMOD::System *__hidden this, float *, float *, float *, float *, float *)
pub fn stub_0xe3010() {
    // IDA 0xe3010: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System18getChannelsPlayingEPi")]
// 0xe3064 — __ZN4FMOD6System18getChannelsPlayingEPi
// type: _DWORD __fastcall(FMOD::System *__hidden this, int *)
pub fn stub_0xe3064() {
    // IDA 0xe3064: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System10getVersionEPj")]
// 0xe3098 — __ZN4FMOD6System10getVersionEPj
// type: _DWORD __fastcall(FMOD::System *__hidden this, unsigned int *)
pub fn stub_0xe3098() {
    // IDA 0xe3098: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System23set3DListenerAttributesEiPK11FMOD_VECTORS3_S3_S3_")]
// 0xe30cc — __ZN4FMOD6System23set3DListenerAttributesEiPK11FMOD_VECTORS3_S3_S3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xe30cc() {
    // IDA 0xe30cc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System13set3DSettingsEfff")]
// 0xe3120 — __ZN4FMOD6System13set3DSettingsEfff
// type: _DWORD __fastcall(FMOD::System *__hidden this, float, float, float)
pub fn stub_0xe3120() {
    // IDA 0xe3120: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System6updateEv")]
// 0xe3164 — __ZN4FMOD6System6updateEv
// type: _DWORD __fastcall(FMOD::System *__hidden this)
pub fn stub_0xe3164() {
    // IDA 0xe3164: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System4initEijPv")]
// 0xe3190 — __ZN4FMOD6System4initEijPv
// type: _DWORD __fastcall(FMOD::System *__hidden this, int, unsigned int, void *)
pub fn stub_0xe3190() {
    // IDA 0xe3190: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System14setSpeakerModeE16FMOD_SPEAKERMODE")]
// 0xe31d4 — __ZN4FMOD6System14setSpeakerModeE16FMOD_SPEAKERMODE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xe31d4() {
    // IDA 0xe31d4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System16setDSPBufferSizeEji")]
// 0xe3208 — __ZN4FMOD6System16setDSPBufferSizeEji
// type: _DWORD __fastcall(FMOD::System *__hidden this, unsigned int, int)
pub fn stub_0xe3208() {
    // IDA 0xe3208: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System9getDriverEPi")]
// 0xe3244 — __ZN4FMOD6System9getDriverEPi
// type: _DWORD __fastcall(FMOD::System *__hidden this, int *)
pub fn stub_0xe3244() {
    // IDA 0xe3244: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System9setDriverEi")]
// 0xe3278 — __ZN4FMOD6System9setDriverEi
// type: _DWORD __fastcall(FMOD::System *__hidden this, int)
pub fn stub_0xe3278() {
    // IDA 0xe3278: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System13getDriverCapsEiPjPiS2_P16FMOD_SPEAKERMODE")]
// 0xe32ac — __ZN4FMOD6System13getDriverCapsEiPjPiS2_P16FMOD_SPEAKERMODE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xe32ac() {
    // IDA 0xe32ac: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System13getDriverInfoEiPciP9FMOD_GUID")]
// 0xe3300 — __ZN4FMOD6System13getDriverInfoEiPciP9FMOD_GUID
// type: int __fastcall(_DWORD)
pub fn stub_0xe3300() {
    // IDA 0xe3300: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__ZN4FMOD6System13getNumDriversEPi")]
// 0xe334c — __ZN4FMOD6System13getNumDriversEPi
// type: _DWORD __fastcall(FMOD::System *__hidden this, int *)
pub fn stub_0xe334c() {
    // IDA 0xe334c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_TT_Set_MM_Blend")]
// 0x2235a8 — _TT_Set_MM_Blend
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_0x2235a8() {
    // IDA 0x2235a8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Set_Var_Design")]
// 0x223c90 — _TT_Set_Var_Design
// type: int __fastcall(int, int, int)
pub fn stub_0x223c90() {
    // IDA 0x223c90: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_New_Context")]
// 0x224298 — _TT_New_Context
// type: _DWORD *__fastcall(int)
pub fn stub_0x224298() {
    // IDA 0x224298: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_driver_init")]
// 0x224378 — _tt_driver_init
// type: int __fastcall(int)
pub fn stub_0x224378() {
    // IDA 0x224378: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_VecLen")]
// 0x224394 — _TT_VecLen
// type: int __fastcall(int, int)
pub fn stub_0x224394() {
    // IDA 0x224394: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Normalize")]
// 0x2243b8 — _Normalize
// type: int __fastcall(int, int, int, _WORD *)
pub fn stub_0x2243b8() {
    // IDA 0x2243b8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Ins_SxVTL")]
// 0x224510 — _Ins_SxVTL
// type: int __fastcall(int, unsigned __int16, unsigned __int16, char, _WORD *)
pub fn stub_0x224510() {
    // IDA 0x224510: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Current_Ratio")]
// 0x2245a8 — _Current_Ratio
// type: int __fastcall(int)
pub fn stub_0x2245a8() {
    // IDA 0x2245a8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Move_CVT_Stretched")]
// 0x224628 — _Move_CVT_Stretched
// type: int __fastcall(int, int, int)
pub fn stub_0x224628() {
    // IDA 0x224628: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Write_CVT_Stretched")]
// 0x224664 — _Write_CVT_Stretched
// type: int __fastcall(int, int, int)
pub fn stub_0x224664() {
    // IDA 0x224664: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Read_CVT_Stretched")]
// 0x224690 — _Read_CVT_Stretched
// type: int __fastcall(int, int)
pub fn stub_0x224690() {
    // IDA 0x224690: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Current_Ppem")]
// 0x2246b4 — _Current_Ppem
// type: int __fastcall(int)
pub fn stub_0x2246b4() {
    // IDA 0x2246b4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Ins_DELTAP")]
// 0x2246d4 — _Ins_DELTAP
// type: unsigned int __fastcall(unsigned int result, unsigned int *)
pub fn stub_0x2246d4() {
    // IDA 0x2246d4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_RunIns")]
// 0x2248d0 — _TT_RunIns
// type: int __fastcall(unsigned int)
pub fn stub_0x2248d0() {
    // IDA 0x2248d0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_compare_kern_pairs")]
// 0x22869c — _compare_kern_pairs
// type: int __fastcall(_DWORD *, _DWORD *)
pub fn stub_0x22869c() {
    // IDA 0x22869c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Get_Kerning")]
// 0x2286cc — _T1_Get_Kerning
// type: _DWORD *__fastcall(int, int, int, _DWORD *)
pub fn stub_0x2286cc() {
    // IDA 0x2286cc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Get_Track_Kerning")]
// 0x22874c — _T1_Get_Track_Kerning
// type: int __fastcall(int, int, int, _DWORD *)
pub fn stub_0x22874c() {
    // IDA 0x22874c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Done_Metrics")]
// 0x228814 — _T1_Done_Metrics
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x228814() {
    // IDA 0x228814: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Read_Metrics")]
// 0x22885c — _T1_Read_Metrics
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x22885c() {
    // IDA 0x22885c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t1_get_index")]
// 0x228c94 — _t1_get_index
// type: int __fastcall(const char *, size_t, int)
pub fn stub_0x228c94() {
    // IDA 0x228c94: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t1_get_ps_name")]
// 0x2291b0 — _t1_get_ps_name
// type: int __fastcall(int)
pub fn stub_0x2291b0() {
    // IDA 0x2291b0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t1_ps_get_font_info")]
// 0x2291b8 — _t1_ps_get_font_info
// type: int __fastcall(_DWORD *, _DWORD *)
pub fn stub_0x2291b8() {
    // IDA 0x2291b8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t1_ps_get_font_extra")]
// 0x2291e8 — _t1_ps_get_font_extra
// type: int __fastcall(int, _WORD *)
pub fn stub_0x2291e8() {
    // IDA 0x2291e8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t1_ps_has_glyph_names")]
// 0x2291f8 — _t1_ps_has_glyph_names
// type: int()
pub fn stub_0x2291f8() {
    // IDA 0x2291f8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t1_ps_get_font_private")]
// 0x229200 — _t1_ps_get_font_private
// type: int __fastcall(int, void *__dst)
pub fn stub_0x229200() {
    // IDA 0x229200: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Get_Kerning")]
// 0x229224 — _Get_Kerning
// type: int __fastcall(int, int, int, _DWORD *)
pub fn stub_0x229224() {
    // IDA 0x229224: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Get_Interface")]
// 0x229250 — _Get_Interface
// type: int __fastcall(int, char *)
pub fn stub_0x229250() {
    // IDA 0x229250: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t1_get_name_index")]
// 0x22926c — _t1_get_name_index
// type: int __fastcall(int, char *__s1)
pub fn stub_0x22926c() {
    // IDA 0x22926c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t1_get_glyph_name_0")]
// 0x2293b0 — _t1_get_glyph_name_0
// type: int __fastcall(int, int, int, int)
pub fn stub_0x2293b0() {
    // IDA 0x2293b0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Parse_Glyph_And_Get_Char_String")]
// 0x2293d4 — _T1_Parse_Glyph_And_Get_Char_String
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_0x2293d4() {
    // IDA 0x2293d4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Parse_Glyph")]
// 0x229524 — _T1_Parse_Glyph
// type: int __fastcall(int, int)
pub fn stub_0x229524() {
    // IDA 0x229524: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Get_Advances")]
// 0x229574 — _T1_Get_Advances
// type: int __fastcall(_DWORD *, int, int, char, int *)
pub fn stub_0x229574() {
    // IDA 0x229574: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Compute_Max_Advance")]
// 0x2298b4 — _T1_Compute_Max_Advance
// type: int __fastcall(_DWORD *, _DWORD *)
pub fn stub_0x2298b4() {
    // IDA 0x2298b4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Load_Glyph")]
// 0x2299c0 — _T1_Load_Glyph
// type: int __fastcall(int, int, unsigned int, int)
pub fn stub_0x2299c0() {
    // IDA 0x2299c0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Get_Multi_Master")]
// 0x229f90 — _T1_Get_Multi_Master
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x229f90() {
    // IDA 0x229f90: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_mm_weights_unmap")]
// 0x22a018 — _mm_weights_unmap
// type: int __fastcall(_DWORD *, int *, int)
pub fn stub_0x22a018() {
    // IDA 0x22a018: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_parse_buildchar")]
// 0x22a1c0 — _parse_buildchar
// type: int __fastcall(int, int)
pub fn stub_0x22a1c0() {
    // IDA 0x22a1c0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_parse_private")]
// 0x22a1ec — _parse_private
// type: int __fastcall(int, int)
pub fn stub_0x22a1ec() {
    // IDA 0x22a1ec: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_read_binary_data")]
// 0x22a1fc — _read_binary_data
// type: bool __fastcall(unsigned __int8 **, _DWORD *, _DWORD *)
pub fn stub_0x22a1fc() {
    // IDA 0x22a1fc: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_parse_encoding")]
// 0x22a29c — _parse_encoding
// type: const char *__fastcall(_DWORD *, int)
pub fn stub_0x22a29c() {
    // IDA 0x22a29c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_parse_dict")]
// 0x22a6e4 — _parse_dict
// type: int __fastcall(_DWORD *, unsigned __int8 **, unsigned __int8 *, int)
pub fn stub_0x22a6e4() {
    // IDA 0x22a6e4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_t1_allocate_blend")]
// 0x22ac6c — _t1_allocate_blend
// type: int __fastcall(int, unsigned int, unsigned int)
pub fn stub_0x22ac6c() {
    // IDA 0x22ac6c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_parse_weight_vector")]
// 0x22b1b8 — _parse_weight_vector
// type: int __fastcall(int, int *)
pub fn stub_0x22b1b8() {
    // IDA 0x22b1b8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_parse_blend_design_map")]
// 0x22b2d4 — _parse_blend_design_map
// type: int __fastcall(int, int *)
pub fn stub_0x22b2d4() {
    // IDA 0x22b2d4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_parse_blend_design_positions")]
// 0x22b4ac — _parse_blend_design_positions
// type: int __fastcall(int, int *)
pub fn stub_0x22b4ac() {
    // IDA 0x22b4ac: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_parse_subrs")]
// 0x22b658 — _parse_subrs
// type: int __fastcall(int, int)
pub fn stub_0x22b658() {
    // IDA 0x22b658: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Done_Blend")]
// 0x22b8fc — _T1_Done_Blend
// type: int __fastcall(int result)
pub fn stub_0x22b8fc() {
    // IDA 0x22b8fc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_parse_font_matrix_0")]
// 0x22bfd4 — _parse_font_matrix_0
// type: int __fastcall(int, int)
pub fn stub_0x22bfd4() {
    // IDA 0x22bfd4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Open_Face")]
// 0x22c0dc — _T1_Open_Face
// type: int __fastcall(int)
pub fn stub_0x22c0dc() {
    // IDA 0x22c0dc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_mm_axis_unmap")]
// 0x22c82c — _mm_axis_unmap
// type: int __fastcall(unsigned __int8 *, int)
pub fn stub_0x22c82c() {
    // IDA 0x22c82c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Get_MM_Var")]
// 0x22ca3c — _T1_Get_MM_Var
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x22ca3c() {
    // IDA 0x22ca3c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Set_MM_Blend")]
// 0x22cc68 — _T1_Set_MM_Blend
// type: int __fastcall(int, int, int)
pub fn stub_0x22cc68() {
    // IDA 0x22cc68: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Set_MM_Design")]
// 0x22cd04 — _T1_Set_MM_Design
// type: int __fastcall(int, int, int)
pub fn stub_0x22cd04() {
    // IDA 0x22cd04: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Set_Var_Design")]
// 0x22cff0 — _T1_Set_Var_Design
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_0x22cff0() {
    // IDA 0x22cff0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_parse_blend_axis_types")]
// 0x22d1b8 — _parse_blend_axis_types
// type: _BYTE *__fastcall(int, int)
pub fn stub_0x22d1b8() {
    // IDA 0x22d1b8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_parse_charstrings")]
// 0x22d2d8 — _parse_charstrings
// type: int __fastcall(int, int)
pub fn stub_0x22d2d8() {
    // IDA 0x22d2d8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_GlyphSlot_Done")]
// 0x22d940 — _T1_GlyphSlot_Done
// type: int __fastcall(int result)
pub fn stub_0x22d940() {
    // IDA 0x22d940: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Driver_Init")]
// 0x22d950 — _T1_Driver_Init
// type: int()
pub fn stub_0x22d950() {
    // IDA 0x22d950: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Driver_Done")]
// 0x22d958 — _T1_Driver_Done
// type: void()
pub fn stub_0x22d958() {
    // IDA 0x22d958: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Face_Init")]
// 0x22d95c — _T1_Face_Init
// type: int __fastcall(int, int, int)
pub fn stub_0x22d95c() {
    // IDA 0x22d95c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Face_Done")]
// 0x22dd78 — _T1_Face_Done
// type: int __fastcall(int result)
pub fn stub_0x22dd78() {
    // IDA 0x22dd78: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_GlyphSlot_Init")]
// 0x22dedc — _T1_GlyphSlot_Init
// type: int __fastcall(int)
pub fn stub_0x22dedc() {
    // IDA 0x22dedc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Size_Get_Globals_Funcs")]
// 0x22df30 — _T1_Size_Get_Globals_Funcs
// type: int __fastcall(int)
pub fn stub_0x22df30() {
    // IDA 0x22df30: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Size_Init")]
// 0x22df80 — _T1_Size_Init
// type: int (__fastcall **__fastcall(_DWORD *))(_DWORD, int, int *)
pub fn stub_0x22df80() {
    // IDA 0x22df80: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Size_Done")]
// 0x22dfc8 — _T1_Size_Done
// type: int __fastcall(int result)
pub fn stub_0x22dfc8() {
    // IDA 0x22dfc8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Size_Request")]
// 0x22e000 — _T1_Size_Request
// type: int __fastcall(_DWORD *, int)
pub fn stub_0x22e000() {
    // IDA 0x22e000: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_read_pfb_tag")]
// 0x22e058 — _read_pfb_tag
// type: int __fastcall(int, _WORD *, int *)
pub fn stub_0x22e058() {
    // IDA 0x22e058: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Get_Private_Dict")]
// 0x22e0e0 — _T1_Get_Private_Dict
// type: int __fastcall(int, int)
pub fn stub_0x22e0e0() {
    // IDA 0x22e0e0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_Finalize_Parser")]
// 0x22ea7c — _T1_Finalize_Parser
// type: int __fastcall(int)
pub fn stub_0x22ea7c() {
    // IDA 0x22ea7c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_check_type1_format")]
// 0x22eacc — _check_type1_format
// type: int __fastcall(int, const void *, size_t)
pub fn stub_0x22eacc() {
    // IDA 0x22eacc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_T1_New_Parser")]
// 0x22eb7c — _T1_New_Parser
// type: int __fastcall(int, _DWORD *, int, int)
pub fn stub_0x22eb7c() {
    // IDA 0x22eb7c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t42_get_ps_font_name")]
// 0x22ed68 — _t42_get_ps_font_name
// type: int __fastcall(int)
pub fn stub_0x22ed68() {
    // IDA 0x22ed68: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t42_ps_get_font_info")]
// 0x22ed70 — _t42_ps_get_font_info
// type: int __fastcall(_DWORD *, _DWORD *)
pub fn stub_0x22ed70() {
    // IDA 0x22ed70: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_t42_ps_get_font_extra")]
// 0x22eda0 — _t42_ps_get_font_extra
// type: int __fastcall(int, _WORD *)
pub fn stub_0x22eda0() {
    // IDA 0x22eda0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
