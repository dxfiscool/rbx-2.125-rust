//! core wd_10k — 120 core stubs EA-sorted asc gap filler not yet in crates/core/src (global EA asc, next uncovered after 0x62778).
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 120 not yet in crates/core/src.
//! Range: 0x62778..0x716e8 | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "+[RobloxMemoryManager sharedInstance]")]
// 0x62778 — +[RobloxMemoryManager sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x62778() -> ! {
    todo!("0x62778 +[RobloxMemoryManager sharedInstance]")
}

#[doc(alias = "___37+[RobloxMemoryManager sharedInstance]_block_invoke")]
// 0x627d4 — ___37+[RobloxMemoryManager sharedInstance]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x627d4() -> ! {
    todo!("0x627d4 ___37+[RobloxMemoryManager sharedInstance]_block_invoke")
}

#[doc(alias = "+[RobloxCachedFlags sharedInstance]")]
// 0x63d30 — +[RobloxCachedFlags sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x63d30() -> ! {
    todo!("0x63d30 +[RobloxCachedFlags sharedInstance]")
}

#[doc(alias = "___35+[RobloxCachedFlags sharedInstance]_block_invoke")]
// 0x63d94 — ___35+[RobloxCachedFlags sharedInstance]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x63d94() -> ! {
    todo!("0x63d94 ___35+[RobloxCachedFlags sharedInstance]_block_invoke")
}

#[doc(alias = "+[CrashReporter sharedInstance]")]
// 0x640e4 — +[CrashReporter sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x640e4() -> ! {
    todo!("0x640e4 +[CrashReporter sharedInstance]")
}

#[doc(alias = "___31+[CrashReporter sharedInstance]_block_invoke")]
// 0x64140 — ___31+[CrashReporter sharedInstance]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x64140() -> ! {
    todo!("0x64140 ___31+[CrashReporter sharedInstance]_block_invoke")
}

#[doc(alias = "+[AppController sharedInstance]")]
// 0x66794 — +[AppController sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x66794() -> ! {
    todo!("0x66794 +[AppController sharedInstance]")
}

#[doc(alias = "___31+[AppController sharedInstance]_block_invoke")]
// 0x667f0 — ___31+[AppController sharedInstance]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x667f0() -> ! {
    todo!("0x667f0 ___31+[AppController sharedInstance]_block_invoke")
}

#[doc(alias = "+[SessionReporter sharedInstance]")]
// 0x674f0 — +[SessionReporter sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x674f0() -> ! {
    todo!("0x674f0 +[SessionReporter sharedInstance]")
}

#[doc(alias = "___33+[SessionReporter sharedInstance]_block_invoke")]
// 0x6754c — ___33+[SessionReporter sharedInstance]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x6754c() -> ! {
    todo!("0x6754c ___33+[SessionReporter sharedInstance]_block_invoke")
}

#[doc(alias = "FMOD::ProfileCpu::init(void)")]
// 0x686a4 — __ZN4FMOD10ProfileCpu4initEv
// type: int __fastcall(FMOD::ProfileCpu *this)
pub fn stub_0x686a4() -> ! {
    todo!("0x686a4 __ZN4FMOD10ProfileCpu4initEv")
}

#[doc(alias = "FMOD::ProfileCpu::update(FMOD::SystemI *,unsigned int)")]
// 0x686ac — __ZN4FMOD10ProfileCpu6updateEPNS_7SystemIEj
// type: int __fastcall(FMOD::ProfileCpu *this, FMOD::SystemI *, unsigned int)
pub fn stub_0x686ac() -> ! {
    todo!("0x686ac __ZN4FMOD10ProfileCpu6updateEPNS_7SystemIEj")
}

#[doc(alias = "FMOD::ProfileCpu::release(void)")]
// 0x68758 — __ZN4FMOD10ProfileCpu7releaseEv
// type: int __fastcall(FMOD::ProfileCpu *this)
pub fn stub_0x68758() -> ! {
    todo!("0x68758 __ZN4FMOD10ProfileCpu7releaseEv")
}

#[doc(alias = "FMOD::ProfileCpu::ProfileCpu(void)")]
// 0x68794 — __ZN4FMOD10ProfileCpuC2Ev
// type: int __fastcall(FMOD::ProfileCpu *this)
pub fn stub_0x68794() -> ! {
    todo!("0x68794 __ZN4FMOD10ProfileCpuC2Ev")
}

#[doc(alias = "FMOD::ProfileCpu::ProfileCpu(void)")]
// 0x687bc — __ZN4FMOD10ProfileCpuC1Ev
// type: int __fastcall(FMOD::ProfileCpu *this)
pub fn stub_0x687bc() -> ! {
    todo!("0x687bc __ZN4FMOD10ProfileCpuC1Ev")
}

#[doc(alias = "FMOD::FMOD_ProfileCpu_Create(void)")]
// 0x687c0 — __ZN4FMOD22FMOD_ProfileCpu_CreateEv
// type: int __fastcall(FMOD *this)
pub fn stub_0x687c0() -> ! {
    todo!("0x687c0 __ZN4FMOD22FMOD_ProfileCpu_CreateEv")
}

#[doc(alias = "FMOD::ProfileDsp::isNodeDuplicate(unsigned long long)")]
// 0x68864 — __ZN4FMOD10ProfileDsp15isNodeDuplicateEy
// type: int __fastcall(FMOD::ProfileDsp *this, unsigned __int64)
pub fn stub_0x68864() -> ! {
    todo!("0x68864 __ZN4FMOD10ProfileDsp15isNodeDuplicateEy")
}

