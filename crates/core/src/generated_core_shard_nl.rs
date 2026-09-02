//! core shard nl — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet stubbed in core (lowest EA uncovered 0xe9e54..0x1012fc, 42380 distinct in core before batch, 43166 uncovered, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + pub fn stub_0xADDR() -> ! { todo!("0xADDR mangled") }
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "FMOD::SystemI::SystemI(void)")]
#[doc(alias = "__ZN4FMOD7SystemIC2Ev")]
// 0xe9e54 — __ZN4FMOD7SystemIC2Ev
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xe9e54() -> ! { todo!("0xe9e54 __ZN4FMOD7SystemIC2Ev") }

#[doc(alias = "FMOD::SystemI::SystemI(void)")]
#[doc(alias = "__ZN4FMOD7SystemIC1Ev")]
// 0xea4ac — __ZN4FMOD7SystemIC1Ev
// type: _DWORD __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xea4ac() -> ! { todo!("0xea4ac __ZN4FMOD7SystemIC1Ev") }

#[doc(alias = "FMOD::SystemI::createSoundInternal(char const*,unsigned int,unsigned int,unsigned int,FMOD_CREATESOUNDEXINFO *,bool,FMOD::SoundI **)")]
#[doc(alias = "__ZN4FMOD7SystemI19createSoundInternalEPKcjjjP22FMOD_CREATESOUNDEXINFObPPNS_6SoundIE")]
// 0xea4b0 — __ZN4FMOD7SystemI19createSoundInternalEPKcjjjP22FMOD_CREATESOUNDEXINFObPPNS_6SoundIE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xea4b0() -> ! { todo!("0xea4b0 __ZN4FMOD7SystemI19createSoundInternalEPKcjjjP22FMOD_CREATESOUNDEXINFObPPNS_6SoundIE") }

#[doc(alias = "FMOD::SystemI::createSound(char const*,unsigned int,FMOD_CREATESOUNDEXINFO *,FMOD::SoundI **)")]
#[doc(alias = "__ZN4FMOD7SystemI11createSoundEPKcjP22FMOD_CREATESOUNDEXINFOPPNS_6SoundIE")]
// 0xed7e4 — __ZN4FMOD7SystemI11createSoundEPKcjP22FMOD_CREATESOUNDEXINFOPPNS_6SoundIE
// type: int __fastcall(FMOD::SystemI *this, int, int, void *__src, int)
pub fn stub_0xed7e4() -> ! { todo!("0xed7e4 __ZN4FMOD7SystemI11createSoundEPKcjP22FMOD_CREATESOUNDEXINFOPPNS_6SoundIE") }

#[doc(alias = "FMOD::SystemI::~SystemI()")]
#[doc(alias = "__ZN4FMOD7SystemID0Ev")]
// 0xedce8 — __ZN4FMOD7SystemID0Ev
// type: void __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xedce8() -> ! { todo!("0xedce8 __ZN4FMOD7SystemID0Ev") }

#[doc(alias = "FMOD::SystemI::~SystemI()")]
#[doc(alias = "__ZN4FMOD7SystemID1Ev")]
// 0xedd54 — __ZN4FMOD7SystemID1Ev
// type: void __fastcall(FMOD::SystemI *__hidden this)
pub fn stub_0xedd54() -> ! { todo!("0xedd54 __ZN4FMOD7SystemID1Ev") }

#[doc(alias = "FMOD::Thread::threadFunc(void)")]
#[doc(alias = "__ZN4FMOD6Thread10threadFuncEv")]
// 0xeddb8 — __ZN4FMOD6Thread10threadFuncEv
// type: _DWORD __fastcall(FMOD::Thread *__hidden this)
pub fn stub_0xeddb8() -> ! { todo!("0xeddb8 __ZN4FMOD6Thread10threadFuncEv") }

#[doc(alias = "FMOD::Thread::Thread(void)")]
#[doc(alias = "__ZN4FMOD6ThreadC2Ev")]
// 0xeddc0 — __ZN4FMOD6ThreadC2Ev
// type: _DWORD __fastcall(FMOD::Thread *__hidden this)
pub fn stub_0xeddc0() -> ! { todo!("0xeddc0 __ZN4FMOD6ThreadC2Ev") }