#[doc(alias = "FMOD::ProfileDsp::sendPacket(FMOD::SystemI *)")]
// 0x68944 — __ZN4FMOD10ProfileDsp10sendPacketEPNS_7SystemIE
// type: int __fastcall(FMOD::ProfileDsp *this, FMOD::SystemI *)
pub fn stub_0x68944() -> ! {
    todo!("0x68944 __ZN4FMOD10ProfileDsp10sendPacketEPNS_7SystemIE")
}

#[doc(alias = "FMOD::ProfileDsp::growNodeStackSpace(void)")]
// 0x68a6c — __ZN4FMOD10ProfileDsp18growNodeStackSpaceEv
// type: int __fastcall(FMOD::ProfileDsp *this)
pub fn stub_0x68a6c() -> ! {
    todo!("0x68a6c __ZN4FMOD10ProfileDsp18growNodeStackSpaceEv")
}

#[doc(alias = "FMOD::ProfileDsp::growPacketSpace(void)")]
// 0x68adc — __ZN4FMOD10ProfileDsp15growPacketSpaceEv
// type: int __fastcall(FMOD::ProfileDsp *this)
pub fn stub_0x68adc() -> ! {
    todo!("0x68adc __ZN4FMOD10ProfileDsp15growPacketSpaceEv")
}

#[doc(alias = "FMOD::ProfileDsp::update(FMOD::SystemI *,unsigned int)")]
// 0x68b68 — __ZN4FMOD10ProfileDsp6updateEPNS_7SystemIEj
// type: int __fastcall(FMOD::ProfileDsp *this, FMOD::SystemI *, unsigned int)
pub fn stub_0x68b68() -> ! {
    todo!("0x68b68 __ZN4FMOD10ProfileDsp6updateEPNS_7SystemIEj")
}

#[doc(alias = "FMOD::ProfileDsp::release(void)")]
// 0x68dfc — __ZN4FMOD10ProfileDsp7releaseEv
// type: int __fastcall(FMOD::ProfileDsp *this)
pub fn stub_0x68dfc() -> ! {
    todo!("0x68dfc __ZN4FMOD10ProfileDsp7releaseEv")
}

#[doc(alias = "FMOD::ProfileDsp::init(void)")]
// 0x68ebc — __ZN4FMOD10ProfileDsp4initEv
// type: int __fastcall(FMOD::ProfileDsp *this)
pub fn stub_0x68ebc() -> ! {
    todo!("0x68ebc __ZN4FMOD10ProfileDsp4initEv")
}

#[doc(alias = "FMOD::ProfileDsp::ProfileDsp(void)")]
// 0x69028 — __ZN4FMOD10ProfileDspC2Ev
// type: int __fastcall(FMOD::ProfileDsp *this)
pub fn stub_0x69028() -> ! {
    todo!("0x69028 __ZN4FMOD10ProfileDspC2Ev")
}

#[doc(alias = "FMOD::ProfileDsp::ProfileDsp(void)")]
// 0x69078 — __ZN4FMOD10ProfileDspC1Ev
// type: int __fastcall(FMOD::ProfileDsp *this)
pub fn stub_0x69078() -> ! {
    todo!("0x69078 __ZN4FMOD10ProfileDspC1Ev")
}

#[doc(alias = "FMOD::FMOD_ProfileDsp_Create(void)")]
// 0x6907c — __ZN4FMOD22FMOD_ProfileDsp_CreateEv
// type: int __fastcall(FMOD *this)
pub fn stub_0x6907c() -> ! {
    todo!("0x6907c __ZN4FMOD22FMOD_ProfileDsp_CreateEv")
}

#[doc(alias = "FMOD::Profile::Profile(void)")]
// 0x6914c — __ZN4FMOD7ProfileC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
pub fn stub_0x6914c() -> ! {
    todo!("0x6914c __ZN4FMOD7ProfileC2Ev")
}

#[doc(alias = "FMOD::Profile::Profile(void)")]
// 0x6919c — __ZN4FMOD7ProfileC1Ev
// type: _DWORD *__fastcall(_DWORD *this)
pub fn stub_0x6919c() -> ! {
    todo!("0x6919c __ZN4FMOD7ProfileC1Ev")
}

#[doc(alias = "FMOD::Profile::registerModule(FMOD::ProfileModule *)")]
// 0x691a0 — __ZN4FMOD7Profile14registerModuleEPNS_13ProfileModuleE
// type: int __fastcall(int, int)
pub fn stub_0x691a0() -> ! {
    todo!("0x691a0 __ZN4FMOD7Profile14registerModuleEPNS_13ProfileModuleE")
}

#[doc(alias = "FMOD::ProfileModule::ProfileModule(void)")]
// 0x691c8 — __ZN4FMOD13ProfileModuleC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
pub fn stub_0x691c8() -> ! {
    todo!("0x691c8 __ZN4FMOD13ProfileModuleC2Ev")
}

#[doc(alias = "FMOD::ProfileModule::init(void)")]
// 0x691fc — __ZN4FMOD13ProfileModule4initEv
// type: int __fastcall(FMOD::ProfileModule *this)
pub fn stub_0x691fc() -> ! {
    todo!("0x691fc __ZN4FMOD13ProfileModule4initEv")
}

#[doc(alias = "FMOD::ProfileModule::release(void)")]
// 0x69204 — __ZN4FMOD13ProfileModule7releaseEv
// type: int __fastcall(FMOD::ProfileModule *this)
pub fn stub_0x69204() -> ! {
    todo!("0x69204 __ZN4FMOD13ProfileModule7releaseEv")
}

#[doc(alias = "FMOD::ProfileModule::update(FMOD::SystemI *,unsigned int)")]
// 0x6920c — __ZN4FMOD13ProfileModule6updateEPNS_7SystemIEj
// type: int()
pub fn stub_0x6920c() -> ! {
    todo!("0x6920c __ZN4FMOD13ProfileModule6updateEPNS_7SystemIEj")
}

#[doc(alias = "FMOD::ProfileClient::ProfileClient(void)")]
// 0x69214 — __ZN4FMOD13ProfileClientC2Ev
// type: char *__fastcall(FMOD::ProfileClient *this)
pub fn stub_0x69214() -> ! {
    todo!("0x69214 __ZN4FMOD13ProfileClientC2Ev")
}

#[doc(alias = "FMOD::ProfileClient::ProfileClient(void)")]
// 0x69280 — __ZN4FMOD13ProfileClientC1Ev
// type: char *__fastcall(FMOD::ProfileClient *this)
pub fn stub_0x69280() -> ! {
    todo!("0x69280 __ZN4FMOD13ProfileClientC1Ev")
}

#[doc(alias = "FMOD::ProfileClient::requestDataType(unsigned char,unsigned char,unsigned int)")]
// 0x69284 — __ZN4FMOD13ProfileClient15requestDataTypeEhhj
// type: int __fastcall(FMOD::ProfileClient *this, int, int, unsigned int)
pub fn stub_0x69284() -> ! {
    todo!("0x69284 __ZN4FMOD13ProfileClient15requestDataTypeEhhj")
}

#[doc(alias = "FMOD::ProfileClient::wantsData(FMOD::ProfilePacketHeader *)")]
// 0x69358 — __ZN4FMOD13ProfileClient9wantsDataEPNS_19ProfilePacketHeaderE
// type: bool __fastcall(int, unsigned __int8 *)
pub fn stub_0x69358() -> ! {
    todo!("0x69358 __ZN4FMOD13ProfileClient9wantsDataEPNS_19ProfilePacketHeaderE")
}

#[doc(alias = "FMOD::ProfileClient::sendData(void)")]
// 0x693f4 — __ZN4FMOD13ProfileClient8sendDataEv
// type: int __fastcall(FMOD::ProfileClient *this)
pub fn stub_0x693f4() -> ! {
    todo!("0x693f4 __ZN4FMOD13ProfileClient8sendDataEv")
}

#[doc(alias = "FMOD::ProfileClient::readData(void)")]
// 0x69480 — __ZN4FMOD13ProfileClient8readDataEv
// type: int __fastcall(const void **this)
pub fn stub_0x69480() -> ! {
    todo!("0x69480 __ZN4FMOD13ProfileClient8readDataEv")
}

#[doc(alias = "FMOD::ProfileClient::update(unsigned int)")]
// 0x695dc — __ZN4FMOD13ProfileClient6updateEj
// type: int __fastcall(FMOD::ProfileClient *this, unsigned int)
pub fn stub_0x695dc() -> ! {
    todo!("0x695dc __ZN4FMOD13ProfileClient6updateEj")
}

#[doc(alias = "FMOD::ProfileClient::addPacket(FMOD::ProfilePacketHeader *)")]
// 0x69634 — __ZN4FMOD13ProfileClient9addPacketEPNS_19ProfilePacketHeaderE
// type: int __fastcall(FMOD::ProfileClient *this, unsigned __int8 *__src)
pub fn stub_0x69634() -> ! {
    todo!("0x69634 __ZN4FMOD13ProfileClient9addPacketEPNS_19ProfilePacketHeaderE")
}

#[doc(alias = "FMOD::ProfileClient::release(void)")]
// 0x69820 — __ZN4FMOD13ProfileClient7releaseEv
// type: int __fastcall(const void **this)
pub fn stub_0x69820() -> ! {
    todo!("0x69820 __ZN4FMOD13ProfileClient7releaseEv")
}

#[doc(alias = "FMOD::ProfileClient::init(void *)")]
// 0x6989c — __ZN4FMOD13ProfileClient4initEPv
// type: int __fastcall(FMOD::ProfileClient *this, void *)
pub fn stub_0x6989c() -> ! {
    todo!("0x6989c __ZN4FMOD13ProfileClient4initEPv")
}

#[doc(alias = "FMOD::Profile::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
// 0x69910 — __ZN4FMOD7Profile17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::Profile *this, FMOD::MemoryTracker *)
pub fn stub_0x69910() -> ! {
    todo!("0x69910 __ZN4FMOD7Profile17getMemoryUsedImplEPNS_13MemoryTrackerE")
}

#[doc(alias = "FMOD::Profile::release(void)")]
// 0x69a78 — __ZN4FMOD7Profile7releaseEv
// type: int __fastcall(FMOD::Profile *this)
pub fn stub_0x69a78() -> ! {
    todo!("0x69a78 __ZN4FMOD7Profile7releaseEv")
}