#[doc(alias = "FMOD::Thread::Thread(void)")]
#[doc(alias = "__ZN4FMOD6ThreadC1Ev")]
// 0xeddfc — __ZN4FMOD6ThreadC1Ev
// type: _DWORD __fastcall(FMOD::Thread *__hidden this)
pub fn stub_0xeddfc() -> ! { todo!("0xeddfc __ZN4FMOD6ThreadC1Ev") }

#[doc(alias = "FMOD::Thread::wakeupThread(bool)")]
#[doc(alias = "__ZN4FMOD6Thread12wakeupThreadEb")]
// 0xede00 — __ZN4FMOD6Thread12wakeupThreadEb
// type: _DWORD __fastcall(FMOD::Thread *__hidden this, bool)
pub fn stub_0xede00() -> ! { todo!("0xede00 __ZN4FMOD6Thread12wakeupThreadEb") }

#[doc(alias = "FMOD::Thread::closeThread(void)")]
#[doc(alias = "__ZN4FMOD6Thread11closeThreadEv")]
// 0xede20 — __ZN4FMOD6Thread11closeThreadEv
// type: _DWORD __fastcall(FMOD::Thread *__hidden this)
pub fn stub_0xede20() -> ! { todo!("0xede20 __ZN4FMOD6Thread11closeThreadEv") }

#[doc(alias = "FMOD::Thread::callback(void *)")]
#[doc(alias = "__ZN4FMOD6Thread8callbackEPv")]
// 0xedef4 — __ZN4FMOD6Thread8callbackEPv
// type: _DWORD __fastcall(FMOD::Thread *__hidden this, void *)
pub fn stub_0xedef4() -> ! { todo!("0xedef4 __ZN4FMOD6Thread8callbackEPv") }

#[doc(alias = "FMOD::Thread::initThread(char const*,void (*)(void *),void *,FMOD::Thread::PRIORITY,void *,int,bool,int,FMOD::SystemI *)")]
#[doc(alias = "__ZN4FMOD6Thread10initThreadEPKcPFvPvES3_NS0_8PRIORITYES3_ibiPNS_7SystemIE")]
// 0xedf94 — __ZN4FMOD6Thread10initThreadEPKcPFvPvES3_NS0_8PRIORITYES3_ibiPNS_7SystemIE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xedf94() -> ! { todo!("0xedf94 __ZN4FMOD6Thread10initThreadEPKcPFvPvES3_NS0_8PRIORITYES3_ibiPNS_7SystemIE") }

#[doc(alias = "FMOD::Thread::~Thread()")]
#[doc(alias = "__ZN4FMOD6ThreadD0Ev")]
// 0xee144 — __ZN4FMOD6ThreadD0Ev
// type: void __fastcall(FMOD::Thread *__hidden this)
pub fn stub_0xee144() -> ! { todo!("0xee144 __ZN4FMOD6ThreadD0Ev") }

#[doc(alias = "FMOD::Thread::~Thread()")]
#[doc(alias = "__ZN4FMOD6ThreadD1Ev")]
// 0xee168 — __ZN4FMOD6ThreadD1Ev
// type: void __fastcall(FMOD::Thread *__hidden this)
pub fn stub_0xee168() -> ! { todo!("0xee168 __ZN4FMOD6ThreadD1Ev") }

#[doc(alias = "FMOD::TimeStamp::TimeStamp(void)")]
#[doc(alias = "__ZN4FMOD9TimeStampC2Ev")]
// 0xee180 — __ZN4FMOD9TimeStampC2Ev
// type: _DWORD __fastcall(FMOD::TimeStamp *__hidden this)
pub fn stub_0xee180() -> ! { todo!("0xee180 __ZN4FMOD9TimeStampC2Ev") }

#[doc(alias = "FMOD::TimeStamp::TimeStamp(void)")]
#[doc(alias = "__ZN4FMOD9TimeStampC1Ev")]
// 0xee1b0 — __ZN4FMOD9TimeStampC1Ev
// type: _DWORD __fastcall(FMOD::TimeStamp *__hidden this)
pub fn stub_0xee1b0() -> ! { todo!("0xee1b0 __ZN4FMOD9TimeStampC1Ev") }