#[doc(alias = "FMOD::FMOD_Profile_Release(void)")]
// 0x69be8 — __ZN4FMOD20FMOD_Profile_ReleaseEv
// type: int __fastcall(FMOD *this)
pub fn stub_0x69be8() -> ! {
    todo!("0x69be8 __ZN4FMOD20FMOD_Profile_ReleaseEv")
}

#[doc(alias = "FMOD::Profile::init(unsigned short)")]
// 0x69c20 — __ZN4FMOD7Profile4initEt
// type: int __fastcall(FMOD::Profile *this, unsigned __int16)
pub fn stub_0x69c20() -> ! {
    todo!("0x69c20 __ZN4FMOD7Profile4initEt")
}

#[doc(alias = "FMOD::FMOD_Profile_Create(unsigned short)")]
// 0x69c9c — __ZN4FMOD19FMOD_Profile_CreateEt
// type: int __fastcall(FMOD *this, unsigned __int16)
pub fn stub_0x69c9c() -> ! {
    todo!("0x69c9c __ZN4FMOD19FMOD_Profile_CreateEt")
}

#[doc(alias = "FMOD::Profile::addPacket(FMOD::ProfilePacketHeader *)")]
// 0x69d50 — __ZN4FMOD7Profile9addPacketEPNS_19ProfilePacketHeaderE
// type: int __fastcall(_DWORD *, int)
pub fn stub_0x69d50() -> ! {
    todo!("0x69d50 __ZN4FMOD7Profile9addPacketEPNS_19ProfilePacketHeaderE")
}

#[doc(alias = "FMOD::Profile::update(FMOD::SystemI *,unsigned int)")]
// 0x69e0c — __ZN4FMOD7Profile6updateEPNS_7SystemIEj
// type: int __fastcall(FMOD::Profile *this, FMOD::SystemI *, unsigned int)
pub fn stub_0x69e0c() -> ! {
    todo!("0x69e0c __ZN4FMOD7Profile6updateEPNS_7SystemIEj")
}

#[doc(alias = "FMOD::FMOD_Profile_Update(FMOD::SystemI *,unsigned int)")]
// 0x6a018 — __ZN4FMOD19FMOD_Profile_UpdateEPNS_7SystemIEj
// type: int __fastcall(FMOD *this, FMOD::SystemI *, unsigned int)
pub fn stub_0x6a018() -> ! {
    todo!("0x6a018 __ZN4FMOD19FMOD_Profile_UpdateEPNS_7SystemIEj")
}

#[doc(alias = "FMOD::Profile::getMemoryUsed(FMOD::MemoryTracker *)")]
// 0x6a04c — __ZN4FMOD7Profile13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
pub fn stub_0x6a04c() -> ! {
    todo!("0x6a04c __ZN4FMOD7Profile13getMemoryUsedEPNS_13MemoryTrackerE")
}

#[doc(alias = "_FMOD_oggpack_look")]
// 0x6d26c — _FMOD_oggpack_look
// type: int __fastcall(int *, int)
pub fn stub_0x6d26c() -> ! {
    todo!("0x6d26c _FMOD_oggpack_look")
}

#[doc(alias = "_FMOD_oggpack_adv")]
// 0x6d318 — _FMOD_oggpack_adv
// type: _DWORD *__fastcall(_DWORD *result, int)
pub fn stub_0x6d318() -> ! {
    todo!("0x6d318 _FMOD_oggpack_adv")
}

#[doc(alias = "_FMOD_oggpack_read")]
// 0x6d354 — _FMOD_oggpack_read
// type: int __fastcall(int *, int)
pub fn stub_0x6d354() -> ! {
    todo!("0x6d354 _FMOD_oggpack_read")
}

#[doc(alias = "_FMOD_oggpack_bytes")]
// 0x6d434 — _FMOD_oggpack_bytes
// type: int __fastcall(int *)
pub fn stub_0x6d434() -> ! {
    todo!("0x6d434 _FMOD_oggpack_bytes")
}

#[doc(alias = "_FMOD_oggpack_readinit")]
// 0x6d44c — _FMOD_oggpack_readinit
// type: _DWORD *__fastcall(_DWORD *result, int, int)
pub fn stub_0x6d44c() -> ! {
    todo!("0x6d44c _FMOD_oggpack_readinit")
}

#[doc(alias = "_FMOD_vorbis_synthesis_restart")]
// 0x6d4b4 — _FMOD_vorbis_synthesis_restart
// type: int __fastcall(int **)
pub fn stub_0x6d4b4() -> ! {
    todo!("0x6d4b4 _FMOD_vorbis_synthesis_restart")
}

#[doc(alias = "_FMOD_vorbis_synthesis_pcmout")]
// 0x6d538 — _FMOD_vorbis_synthesis_pcmout
// type: int __fastcall(int *, _DWORD *)
pub fn stub_0x6d538() -> ! {
    todo!("0x6d538 _FMOD_vorbis_synthesis_pcmout")
}

#[doc(alias = "_FMOD_vorbis_synthesis_read")]
// 0x6d5c8 — _FMOD_vorbis_synthesis_read
// type: int __fastcall(int, int)
pub fn stub_0x6d5c8() -> ! {
    todo!("0x6d5c8 _FMOD_vorbis_synthesis_read")
}

#[doc(alias = "_FMOD_vorbis_synthesis_blockin")]
// 0x6d600 — _FMOD_vorbis_synthesis_blockin
// type: int __fastcall(int *, int)
pub fn stub_0x6d600() -> ! {
    todo!("0x6d600 _FMOD_vorbis_synthesis_blockin")
}