#[doc(alias = "FMOD::TimeStamp::getCPUUsage(float *)")]
#[doc(alias = "__ZN4FMOD9TimeStamp11getCPUUsageEPf")]
// 0xee1b4 — __ZN4FMOD9TimeStamp11getCPUUsageEPf
// type: _DWORD __fastcall(FMOD::TimeStamp *__hidden this, float *)
pub fn stub_0xee1b4() -> ! { todo!("0xee1b4 __ZN4FMOD9TimeStamp11getCPUUsageEPf") }

#[doc(alias = "FMOD::TimeStamp::setPaused(bool)")]
#[doc(alias = "__ZN4FMOD9TimeStamp9setPausedEb")]
// 0xee1cc — __ZN4FMOD9TimeStamp9setPausedEb
// type: _DWORD __fastcall(FMOD::TimeStamp *__hidden this, bool)
pub fn stub_0xee1cc() -> ! { todo!("0xee1cc __ZN4FMOD9TimeStamp9setPausedEb") }

#[doc(alias = "FMOD::TimeStamp::stampOut(int)")]
#[doc(alias = "__ZN4FMOD9TimeStamp8stampOutEi")]
// 0xee260 — __ZN4FMOD9TimeStamp8stampOutEi
// type: _DWORD __fastcall(FMOD::TimeStamp *__hidden this, int)
pub fn stub_0xee260() -> ! { todo!("0xee260 __ZN4FMOD9TimeStamp8stampOutEi") }

#[doc(alias = "FMOD::TimeStamp::stampIn(void)")]
#[doc(alias = "__ZN4FMOD9TimeStamp7stampInEv")]
// 0xee344 — __ZN4FMOD9TimeStamp7stampInEv
// type: _DWORD __fastcall(FMOD::TimeStamp *__hidden this)
pub fn stub_0xee344() -> ! { todo!("0xee344 __ZN4FMOD9TimeStamp7stampInEv") }

#[doc(alias = "_FMOD_ogg_page_version")]
// 0xee42c — _FMOD_ogg_page_version
pub fn stub_0xee42c() -> ! { todo!("0xee42c _FMOD_ogg_page_version") }

#[doc(alias = "_FMOD_ogg_page_continued")]
// 0xee438 — _FMOD_ogg_page_continued
pub fn stub_0xee438() -> ! { todo!("0xee438 _FMOD_ogg_page_continued") }

#[doc(alias = "_FMOD_ogg_page_bos")]
// 0xee448 — _FMOD_ogg_page_bos
// type: int __fastcall(_DWORD)
pub fn stub_0xee448() -> ! { todo!("0xee448 _FMOD_ogg_page_bos") }

#[doc(alias = "_FMOD_ogg_page_eos")]
// 0xee458 — _FMOD_ogg_page_eos
pub fn stub_0xee458() -> ! { todo!("0xee458 _FMOD_ogg_page_eos") }

#[doc(alias = "_FMOD_ogg_page_granulepos")]
// 0xee468 — _FMOD_ogg_page_granulepos
// type: __int64 __fastcall(_DWORD)
pub fn stub_0xee468() -> ! { todo!("0xee468 _FMOD_ogg_page_granulepos") }

#[doc(alias = "_FMOD_ogg_page_serialno")]
// 0xee53c — _FMOD_ogg_page_serialno
// type: int __fastcall(_DWORD)
pub fn stub_0xee53c() -> ! { todo!("0xee53c _FMOD_ogg_page_serialno") }

#[doc(alias = "_FMOD_ogg_page_pageno")]
// 0xee564 — _FMOD_ogg_page_pageno
pub fn stub_0xee564() -> ! { todo!("0xee564 _FMOD_ogg_page_pageno") }

#[doc(alias = "_FMOD_ogg_stream_check")]
// 0xee58c — _FMOD_ogg_stream_check
pub fn stub_0xee58c() -> ! { todo!("0xee58c _FMOD_ogg_stream_check") }