#[doc(alias = "__FMOD_vorbis_block_alloc")]
// 0x6dee8 — __FMOD_vorbis_block_alloc
// type: int __fastcall(int, _DWORD *, int)
pub fn stub_0x6dee8() -> ! {
    todo!("0x6dee8 __FMOD_vorbis_block_alloc")
}

#[doc(alias = "__FMOD_vorbis_block_ripcord")]
// 0x6df94 — __FMOD_vorbis_block_ripcord
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x6df94() -> ! {
    todo!("0x6df94 __FMOD_vorbis_block_ripcord")
}

#[doc(alias = "_FMOD_vorbis_block_init")]
// 0x6e044 — _FMOD_vorbis_block_init
// type: int __fastcall(int, int, void *__b)
pub fn stub_0x6e044() -> ! {
    todo!("0x6e044 _FMOD_vorbis_block_init")
}

#[doc(alias = "_FMOD_vorbis_dsp_clear")]
// 0x6e078 — _FMOD_vorbis_dsp_clear
// type: void *__fastcall(void *result, int *, int, int)
pub fn stub_0x6e078() -> ! {
    todo!("0x6e078 _FMOD_vorbis_dsp_clear")
}

#[doc(alias = "_FMOD_vorbis_synthesis_init")]
// 0x6e2c4 — _FMOD_vorbis_synthesis_init
// type: int __fastcall(void *, int *__b, int, int)
pub fn stub_0x6e2c4() -> ! {
    todo!("0x6e2c4 _FMOD_vorbis_synthesis_init")
}

#[doc(alias = "_FMOD_vorbis_block_clear")]
// 0x6e6c0 — _FMOD_vorbis_block_clear
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x6e6c0() -> ! {
    todo!("0x6e6c0 _FMOD_vorbis_block_clear")
}

#[doc(alias = "_FMOD_vorbis_book_decode")]
// 0x6e778 — _FMOD_vorbis_book_decode
// type: int __fastcall(int *, int *)
pub fn stub_0x6e778() -> ! {
    todo!("0x6e778 _FMOD_vorbis_book_decode")
}

#[doc(alias = "_FMOD_vorbis_staticbook_unpack")]
// 0x6e8c4 — _FMOD_vorbis_staticbook_unpack
// type: int __fastcall(int, int *, int *)
pub fn stub_0x6e8c4() -> ! {
    todo!("0x6e8c4 _FMOD_vorbis_staticbook_unpack")
}

#[doc(alias = "_FMOD_vorbis_book_decodevv_add")]
// 0x6ec78 — _FMOD_vorbis_book_decodevv_add
// type: int __fastcall(int *, int, int, int, int *, int)
pub fn stub_0x6ec78() -> ! {
    todo!("0x6ec78 _FMOD_vorbis_book_decodevv_add")
}

#[doc(alias = "_FMOD_vorbis_book_decodev_add")]
// 0x6ee98 — _FMOD_vorbis_book_decodev_add
// type: int __fastcall(int *, int, int *, int)
pub fn stub_0x6ee98() -> ! {
    todo!("0x6ee98 _FMOD_vorbis_book_decodev_add")
}

#[doc(alias = "_FMOD_vorbis_book_decodevs_add")]
// 0x6f37c — _FMOD_vorbis_book_decodevs_add
// type: int __fastcall(int *, __int32 *, int *, int)
pub fn stub_0x6f37c() -> ! {
    todo!("0x6f37c _FMOD_vorbis_book_decodevs_add")
}

#[doc(alias = "_FMOD_floor1_inverse1")]
// 0x6f840 — _FMOD_floor1_inverse1
// type: int *__fastcall(int, int, _DWORD *)
pub fn stub_0x6f840() -> ! {
    todo!("0x6f840 _FMOD_floor1_inverse1")
}

#[doc(alias = "_FMOD_floor1_free_look")]
// 0x6fbac — _FMOD_floor1_free_look
// type: int __fastcall(int result, void *)
pub fn stub_0x6fbac() -> ! {
    todo!("0x6fbac _FMOD_floor1_free_look")
}

#[doc(alias = "_FMOD_floor1_look")]
// 0x6fbe0 — _FMOD_floor1_look
// type: _DWORD *__fastcall(int, int, int *)
pub fn stub_0x6fbe0() -> ! {
    todo!("0x6fbe0 _FMOD_floor1_look")
}

#[doc(alias = "_FMOD_floor1_free_info")]
// 0x6fe68 — _FMOD_floor1_free_info
// type: int __fastcall(int result, void *)
pub fn stub_0x6fe68() -> ! {
    todo!("0x6fe68 _FMOD_floor1_free_info")
}

#[doc(alias = "_FMOD_floor1_unpack")]
// 0x6fe9c — _FMOD_floor1_unpack
// type: int *__fastcall(int, int, int *)
pub fn stub_0x6fe9c() -> ! {
    todo!("0x6fe9c _FMOD_floor1_unpack")
}

#[doc(alias = "_FMOD_floor1_inverse2")]
// 0x701fc — _FMOD_floor1_inverse2
// type: int __fastcall(int, int, int, _DWORD *, char *__b)
pub fn stub_0x701fc() -> ! {
    todo!("0x701fc _FMOD_floor1_inverse2")
}