#[doc(alias = "_FMOD_ogg_page_checksum_set")]
// 0xee5a4 — _FMOD_ogg_page_checksum_set
pub fn stub_0xee5a4() -> ! { todo!("0xee5a4 _FMOD_ogg_page_checksum_set") }

#[doc(alias = "_FMOD_ogg_sync_check")]
// 0xee688 — _FMOD_ogg_sync_check
pub fn stub_0xee688() -> ! { todo!("0xee688 _FMOD_ogg_sync_check") }

#[doc(alias = "_FMOD_ogg_sync_wrote")]
// 0xee69c — _FMOD_ogg_sync_wrote
pub fn stub_0xee69c() -> ! { todo!("0xee69c _FMOD_ogg_sync_wrote") }

#[doc(alias = "_FMOD_ogg_sync_reset")]
// 0xee6dc — _FMOD_ogg_sync_reset
pub fn stub_0xee6dc() -> ! { todo!("0xee6dc _FMOD_ogg_sync_reset") }

#[doc(alias = "_FMOD_ogg_stream_reset")]
// 0xee70c — _FMOD_ogg_stream_reset
pub fn stub_0xee70c() -> ! { todo!("0xee70c _FMOD_ogg_stream_reset") }

#[doc(alias = "_FMOD_ogg_stream_reset_serialno")]
// 0xee76c — _FMOD_ogg_stream_reset_serialno
pub fn stub_0xee76c() -> ! { todo!("0xee76c _FMOD_ogg_stream_reset_serialno") }

#[doc(alias = "_FMOD_ogg_stream_packetout")]
// 0xee8dc — _FMOD_ogg_stream_packetout
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xee8dc() -> ! { todo!("0xee8dc _FMOD_ogg_stream_packetout") }

#[doc(alias = "_FMOD_ogg_stream_packetpeek")]
// 0xee914 — _FMOD_ogg_stream_packetpeek
pub fn stub_0xee914() -> ! { todo!("0xee914 _FMOD_ogg_stream_packetpeek") }

#[doc(alias = "_FMOD_ogg_sync_clear")]
// 0xee948 — _FMOD_ogg_sync_clear
pub fn stub_0xee948() -> ! { todo!("0xee948 _FMOD_ogg_sync_clear") }

#[doc(alias = "_FMOD_ogg_sync_buffer")]
// 0xee998 — _FMOD_ogg_sync_buffer
pub fn stub_0xee998() -> ! { todo!("0xee998 _FMOD_ogg_sync_buffer") }

#[doc(alias = "_FMOD_ogg_sync_init")]
// 0xeea80 — _FMOD_ogg_sync_init
pub fn stub_0xeea80() -> ! { todo!("0xeea80 _FMOD_ogg_sync_init") }

#[doc(alias = "_FMOD_ogg_stream_clear")]
// 0xeeab8 — _FMOD_ogg_stream_clear
pub fn stub_0xeeab8() -> ! { todo!("0xeeab8 _FMOD_ogg_stream_clear") }

#[doc(alias = "_FMOD_ogg_stream_pagein")]
// 0xeeb1c — _FMOD_ogg_stream_pagein
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xeeb1c() -> ! { todo!("0xeeb1c _FMOD_ogg_stream_pagein") }

#[doc(alias = "_FMOD_ogg_stream_init")]
// 0xeefdc — _FMOD_ogg_stream_init
pub fn stub_0xeefdc() -> ! { todo!("0xeefdc _FMOD_ogg_stream_init") }

#[doc(alias = "_FMOD_ogg_sync_pageseek")]
// 0xef088 — _FMOD_ogg_sync_pageseek
pub fn stub_0xef088() -> ! { todo!("0xef088 _FMOD_ogg_sync_pageseek") }

#[doc(alias = "_FMOD_vorbis_info_blocksize")]
// 0xef274 — _FMOD_vorbis_info_blocksize
pub fn stub_0xef274() -> ! { todo!("0xef274 _FMOD_vorbis_info_blocksize") }

#[doc(alias = "_FMOD_vorbis_synthesis_idheader")]
// 0xef2cc — _FMOD_vorbis_synthesis_idheader
pub fn stub_0xef2cc() -> ! { todo!("0xef2cc _FMOD_vorbis_synthesis_idheader") }