#[doc(alias = "_FMOD_Channel_GetUserData")]
// 0x70458 — _FMOD_Channel_GetUserData
// type: int __fastcall(FMOD::Channel *, void **)
pub fn stub_0x70458() -> ! {
    todo!("0x70458 _FMOD_Channel_GetUserData")
}

#[doc(alias = "_FMOD_System_Create")]
// 0x70474 — _FMOD_System_Create
// type: int __fastcall(FMOD::SystemI **)
pub fn stub_0x70474() -> ! {
    todo!("0x70474 _FMOD_System_Create")
}

#[doc(alias = "_FMOD_Memory_GetStats")]
// 0x705cc — _FMOD_Memory_GetStats
// type: int __fastcall(_DWORD *, _DWORD *, int)
pub fn stub_0x705cc() -> ! {
    todo!("0x705cc _FMOD_Memory_GetStats")
}

#[doc(alias = "FMOD::AsyncThread::release(void)")]
// 0x7069c — __ZN4FMOD11AsyncThread7releaseEv
// type: int __fastcall(FMOD::AsyncThread *this)
pub fn stub_0x7069c() -> ! {
    todo!("0x7069c __ZN4FMOD11AsyncThread7releaseEv")
}

#[doc(alias = "FMOD::AsyncThread::threadFunc(void)")]
// 0x706b4 — __ZN4FMOD11AsyncThread10threadFuncEv
// type: int __fastcall(FMOD::AsyncThread *this)
pub fn stub_0x706b4() -> ! {
    todo!("0x706b4 __ZN4FMOD11AsyncThread10threadFuncEv")
}

#[doc(alias = "FMOD::asyncThreadFunc(void *)")]
// 0x70ab0 — __ZN4FMOD15asyncThreadFuncEPv
// type: int __fastcall(FMOD::AsyncThread *this, void *)
pub fn stub_0x70ab0() -> ! {
    todo!("0x70ab0 __ZN4FMOD15asyncThreadFuncEPv")
}

#[doc(alias = "FMOD::AsyncThread::reallyRelease(void)")]
// 0x70ab4 — __ZN4FMOD11AsyncThread13reallyReleaseEv
// type: int __fastcall(FMOD::AsyncThread *this)
pub fn stub_0x70ab4() -> ! {
    todo!("0x70ab4 __ZN4FMOD11AsyncThread13reallyReleaseEv")
}

#[doc(alias = "FMOD::AsyncThread::init(bool,FMOD::SystemI *)")]
// 0x70bbc — __ZN4FMOD11AsyncThread4initEbPNS_7SystemIE
// type: int __fastcall(FMOD::AsyncThread *this, bool, FMOD::SystemI *)
pub fn stub_0x70bbc() -> ! {
    todo!("0x70bbc __ZN4FMOD11AsyncThread4initEbPNS_7SystemIE")
}

#[doc(alias = "FMOD::AsyncThread::AsyncThread(void)")]
// 0x70c98 — __ZN4FMOD11AsyncThreadC2Ev
// type: int __fastcall(FMOD::AsyncThread *this)
pub fn stub_0x70c98() -> ! {
    todo!("0x70c98 __ZN4FMOD11AsyncThreadC2Ev")
}

#[doc(alias = "FMOD::AsyncThread::AsyncThread(void)")]
// 0x70cec — __ZN4FMOD11AsyncThreadC1Ev
// type: int __fastcall(FMOD::AsyncThread *this)
pub fn stub_0x70cec() -> ! {
    todo!("0x70cec __ZN4FMOD11AsyncThreadC1Ev")
}

#[doc(alias = "FMOD::AsyncThread::getAsyncThread(FMOD::SoundI *)")]
// 0x70cf0 — __ZN4FMOD11AsyncThread14getAsyncThreadEPNS_6SoundIE
// type: int __fastcall(FMOD::AsyncThread *this, FMOD::SoundI *)
pub fn stub_0x70cf0() -> ! {
    todo!("0x70cf0 __ZN4FMOD11AsyncThread14getAsyncThreadEPNS_6SoundIE")
}

#[doc(alias = "FMOD::AsyncThread::shutDown(void)")]
// 0x70ddc — __ZN4FMOD11AsyncThread8shutDownEv
// type: int __fastcall(FMOD::AsyncThread *this)
pub fn stub_0x70ddc() -> ! {
    todo!("0x70ddc __ZN4FMOD11AsyncThread8shutDownEv")
}

#[doc(alias = "FMOD::AsyncThread::update(void)")]
// 0x70e5c — __ZN4FMOD11AsyncThread6updateEv
// type: int __fastcall(FMOD::AsyncThread *this)
pub fn stub_0x70e5c() -> ! {
    todo!("0x70e5c __ZN4FMOD11AsyncThread6updateEv")
}

#[doc(alias = "global constructor keyed toFMOD::AsyncThread::gAsyncHead")]
// 0x70f2c — __GLOBAL__I__ZN4FMOD11AsyncThread10gAsyncHeadE
// type: int()
pub fn stub_0x70f2c() -> ! {
    todo!("0x70f2c __GLOBAL__I__ZN4FMOD11AsyncThread10gAsyncHeadE")
}

#[doc(alias = "FMOD::Channel::getUserData(void **)")]
// 0x70f38 — __ZN4FMOD7Channel11getUserDataEPPv
// type: int __fastcall(FMOD::Channel *this, void **, FMOD::ChannelI **)
pub fn stub_0x70f38() -> ! {
    todo!("0x70f38 __ZN4FMOD7Channel11getUserDataEPPv")
}

#[doc(alias = "FMOD::Channel::setUserData(void *)")]
// 0x70f7c — __ZN4FMOD7Channel11setUserDataEPv
// type: int __fastcall(FMOD::Channel *this, void *, FMOD::ChannelI **)
pub fn stub_0x70f7c() -> ! {
    todo!("0x70f7c __ZN4FMOD7Channel11setUserDataEPv")
}

#[doc(alias = "FMOD::Channel::setLoopCount(int)")]
// 0x70fb0 — __ZN4FMOD7Channel12setLoopCountEi
// type: int __fastcall(FMOD::Channel *this, int, FMOD::ChannelI **)
pub fn stub_0x70fb0() -> ! {
    todo!("0x70fb0 __ZN4FMOD7Channel12setLoopCountEi")
}

#[doc(alias = "FMOD::Channel::getMode(unsigned int *)")]
// 0x70fe4 — __ZN4FMOD7Channel7getModeEPj
// type: int __fastcall(FMOD::Channel *this, unsigned int *, FMOD::ChannelI **)
pub fn stub_0x70fe4() -> ! {
    todo!("0x70fe4 __ZN4FMOD7Channel7getModeEPj")
}

#[doc(alias = "FMOD::Channel::setMode(unsigned int)")]
// 0x71028 — __ZN4FMOD7Channel7setModeEj
// type: int __fastcall(FMOD::Channel *this, unsigned int, FMOD::ChannelI **)
pub fn stub_0x71028() -> ! {
    todo!("0x71028 __ZN4FMOD7Channel7setModeEj")
}

#[doc(alias = "FMOD::Channel::isPlaying(bool *)")]
// 0x7105c — __ZN4FMOD7Channel9isPlayingEPb
// type: int __fastcall(FMOD::Channel *this, bool *, FMOD::ChannelI **)
pub fn stub_0x7105c() -> ! {
    todo!("0x7105c __ZN4FMOD7Channel9isPlayingEPb")
}

#[doc(alias = "FMOD::Channel::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")]
// 0x710a0 — __ZN4FMOD7Channel15set3DAttributesEPK11FMOD_VECTORS3_
// type: int __fastcall(FMOD::ChannelI *, int, FMOD::ChannelI **)
pub fn stub_0x710a0() -> ! {
    todo!("0x710a0 __ZN4FMOD7Channel15set3DAttributesEPK11FMOD_VECTORS3_")
}

#[doc(alias = "FMOD::Channel::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")]
// 0x710dc — __ZN4FMOD7Channel11setCallbackEPF11FMOD_RESULTP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS5_E
// type: int __fastcall(FMOD::ChannelI *, int, FMOD::ChannelI **)
pub fn stub_0x710dc() -> ! {
    todo!("0x710dc __ZN4FMOD7Channel11setCallbackEPF11FMOD_RESULTP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS5_E")
}

#[doc(alias = "FMOD::Channel::setChannelGroup(FMOD::ChannelGroup *)")]
// 0x71110 — __ZN4FMOD7Channel15setChannelGroupEPNS_12ChannelGroupE
// type: int __fastcall(FMOD::ChannelI *, FMOD::ChannelGroupI *, FMOD::ChannelI **)
pub fn stub_0x71110() -> ! {
    todo!("0x71110 __ZN4FMOD7Channel15setChannelGroupEPNS_12ChannelGroupE")
}

#[doc(alias = "FMOD::Channel::setPriority(int)")]
// 0x71144 — __ZN4FMOD7Channel11setPriorityEi
// type: int __fastcall(FMOD::Channel *this, int, FMOD::ChannelI **)
pub fn stub_0x71144() -> ! {
    todo!("0x71144 __ZN4FMOD7Channel11setPriorityEi")
}

#[doc(alias = "FMOD::Channel::setMute(bool)")]
// 0x71178 — __ZN4FMOD7Channel7setMuteEb
// type: int __fastcall(FMOD::Channel *this, bool, FMOD::ChannelI **)
pub fn stub_0x71178() -> ! {
    todo!("0x71178 __ZN4FMOD7Channel7setMuteEb")
}

#[doc(alias = "FMOD::Channel::getFrequency(float *)")]
// 0x711ac — __ZN4FMOD7Channel12getFrequencyEPf
// type: int __fastcall(FMOD::Channel *this, float *, FMOD::ChannelI **)
pub fn stub_0x711ac() -> ! {
    todo!("0x711ac __ZN4FMOD7Channel12getFrequencyEPf")
}

#[doc(alias = "FMOD::Channel::setFrequency(float)")]
// 0x711f0 — __ZN4FMOD7Channel12setFrequencyEf
// type: int __fastcall(FMOD::Channel *this, float, FMOD::ChannelI **)
pub fn stub_0x711f0() -> ! {
    todo!("0x711f0 __ZN4FMOD7Channel12setFrequencyEf")
}

#[doc(alias = "FMOD::Channel::setVolume(float)")]
// 0x71224 — __ZN4FMOD7Channel9setVolumeEf
// type: int __fastcall(FMOD::Channel *this, float, FMOD::ChannelI **)
pub fn stub_0x71224() -> ! {
    todo!("0x71224 __ZN4FMOD7Channel9setVolumeEf")
}