#[doc(alias = "_FMOD_vorbis_comment_clear")]
// 0xef35c — _FMOD_vorbis_comment_clear
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xef35c() -> ! { todo!("0xef35c _FMOD_vorbis_comment_clear") }

#[doc(alias = "_FMOD_vorbis_comment_init")]
// 0xef408 — _FMOD_vorbis_comment_init
pub fn stub_0xef408() -> ! { todo!("0xef408 _FMOD_vorbis_comment_init") }

#[doc(alias = "_FMOD_vorbis_info_clear")]
// 0xef428 — _FMOD_vorbis_info_clear
pub fn stub_0xef428() -> ! { todo!("0xef428 _FMOD_vorbis_info_clear") }

#[doc(alias = "_FMOD_vorbis_synthesis_headerin")]
// 0xef630 — _FMOD_vorbis_synthesis_headerin
pub fn stub_0xef630() -> ! { todo!("0xef630 _FMOD_vorbis_synthesis_headerin") }

#[doc(alias = "_FMOD_vorbis_info_init")]
// 0xefd04 — _FMOD_vorbis_info_init
pub fn stub_0xefd04() -> ! { todo!("0xefd04 _FMOD_vorbis_info_init") }

#[doc(alias = "_FMOD_mapping0_free_info")]
// 0xf17a0 — _FMOD_mapping0_free_info
pub fn stub_0xf17a0() -> ! { todo!("0xf17a0 _FMOD_mapping0_free_info") }

#[doc(alias = "_FMOD_mapping0_unpack")]
// 0xf17d4 — _FMOD_mapping0_unpack
pub fn stub_0xf17d4() -> ! { todo!("0xf17d4 _FMOD_mapping0_unpack") }

#[doc(alias = "_FMOD_mapping0_inverse")]
// 0xf1a1c — _FMOD_mapping0_inverse
pub fn stub_0xf1a1c() -> ! { todo!("0xf1a1c _FMOD_mapping0_inverse") }

#[doc(alias = "_FMOD_mdct_clear")]
// 0xf2f34 — _FMOD_mdct_clear
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2f34() -> ! { todo!("0xf2f34 _FMOD_mdct_clear") }

#[doc(alias = "_FMOD_mdct_init")]
// 0xf2f90 — _FMOD_mdct_init
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf2f90() -> ! { todo!("0xf2f90 _FMOD_mdct_init") }

#[doc(alias = "_FMOD_mdct_backward")]
// 0xf3bfc — _FMOD_mdct_backward
pub fn stub_0xf3bfc() -> ! { todo!("0xf3bfc _FMOD_mdct_backward") }

#[doc(alias = "_FMOD_res2_inverse")]
// 0xf4120 — _FMOD_res2_inverse
pub fn stub_0xf4120() -> ! { todo!("0xf4120 _FMOD_res2_inverse") }

#[doc(alias = "__FMOD_01inverse")]
// 0xf4374 — __FMOD_01inverse
pub fn stub_0xf4374() -> ! { todo!("0xf4374 __FMOD_01inverse") }

#[doc(alias = "_FMOD_res1_inverse")]
// 0xf4650 — _FMOD_res1_inverse
pub fn stub_0xf4650() -> ! { todo!("0xf4650 _FMOD_res1_inverse") }

#[doc(alias = "_FMOD_res0_inverse")]
// 0xf46dc — _FMOD_res0_inverse
pub fn stub_0xf46dc() -> ! { todo!("0xf46dc _FMOD_res0_inverse") }

#[doc(alias = "_FMOD_res0_look")]
// 0xf4768 — _FMOD_res0_look
pub fn stub_0xf4768() -> ! { todo!("0xf4768 _FMOD_res0_look") }

#[doc(alias = "_FMOD_res0_free_look")]
// 0xf4a4c — _FMOD_res0_free_look
pub fn stub_0xf4a4c() -> ! { todo!("0xf4a4c _FMOD_res0_free_look") }

#[doc(alias = "_FMOD_res0_free_info")]
// 0xf4b30 — _FMOD_res0_free_info
pub fn stub_0xf4b30() -> ! { todo!("0xf4b30 _FMOD_res0_free_info") }

#[doc(alias = "_FMOD_res0_unpack")]
// 0xf4b64 — _FMOD_res0_unpack
pub fn stub_0xf4b64() -> ! { todo!("0xf4b64 _FMOD_res0_unpack") }

#[doc(alias = "__FMOD_ilog")]
// 0xf5a68 — __FMOD_ilog
// type: int __fastcall(unsigned int)
pub fn stub_0xf5a68() -> ! { todo!("0xf5a68 __FMOD_ilog") }

#[doc(alias = "_FMOD_vorbis_book_clear")]
// 0xf5b20 — _FMOD_vorbis_book_clear
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0xf5b20() -> ! { todo!("0xf5b20 _FMOD_vorbis_book_clear") }

#[doc(alias = "__FMOD_book_maptype1_quantvals")]
// 0xf5bcc — __FMOD_book_maptype1_quantvals
// type: int __fastcall(_DWORD)
pub fn stub_0xf5bcc() -> ! { todo!("0xf5bcc __FMOD_book_maptype1_quantvals") }

#[doc(alias = "__FMOD_make_words")]
// 0xf5c70 — __FMOD_make_words
pub fn stub_0xf5c70() -> ! { todo!("0xf5c70 __FMOD_make_words") }

#[doc(alias = "__FMOD_float32_unpack")]
// 0xf5ec0 — __FMOD_float32_unpack
pub fn stub_0xf5ec0() -> ! { todo!("0xf5ec0 __FMOD_float32_unpack") }

#[doc(alias = "__FMOD_book_unquantize")]
// 0xf5f08 — __FMOD_book_unquantize
pub fn stub_0xf5f08() -> ! { todo!("0xf5f08 __FMOD_book_unquantize") }

#[doc(alias = "_FMOD_vorbis_staticbook_clear")]
// 0xf61b4 — _FMOD_vorbis_staticbook_clear
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf61b4() -> ! { todo!("0xf61b4 _FMOD_vorbis_staticbook_clear") }

#[doc(alias = "_FMOD_vorbis_staticbook_destroy")]
// 0xf622c — _FMOD_vorbis_staticbook_destroy
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf622c() -> ! { todo!("0xf622c _FMOD_vorbis_staticbook_destroy") }

#[doc(alias = "_FMOD_vorbis_book_init_decode")]
// 0xf625c — _FMOD_vorbis_book_init_decode
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf625c() -> ! { todo!("0xf625c _FMOD_vorbis_book_init_decode") }

#[doc(alias = "_FMOD_vorbis_packet_blocksize")]
// 0xfaef4 — _FMOD_vorbis_packet_blocksize
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xfaef4() -> ! { todo!("0xfaef4 _FMOD_vorbis_packet_blocksize") }

#[doc(alias = "_FMOD_vorbis_synthesis_trackonly")]
// 0xfaf80 — _FMOD_vorbis_synthesis_trackonly
pub fn stub_0xfaf80() -> ! { todo!("0xfaf80 _FMOD_vorbis_synthesis_trackonly") }

#[doc(alias = "_FMOD_vorbis_synthesis")]
// 0xfb088 — _FMOD_vorbis_synthesis
pub fn stub_0xfb088() -> ! { todo!("0xfb088 _FMOD_vorbis_synthesis") }

#[doc(alias = "_FMOD_vorbis_ftoi")]
// 0xfbe94 — _FMOD_vorbis_ftoi
pub fn stub_0xfbe94() -> ! { todo!("0xfbe94 _FMOD_vorbis_ftoi") }

#[doc(alias = "__FMOD_vorbis_window_get")]
// 0xfe764 — __FMOD_vorbis_window_get
// type: int __fastcall(_DWORD)
pub fn stub_0xfe764() -> ! { todo!("0xfe764 __FMOD_vorbis_window_get") }

#[doc(alias = "__FMOD_vorbis_window_init")]
// 0xfe778 — __FMOD_vorbis_window_init
// type: int(void)
pub fn stub_0xfe778() -> ! { todo!("0xfe778 __FMOD_vorbis_window_init") }