#[doc(alias = "FMOD::Channel::getPaused(bool *)")]
// 0x71260 — __ZN4FMOD7Channel9getPausedEPb
// type: int __fastcall(FMOD::Channel *this, bool *, FMOD::ChannelI **)
pub fn stub_0x71260() -> ! {
    todo!("0x71260 __ZN4FMOD7Channel9getPausedEPb")
}

#[doc(alias = "FMOD::Channel::setPaused(bool)")]
// 0x712a4 — __ZN4FMOD7Channel9setPausedEb
// type: int __fastcall(FMOD::Channel *this, bool, FMOD::ChannelI **)
pub fn stub_0x712a4() -> ! {
    todo!("0x712a4 __ZN4FMOD7Channel9setPausedEb")
}

#[doc(alias = "FMOD::Channel::stop(void)")]
// 0x712d8 — __ZN4FMOD7Channel4stopEv
// type: int __fastcall(FMOD::Channel *this, int, FMOD::ChannelI **)
pub fn stub_0x712d8() -> ! {
    todo!("0x712d8 __ZN4FMOD7Channel4stopEv")
}

#[doc(alias = "FMOD::ChannelEmulated::isVirtual(bool *)")]
// 0x71304 — __ZN4FMOD15ChannelEmulated9isVirtualEPb
// type: int __fastcall(FMOD::ChannelEmulated *this, bool *, int, bool)
pub fn stub_0x71304() -> ! {
    todo!("0x71304 __ZN4FMOD15ChannelEmulated9isVirtualEPb")
}

#[doc(alias = "FMOD::ChannelEmulated::getDSPHead(FMOD::DSPI **)")]
// 0x7131c — __ZN4FMOD15ChannelEmulated10getDSPHeadEPPNS_4DSPIE
// type: int __fastcall(int, int *)
pub fn stub_0x7131c() -> ! {
    todo!("0x7131c __ZN4FMOD15ChannelEmulated10getDSPHeadEPPNS_4DSPIE")
}

#[doc(alias = "FMOD::ChannelEmulated::setSpeakerLevels(int,float *,int)")]
// 0x71334 — __ZN4FMOD15ChannelEmulated16setSpeakerLevelsEiPfi
// type: int __fastcall(FMOD::ChannelEmulated *this, int, float *, int)
pub fn stub_0x71334() -> ! {
    todo!("0x71334 __ZN4FMOD15ChannelEmulated16setSpeakerLevelsEiPfi")
}

#[doc(alias = "FMOD::ChannelEmulated::setSpeakerMix(float,float,float,float,float,float,float,float)")]
// 0x7133c — __ZN4FMOD15ChannelEmulated13setSpeakerMixEffffffff
// type: int __fastcall(FMOD::ChannelEmulated *this, float, float, float, float, float, float, float, float)
pub fn stub_0x7133c() -> ! {
    todo!("0x7133c __ZN4FMOD15ChannelEmulated13setSpeakerMixEffffffff")
}

#[doc(alias = "FMOD::ChannelEmulated::update(int)")]
// 0x71344 — __ZN4FMOD15ChannelEmulated6updateEi
// type: int __fastcall(FMOD::ChannelEmulated *this, int)
pub fn stub_0x71344() -> ! {
    todo!("0x71344 __ZN4FMOD15ChannelEmulated6updateEi")
}

#[doc(alias = "FMOD::ChannelEmulated::close(void)")]
// 0x71540 — __ZN4FMOD15ChannelEmulated5closeEv
// type: int __fastcall(FMOD::ChannelEmulated *this)
pub fn stub_0x71540() -> ! {
    todo!("0x71540 __ZN4FMOD15ChannelEmulated5closeEv")
}

#[doc(alias = "FMOD::ChannelEmulated::alloc(void)")]
// 0x71580 — __ZN4FMOD15ChannelEmulated5allocEv
// type: int __fastcall(FMOD::DSPI **this)
pub fn stub_0x71580() -> ! {
    todo!("0x71580 __ZN4FMOD15ChannelEmulated5allocEv")
}

#[doc(alias = "FMOD::ChannelEmulated::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
// 0x715e8 — __ZN4FMOD15ChannelEmulated4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE
// type: int __fastcall(FMOD::ChannelEmulated *this, int, FMOD::SystemI *, FMOD::Output *, FMOD::DSPI *)
pub fn stub_0x715e8() -> ! {
    todo!("0x715e8 __ZN4FMOD15ChannelEmulated4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE")
}

#[doc(alias = "FMOD::ChannelEmulated::ChannelEmulated(void)")]
// 0x71698 — __ZN4FMOD15ChannelEmulatedC2Ev
// type: int __fastcall(FMOD::ChannelEmulated *this)
pub fn stub_0x71698() -> ! {
    todo!("0x71698 __ZN4FMOD15ChannelEmulatedC2Ev")
}

#[doc(alias = "FMOD::ChannelEmulated::ChannelEmulated(void)")]
// 0x716e4 — __ZN4FMOD15ChannelEmulatedC1Ev
// type: int __fastcall(FMOD::ChannelEmulated *this)
pub fn stub_0x716e4() -> ! {
    todo!("0x716e4 __ZN4FMOD15ChannelEmulatedC1Ev")
}

#[doc(alias = "FMOD::ChannelEmulated::stop(void)")]
// 0x716e8 — __ZN4FMOD15ChannelEmulated4stopEv
// type: int __fastcall(FMOD::ChannelEmulated *this)
pub fn stub_0x716e8() -> ! {
    todo!("0x716e8 __ZN4FMOD15ChannelEmulated4stopEv")
}