#[doc(alias = "FMOD::DSPConnectionI::init(float *&,int,int)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI4initERPfii")]
// 0xfe900 — __ZN4FMOD14DSPConnectionI4initERPfii
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this, float **, int, int)
pub fn stub_0xfe900() -> ! { todo!("0xfe900 __ZN4FMOD14DSPConnectionI4initERPfii") }

#[doc(alias = "FMOD::DSPConnectionI::reset(void)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI5resetEv")]
// 0xfe9d4 — __ZN4FMOD14DSPConnectionI5resetEv
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this)
pub fn stub_0xfe9d4() -> ! { todo!("0xfe9d4 __ZN4FMOD14DSPConnectionI5resetEv") }

#[doc(alias = "FMOD::DSPConnectionI::setUnity(void)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI8setUnityEv")]
// 0xfea70 — __ZN4FMOD14DSPConnectionI8setUnityEv
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this)
pub fn stub_0xfea70() -> ! { todo!("0xfea70 __ZN4FMOD14DSPConnectionI8setUnityEv") }

#[doc(alias = "FMOD::DSPConnectionI::mixAndRamp(float *,float *,int,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI10mixAndRampEPfS1_iij")]
// 0xfeb34 — __ZN4FMOD14DSPConnectionI10mixAndRampEPfS1_iij
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this, float *, float *, int, int, unsigned int)
pub fn stub_0xfeb34() -> ! { todo!("0xfeb34 __ZN4FMOD14DSPConnectionI10mixAndRampEPfS1_iij") }

#[doc(alias = "FMOD::DSPConnectionI::rampTo(void)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI6rampToEv")]
// 0xfecb0 — __ZN4FMOD14DSPConnectionI6rampToEv
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this)
pub fn stub_0xfecb0() -> ! { todo!("0xfecb0 __ZN4FMOD14DSPConnectionI6rampToEv") }

#[doc(alias = "FMOD::DSPConnectionI::checkUnity(int,int)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI10checkUnityEii")]
// 0xff094 — __ZN4FMOD14DSPConnectionI10checkUnityEii
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this, int, int)
pub fn stub_0xff094() -> ! { todo!("0xff094 __ZN4FMOD14DSPConnectionI10checkUnityEii") }

#[doc(alias = "FMOD::DSPConnectionI::setLevels(float *,int)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI9setLevelsEPfi")]
// 0xff144 — __ZN4FMOD14DSPConnectionI9setLevelsEPfi
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this, float *, FMOD::DSPConnectionI *)
pub fn stub_0xff144() -> ! { todo!("0xff144 __ZN4FMOD14DSPConnectionI9setLevelsEPfi") }

#[doc(alias = "FMOD::DSPConnectionI::setMix(float)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI6setMixEf")]
// 0xff210 — __ZN4FMOD14DSPConnectionI6setMixEf
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this, float)
pub fn stub_0xff210() -> ! { todo!("0xff210 __ZN4FMOD14DSPConnectionI6setMixEf") }

#[doc(alias = "FMOD::DSPConnectionI::setLevels(FMOD_SPEAKER,float *,int)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI9setLevelsE12FMOD_SPEAKERPfi")]
// 0xff258 — __ZN4FMOD14DSPConnectionI9setLevelsE12FMOD_SPEAKERPfi
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xff258() -> ! { todo!("0xff258 __ZN4FMOD14DSPConnectionI9setLevelsE12FMOD_SPEAKERPfi") }

#[doc(alias = "FMOD::DSPConnectionI::copy(FMOD::DSPConnectionI*)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI4copyEPS0_")]
// 0xff318 — __ZN4FMOD14DSPConnectionI4copyEPS0_
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this, FMOD::DSPConnectionI *)
pub fn stub_0xff318() -> ! { todo!("0xff318 __ZN4FMOD14DSPConnectionI4copyEPS0_") }

#[doc(alias = "FMOD::DSPConnectionI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0xff3e0 — __ZN4FMOD14DSPConnectionI17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this, FMOD::MemoryTracker *)
pub fn stub_0xff3e0() -> ! { todo!("0xff3e0 __ZN4FMOD14DSPConnectionI17getMemoryUsedImplEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::DSPConnectionI::setPan(float,int,int,FMOD_SPEAKERMODE)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI6setPanEfii16FMOD_SPEAKERMODE")]
// 0xff404 — __ZN4FMOD14DSPConnectionI6setPanEfii16FMOD_SPEAKERMODE
// type: int __fastcall(FMOD::DSPConnectionI *this, int, int, int, int)
pub fn stub_0xff404() -> ! { todo!("0xff404 __ZN4FMOD14DSPConnectionI6setPanEfii16FMOD_SPEAKERMODE") }

#[doc(alias = "FMOD::DSPConnectionI::mix(float *,float *,int,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI3mixEPfS1_iij")]
// 0xff990 — __ZN4FMOD14DSPConnectionI3mixEPfS1_iij
// type: _DWORD __fastcall(FMOD::DSPConnectionI *__hidden this, float *, float *, int, int, unsigned int)
pub fn stub_0xff990() -> ! { todo!("0xff990 __ZN4FMOD14DSPConnectionI3mixEPfS1_iij") }

#[doc(alias = "FMOD::DSPConnectionI::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD14DSPConnectionI13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0xffe58 — __ZN4FMOD14DSPConnectionI13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_0xffe58() -> ! { todo!("0xffe58 __ZN4FMOD14DSPConnectionI13getMemoryUsedEPNS_13MemoryTrackerE") }

#[doc(alias = "FMOD::init_mparams(void)")]
#[doc(alias = "__ZN4FMODL12init_mparamsEv")]
// 0xffeb0 — __ZN4FMODL12init_mparamsEv
// type: _DWORD __fastcall(FMOD *__hidden this)
pub fn stub_0xffeb0() -> ! { todo!("0xffeb0 __ZN4FMODL12init_mparamsEv") }

#[doc(alias = "FMOD::sys_trim(FMOD::malloc_state *,unsigned long)")]
#[doc(alias = "__ZN4FMODL8sys_trimEPNS_12malloc_stateEm")]
// 0xfff4c — __ZN4FMODL8sys_trimEPNS_12malloc_stateEm
pub fn stub_0xfff4c() -> ! { todo!("0xfff4c __ZN4FMODL8sys_trimEPNS_12malloc_stateEm") }

#[doc(alias = "FMOD::mspace_free(void *,void *)")]
#[doc(alias = "__ZN4FMOD11mspace_freeEPvS0_")]
// 0xfffac — __ZN4FMOD11mspace_freeEPvS0_
// type: _DWORD __fastcall(FMOD *__hidden this, void *, void *)
pub fn stub_0xfffac() -> ! { todo!("0xfffac __ZN4FMOD11mspace_freeEPvS0_") }

#[doc(alias = "FMOD::mspace_malloc(void *,unsigned long)")]
#[doc(alias = "__ZN4FMOD13mspace_mallocEPvm")]
// 0x100684 — __ZN4FMOD13mspace_mallocEPvm
// type: _DWORD __fastcall(FMOD *__hidden this, void *, unsigned int)
pub fn stub_0x100684() -> ! { todo!("0x100684 __ZN4FMOD13mspace_mallocEPvm") }

#[doc(alias = "FMOD::mspace_realloc(void *,void *,unsigned long)")]
#[doc(alias = "__ZN4FMOD14mspace_reallocEPvS0_m")]
// 0x101178 — __ZN4FMOD14mspace_reallocEPvS0_m
// type: _DWORD __fastcall(FMOD *__hidden this, void *, void *, unsigned int)
pub fn stub_0x101178() -> ! { todo!("0x101178 __ZN4FMOD14mspace_reallocEPvS0_m") }

#[doc(alias = "FMOD::MemoryTracker::add(bool,int,unsigned int)")]
#[doc(alias = "__ZN4FMOD13MemoryTracker3addEbij")]
// 0x1012fc — __ZN4FMOD13MemoryTracker3addEbij
// type: _DWORD __fastcall(FMOD::MemoryTracker *__hidden this, bool, int, unsigned int)
pub fn stub_0x1012fc() -> ! { todo!("0x1012fc __ZN4FMOD13MemoryTracker3addEbij") }
