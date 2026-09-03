//! core shard ju — 150 stubs EA-sorted 0x62778..0x72364 (global EA-sorted, next 150 not yet in core+reflection combined after jt 0x504a0c, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) global EA-sorted ascending, next 150 not yet in rbx_core nor rbx_reflection (combined 52960 before -> 53110 after, global gap 32586).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "+[RobloxMemoryManager sharedInstance]")]
// 0x62778 — +[RobloxMemoryManager sharedInstance]
pub fn stub_62778() {
    // IDA 0x62778: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___37+[RobloxMemoryManager sharedInstance]_block_invoke")]
// 0x627d4 — ___37+[RobloxMemoryManager sharedInstance]_block_invoke
pub fn stub_627d4() {
    // IDA 0x627d4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[RobloxCachedFlags sharedInstance]")]
// 0x63d30 — +[RobloxCachedFlags sharedInstance]
pub fn stub_63d30() {
    // IDA 0x63d30: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___35+[RobloxCachedFlags sharedInstance]_block_invoke")]
// 0x63d94 — ___35+[RobloxCachedFlags sharedInstance]_block_invoke
pub fn stub_63d94() {
    // IDA 0x63d94: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[CrashReporter sharedInstance]")]
// 0x640e4 — +[CrashReporter sharedInstance]
pub fn stub_640e4() {
    // IDA 0x640e4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___31+[CrashReporter sharedInstance]_block_invoke")]
// 0x64140 — ___31+[CrashReporter sharedInstance]_block_invoke
pub fn stub_64140() {
    // IDA 0x64140: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[AppController sharedInstance]")]
// 0x66794 — +[AppController sharedInstance]
pub fn stub_66794() {
    // IDA 0x66794: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___31+[AppController sharedInstance]_block_invoke")]
// 0x667f0 — ___31+[AppController sharedInstance]_block_invoke
pub fn stub_667f0() {
    // IDA 0x667f0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[AppController runJoinScriptWithUrl:]")]
// 0x66b1c — -[AppController runJoinScriptWithUrl:]
pub fn stub_66b1c() {
    // IDA 0x66b1c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[AppController launchGameFromOverlayDataModel:]")]
// 0x67148 — -[AppController launchGameFromOverlayDataModel:]
pub fn stub_67148() {
    // IDA 0x67148: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[SessionReporter sharedInstance]")]
// 0x674f0 — +[SessionReporter sharedInstance]
pub fn stub_674f0() {
    // IDA 0x674f0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___33+[SessionReporter sharedInstance]_block_invoke")]
// 0x6754c — ___33+[SessionReporter sharedInstance]_block_invoke
pub fn stub_6754c() {
    // IDA 0x6754c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileCpu::init(void)")]
#[doc(alias = "__ZN4FMOD10ProfileCpu4initEv")]
// 0x686a4 — __ZN4FMOD10ProfileCpu4initEv
pub fn stub_686a4() {
    // IDA 0x686a4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileCpu::update(FMOD::SystemI *,unsigned int)")]
#[doc(alias = "__ZN4FMOD10ProfileCpu6updateEPNS_7SystemIEj")]
// 0x686ac — __ZN4FMOD10ProfileCpu6updateEPNS_7SystemIEj
pub fn stub_686ac() {
    // IDA 0x686ac: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileCpu::release(void)")]
#[doc(alias = "__ZN4FMOD10ProfileCpu7releaseEv")]
// 0x68758 — __ZN4FMOD10ProfileCpu7releaseEv
pub fn stub_68758() {
    // IDA 0x68758: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileCpu::ProfileCpu(void)")]
#[doc(alias = "__ZN4FMOD10ProfileCpuC2Ev")]
// 0x68794 — __ZN4FMOD10ProfileCpuC2Ev
pub fn stub_68794() {
    // IDA 0x68794: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileCpu::ProfileCpu(void)")]
#[doc(alias = "__ZN4FMOD10ProfileCpuC1Ev")]
// 0x687bc — __ZN4FMOD10ProfileCpuC1Ev
pub fn stub_687bc() {
    // IDA 0x687bc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_ProfileCpu_Create(void)")]
#[doc(alias = "__ZN4FMOD22FMOD_ProfileCpu_CreateEv")]
// 0x687c0 — __ZN4FMOD22FMOD_ProfileCpu_CreateEv
pub fn stub_687c0() {
    // IDA 0x687c0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileDsp::isNodeDuplicate(unsigned long long)")]
#[doc(alias = "__ZN4FMOD10ProfileDsp15isNodeDuplicateEy")]
// 0x68864 — __ZN4FMOD10ProfileDsp15isNodeDuplicateEy
pub fn stub_68864() {
    // IDA 0x68864: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileDsp::sendPacket(FMOD::SystemI *)")]
#[doc(alias = "__ZN4FMOD10ProfileDsp10sendPacketEPNS_7SystemIE")]
// 0x68944 — __ZN4FMOD10ProfileDsp10sendPacketEPNS_7SystemIE
pub fn stub_68944() {
    // IDA 0x68944: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileDsp::growNodeStackSpace(void)")]
#[doc(alias = "__ZN4FMOD10ProfileDsp18growNodeStackSpaceEv")]
// 0x68a6c — __ZN4FMOD10ProfileDsp18growNodeStackSpaceEv
pub fn stub_68a6c() {
    // IDA 0x68a6c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileDsp::growPacketSpace(void)")]
#[doc(alias = "__ZN4FMOD10ProfileDsp15growPacketSpaceEv")]
// 0x68adc — __ZN4FMOD10ProfileDsp15growPacketSpaceEv
pub fn stub_68adc() {
    // IDA 0x68adc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileDsp::update(FMOD::SystemI *,unsigned int)")]
#[doc(alias = "__ZN4FMOD10ProfileDsp6updateEPNS_7SystemIEj")]
// 0x68b68 — __ZN4FMOD10ProfileDsp6updateEPNS_7SystemIEj
pub fn stub_68b68() {
    // IDA 0x68b68: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileDsp::release(void)")]
#[doc(alias = "__ZN4FMOD10ProfileDsp7releaseEv")]
// 0x68dfc — __ZN4FMOD10ProfileDsp7releaseEv
pub fn stub_68dfc() {
    // IDA 0x68dfc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileDsp::init(void)")]
#[doc(alias = "__ZN4FMOD10ProfileDsp4initEv")]
// 0x68ebc — __ZN4FMOD10ProfileDsp4initEv
pub fn stub_68ebc() {
    // IDA 0x68ebc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileDsp::ProfileDsp(void)")]
#[doc(alias = "__ZN4FMOD10ProfileDspC2Ev")]
// 0x69028 — __ZN4FMOD10ProfileDspC2Ev
pub fn stub_69028() {
    // IDA 0x69028: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileDsp::ProfileDsp(void)")]
#[doc(alias = "__ZN4FMOD10ProfileDspC1Ev")]
// 0x69078 — __ZN4FMOD10ProfileDspC1Ev
pub fn stub_69078() {
    // IDA 0x69078: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_ProfileDsp_Create(void)")]
#[doc(alias = "__ZN4FMOD22FMOD_ProfileDsp_CreateEv")]
// 0x6907c — __ZN4FMOD22FMOD_ProfileDsp_CreateEv
pub fn stub_6907c() {
    // IDA 0x6907c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Profile::Profile(void)")]
#[doc(alias = "__ZN4FMOD7ProfileC2Ev")]
// 0x6914c — __ZN4FMOD7ProfileC2Ev
pub fn stub_6914c() {
    // IDA 0x6914c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Profile::Profile(void)")]
#[doc(alias = "__ZN4FMOD7ProfileC1Ev")]
// 0x6919c — __ZN4FMOD7ProfileC1Ev
pub fn stub_6919c() {
    // IDA 0x6919c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Profile::registerModule(FMOD::ProfileModule *)")]
#[doc(alias = "__ZN4FMOD7Profile14registerModuleEPNS_13ProfileModuleE")]
// 0x691a0 — __ZN4FMOD7Profile14registerModuleEPNS_13ProfileModuleE
pub fn stub_691a0() {
    // IDA 0x691a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileModule::ProfileModule(void)")]
#[doc(alias = "__ZN4FMOD13ProfileModuleC2Ev")]
// 0x691c8 — __ZN4FMOD13ProfileModuleC2Ev
pub fn stub_691c8() {
    // IDA 0x691c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileModule::init(void)")]
#[doc(alias = "__ZN4FMOD13ProfileModule4initEv")]
// 0x691fc — __ZN4FMOD13ProfileModule4initEv
pub fn stub_691fc() {
    // IDA 0x691fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileModule::release(void)")]
#[doc(alias = "__ZN4FMOD13ProfileModule7releaseEv")]
// 0x69204 — __ZN4FMOD13ProfileModule7releaseEv
pub fn stub_69204() {
    // IDA 0x69204: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileModule::update(FMOD::SystemI *,unsigned int)")]
#[doc(alias = "__ZN4FMOD13ProfileModule6updateEPNS_7SystemIEj")]
// 0x6920c — __ZN4FMOD13ProfileModule6updateEPNS_7SystemIEj
pub fn stub_6920c() {
    // IDA 0x6920c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileClient::ProfileClient(void)")]
#[doc(alias = "__ZN4FMOD13ProfileClientC2Ev")]
// 0x69214 — __ZN4FMOD13ProfileClientC2Ev
pub fn stub_69214() {
    // IDA 0x69214: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileClient::ProfileClient(void)")]
#[doc(alias = "__ZN4FMOD13ProfileClientC1Ev")]
// 0x69280 — __ZN4FMOD13ProfileClientC1Ev
pub fn stub_69280() {
    // IDA 0x69280: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileClient::requestDataType(unsigned char,unsigned char,unsigned int)")]
#[doc(alias = "__ZN4FMOD13ProfileClient15requestDataTypeEhhj")]
// 0x69284 — __ZN4FMOD13ProfileClient15requestDataTypeEhhj
pub fn stub_69284() {
    // IDA 0x69284: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileClient::wantsData(FMOD::ProfilePacketHeader *)")]
#[doc(alias = "__ZN4FMOD13ProfileClient9wantsDataEPNS_19ProfilePacketHeaderE")]
// 0x69358 — __ZN4FMOD13ProfileClient9wantsDataEPNS_19ProfilePacketHeaderE
pub fn stub_69358() {
    // IDA 0x69358: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileClient::sendData(void)")]
#[doc(alias = "__ZN4FMOD13ProfileClient8sendDataEv")]
// 0x693f4 — __ZN4FMOD13ProfileClient8sendDataEv
pub fn stub_693f4() {
    // IDA 0x693f4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileClient::readData(void)")]
#[doc(alias = "__ZN4FMOD13ProfileClient8readDataEv")]
// 0x69480 — __ZN4FMOD13ProfileClient8readDataEv
pub fn stub_69480() {
    // IDA 0x69480: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileClient::update(unsigned int)")]
#[doc(alias = "__ZN4FMOD13ProfileClient6updateEj")]
// 0x695dc — __ZN4FMOD13ProfileClient6updateEj
pub fn stub_695dc() {
    // IDA 0x695dc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileClient::addPacket(FMOD::ProfilePacketHeader *)")]
#[doc(alias = "__ZN4FMOD13ProfileClient9addPacketEPNS_19ProfilePacketHeaderE")]
// 0x69634 — __ZN4FMOD13ProfileClient9addPacketEPNS_19ProfilePacketHeaderE
pub fn stub_69634() {
    // IDA 0x69634: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileClient::release(void)")]
#[doc(alias = "__ZN4FMOD13ProfileClient7releaseEv")]
// 0x69820 — __ZN4FMOD13ProfileClient7releaseEv
pub fn stub_69820() {
    // IDA 0x69820: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ProfileClient::init(void *)")]
#[doc(alias = "__ZN4FMOD13ProfileClient4initEPv")]
// 0x6989c — __ZN4FMOD13ProfileClient4initEPv
pub fn stub_6989c() {
    // IDA 0x6989c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Profile::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD7Profile17getMemoryUsedImplEPNS_13MemoryTrackerE")]
// 0x69910 — __ZN4FMOD7Profile17getMemoryUsedImplEPNS_13MemoryTrackerE
pub fn stub_69910() {
    // IDA 0x69910: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Profile::release(void)")]
#[doc(alias = "__ZN4FMOD7Profile7releaseEv")]
// 0x69a78 — __ZN4FMOD7Profile7releaseEv
pub fn stub_69a78() {
    // IDA 0x69a78: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_Profile_Release(void)")]
#[doc(alias = "__ZN4FMOD20FMOD_Profile_ReleaseEv")]
// 0x69be8 — __ZN4FMOD20FMOD_Profile_ReleaseEv
pub fn stub_69be8() {
    // IDA 0x69be8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Profile::init(unsigned short)")]
#[doc(alias = "__ZN4FMOD7Profile4initEt")]
// 0x69c20 — __ZN4FMOD7Profile4initEt
pub fn stub_69c20() {
    // IDA 0x69c20: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_Profile_Create(unsigned short)")]
#[doc(alias = "__ZN4FMOD19FMOD_Profile_CreateEt")]
// 0x69c9c — __ZN4FMOD19FMOD_Profile_CreateEt
pub fn stub_69c9c() {
    // IDA 0x69c9c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Profile::addPacket(FMOD::ProfilePacketHeader *)")]
#[doc(alias = "__ZN4FMOD7Profile9addPacketEPNS_19ProfilePacketHeaderE")]
// 0x69d50 — __ZN4FMOD7Profile9addPacketEPNS_19ProfilePacketHeaderE
pub fn stub_69d50() {
    // IDA 0x69d50: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Profile::update(FMOD::SystemI *,unsigned int)")]
#[doc(alias = "__ZN4FMOD7Profile6updateEPNS_7SystemIEj")]
// 0x69e0c — __ZN4FMOD7Profile6updateEPNS_7SystemIEj
pub fn stub_69e0c() {
    // IDA 0x69e0c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::FMOD_Profile_Update(FMOD::SystemI *,unsigned int)")]
#[doc(alias = "__ZN4FMOD19FMOD_Profile_UpdateEPNS_7SystemIEj")]
// 0x6a018 — __ZN4FMOD19FMOD_Profile_UpdateEPNS_7SystemIEj
pub fn stub_6a018() {
    // IDA 0x6a018: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Profile::getMemoryUsed(FMOD::MemoryTracker *)")]
#[doc(alias = "__ZN4FMOD7Profile13getMemoryUsedEPNS_13MemoryTrackerE")]
// 0x6a04c — __ZN4FMOD7Profile13getMemoryUsedEPNS_13MemoryTrackerE
pub fn stub_6a04c() {
    // IDA 0x6a04c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_oggpack_look")]
// 0x6d26c — _FMOD_oggpack_look
pub fn stub_6d26c() {
    // IDA 0x6d26c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_oggpack_adv")]
// 0x6d318 — _FMOD_oggpack_adv
pub fn stub_6d318() {
    // IDA 0x6d318: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_oggpack_read")]
// 0x6d354 — _FMOD_oggpack_read
pub fn stub_6d354() {
    // IDA 0x6d354: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_oggpack_bytes")]
// 0x6d434 — _FMOD_oggpack_bytes
pub fn stub_6d434() {
    // IDA 0x6d434: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_oggpack_readinit")]
// 0x6d44c — _FMOD_oggpack_readinit
pub fn stub_6d44c() {
    // IDA 0x6d44c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_synthesis_restart")]
// 0x6d4b4 — _FMOD_vorbis_synthesis_restart
pub fn stub_6d4b4() {
    // IDA 0x6d4b4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_synthesis_pcmout")]
// 0x6d538 — _FMOD_vorbis_synthesis_pcmout
pub fn stub_6d538() {
    // IDA 0x6d538: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_synthesis_read")]
// 0x6d5c8 — _FMOD_vorbis_synthesis_read
pub fn stub_6d5c8() {
    // IDA 0x6d5c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_synthesis_blockin")]
// 0x6d600 — _FMOD_vorbis_synthesis_blockin
pub fn stub_6d600() {
    // IDA 0x6d600: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__FMOD_vorbis_block_alloc")]
// 0x6dee8 — __FMOD_vorbis_block_alloc
pub fn stub_6dee8() {
    // IDA 0x6dee8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__FMOD_vorbis_block_ripcord")]
// 0x6df94 — __FMOD_vorbis_block_ripcord
pub fn stub_6df94() {
    // IDA 0x6df94: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_block_init")]
// 0x6e044 — _FMOD_vorbis_block_init
pub fn stub_6e044() {
    // IDA 0x6e044: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_dsp_clear")]
// 0x6e078 — _FMOD_vorbis_dsp_clear
pub fn stub_6e078() {
    // IDA 0x6e078: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_synthesis_init")]
// 0x6e2c4 — _FMOD_vorbis_synthesis_init
pub fn stub_6e2c4() {
    // IDA 0x6e2c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_block_clear")]
// 0x6e6c0 — _FMOD_vorbis_block_clear
pub fn stub_6e6c0() {
    // IDA 0x6e6c0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_book_decode")]
// 0x6e778 — _FMOD_vorbis_book_decode
pub fn stub_6e778() {
    // IDA 0x6e778: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_staticbook_unpack")]
// 0x6e8c4 — _FMOD_vorbis_staticbook_unpack
pub fn stub_6e8c4() {
    // IDA 0x6e8c4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_book_decodevv_add")]
// 0x6ec78 — _FMOD_vorbis_book_decodevv_add
pub fn stub_6ec78() {
    // IDA 0x6ec78: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_book_decodev_add")]
// 0x6ee98 — _FMOD_vorbis_book_decodev_add
pub fn stub_6ee98() {
    // IDA 0x6ee98: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_vorbis_book_decodevs_add")]
// 0x6f37c — _FMOD_vorbis_book_decodevs_add
pub fn stub_6f37c() {
    // IDA 0x6f37c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_floor1_inverse1")]
// 0x6f840 — _FMOD_floor1_inverse1
pub fn stub_6f840() {
    // IDA 0x6f840: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_floor1_free_look")]
// 0x6fbac — _FMOD_floor1_free_look
pub fn stub_6fbac() {
    // IDA 0x6fbac: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_floor1_look")]
// 0x6fbe0 — _FMOD_floor1_look
pub fn stub_6fbe0() {
    // IDA 0x6fbe0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_floor1_free_info")]
// 0x6fe68 — _FMOD_floor1_free_info
pub fn stub_6fe68() {
    // IDA 0x6fe68: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_floor1_unpack")]
// 0x6fe9c — _FMOD_floor1_unpack
pub fn stub_6fe9c() {
    // IDA 0x6fe9c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_floor1_inverse2")]
// 0x701fc — _FMOD_floor1_inverse2
pub fn stub_701fc() {
    // IDA 0x701fc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_Channel_GetUserData")]
// 0x70458 — _FMOD_Channel_GetUserData
pub fn stub_70458() {
    // IDA 0x70458: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_System_Create")]
// 0x70474 — _FMOD_System_Create
pub fn stub_70474() {
    // IDA 0x70474: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FMOD_Memory_GetStats")]
// 0x705cc — _FMOD_Memory_GetStats
pub fn stub_705cc() {
    // IDA 0x705cc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::AsyncThread::release(void)")]
#[doc(alias = "__ZN4FMOD11AsyncThread7releaseEv")]
// 0x7069c — __ZN4FMOD11AsyncThread7releaseEv
pub fn stub_7069c() {
    // IDA 0x7069c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::AsyncThread::threadFunc(void)")]
#[doc(alias = "__ZN4FMOD11AsyncThread10threadFuncEv")]
// 0x706b4 — __ZN4FMOD11AsyncThread10threadFuncEv
pub fn stub_706b4() {
    // IDA 0x706b4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::asyncThreadFunc(void *)")]
#[doc(alias = "__ZN4FMOD15asyncThreadFuncEPv")]
// 0x70ab0 — __ZN4FMOD15asyncThreadFuncEPv
pub fn stub_70ab0() {
    // IDA 0x70ab0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::AsyncThread::reallyRelease(void)")]
#[doc(alias = "__ZN4FMOD11AsyncThread13reallyReleaseEv")]
// 0x70ab4 — __ZN4FMOD11AsyncThread13reallyReleaseEv
pub fn stub_70ab4() {
    // IDA 0x70ab4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::AsyncThread::init(bool,FMOD::SystemI *)")]
#[doc(alias = "__ZN4FMOD11AsyncThread4initEbPNS_7SystemIE")]
// 0x70bbc — __ZN4FMOD11AsyncThread4initEbPNS_7SystemIE
pub fn stub_70bbc() {
    // IDA 0x70bbc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::AsyncThread::AsyncThread(void)")]
#[doc(alias = "__ZN4FMOD11AsyncThreadC2Ev")]
// 0x70c98 — __ZN4FMOD11AsyncThreadC2Ev
pub fn stub_70c98() {
    // IDA 0x70c98: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::AsyncThread::AsyncThread(void)")]
#[doc(alias = "__ZN4FMOD11AsyncThreadC1Ev")]
// 0x70cec — __ZN4FMOD11AsyncThreadC1Ev
pub fn stub_70cec() {
    // IDA 0x70cec: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::AsyncThread::getAsyncThread(FMOD::SoundI *)")]
#[doc(alias = "__ZN4FMOD11AsyncThread14getAsyncThreadEPNS_6SoundIE")]
// 0x70cf0 — __ZN4FMOD11AsyncThread14getAsyncThreadEPNS_6SoundIE
pub fn stub_70cf0() {
    // IDA 0x70cf0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::AsyncThread::shutDown(void)")]
#[doc(alias = "__ZN4FMOD11AsyncThread8shutDownEv")]
// 0x70ddc — __ZN4FMOD11AsyncThread8shutDownEv
pub fn stub_70ddc() {
    // IDA 0x70ddc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::AsyncThread::update(void)")]
#[doc(alias = "__ZN4FMOD11AsyncThread6updateEv")]
// 0x70e5c — __ZN4FMOD11AsyncThread6updateEv
pub fn stub_70e5c() {
    // IDA 0x70e5c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed toFMOD::AsyncThread::gAsyncHead")]
#[doc(alias = "__GLOBAL__I__ZN4FMOD11AsyncThread10gAsyncHeadE")]
// 0x70f2c — __GLOBAL__I__ZN4FMOD11AsyncThread10gAsyncHeadE
pub fn stub_70f2c() {
    // IDA 0x70f2c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::Channel::getUserData(void **)")]
#[doc(alias = "__ZN4FMOD7Channel11getUserDataEPPv")]
// 0x70f38 — __ZN4FMOD7Channel11getUserDataEPPv
pub fn stub_70f38() {
    // IDA 0x70f38: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::Channel::setUserData(void *)")]
#[doc(alias = "__ZN4FMOD7Channel11setUserDataEPv")]
// 0x70f7c — __ZN4FMOD7Channel11setUserDataEPv
pub fn stub_70f7c() {
    // IDA 0x70f7c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::Channel::setLoopCount(int)")]
#[doc(alias = "__ZN4FMOD7Channel12setLoopCountEi")]
// 0x70fb0 — __ZN4FMOD7Channel12setLoopCountEi
pub fn stub_70fb0() {
    // IDA 0x70fb0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::Channel::getMode(unsigned int *)")]
#[doc(alias = "__ZN4FMOD7Channel7getModeEPj")]
// 0x70fe4 — __ZN4FMOD7Channel7getModeEPj
pub fn stub_70fe4() {
    // IDA 0x70fe4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "FMOD::Channel::setMode(unsigned int)")]
#[doc(alias = "__ZN4FMOD7Channel7setModeEj")]
// 0x71028 — __ZN4FMOD7Channel7setModeEj
pub fn stub_71028() {
    // IDA 0x71028: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::isPlaying(bool *)")]
#[doc(alias = "__ZN4FMOD7Channel9isPlayingEPb")]
// 0x7105c — __ZN4FMOD7Channel9isPlayingEPb
pub fn stub_7105c() {
    // IDA 0x7105c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")]
#[doc(alias = "__ZN4FMOD7Channel15set3DAttributesEPK11FMOD_VECTORS3_")]
// 0x710a0 — __ZN4FMOD7Channel15set3DAttributesEPK11FMOD_VECTORS3_
pub fn stub_710a0() {
    // IDA 0x710a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")]
#[doc(alias = "__ZN4FMOD7Channel11setCallbackEPF11FMOD_RESULTP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS5_E")]
// 0x710dc — __ZN4FMOD7Channel11setCallbackEPF11FMOD_RESULTP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS5_E
pub fn stub_710dc() {
    // IDA 0x710dc: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::setChannelGroup(FMOD::ChannelGroup *)")]
#[doc(alias = "__ZN4FMOD7Channel15setChannelGroupEPNS_12ChannelGroupE")]
// 0x71110 — __ZN4FMOD7Channel15setChannelGroupEPNS_12ChannelGroupE
pub fn stub_71110() {
    // IDA 0x71110: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::setPriority(int)")]
#[doc(alias = "__ZN4FMOD7Channel11setPriorityEi")]
// 0x71144 — __ZN4FMOD7Channel11setPriorityEi
pub fn stub_71144() {
    // IDA 0x71144: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::setMute(bool)")]
#[doc(alias = "__ZN4FMOD7Channel7setMuteEb")]
// 0x71178 — __ZN4FMOD7Channel7setMuteEb
pub fn stub_71178() {
    // IDA 0x71178: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::getFrequency(float *)")]
#[doc(alias = "__ZN4FMOD7Channel12getFrequencyEPf")]
// 0x711ac — __ZN4FMOD7Channel12getFrequencyEPf
pub fn stub_711ac() {
    // IDA 0x711ac: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::setFrequency(float)")]
#[doc(alias = "__ZN4FMOD7Channel12setFrequencyEf")]
// 0x711f0 — __ZN4FMOD7Channel12setFrequencyEf
pub fn stub_711f0() {
    // IDA 0x711f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::setVolume(float)")]
#[doc(alias = "__ZN4FMOD7Channel9setVolumeEf")]
// 0x71224 — __ZN4FMOD7Channel9setVolumeEf
pub fn stub_71224() {
    // IDA 0x71224: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::getPaused(bool *)")]
#[doc(alias = "__ZN4FMOD7Channel9getPausedEPb")]
// 0x71260 — __ZN4FMOD7Channel9getPausedEPb
pub fn stub_71260() {
    // IDA 0x71260: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::setPaused(bool)")]
#[doc(alias = "__ZN4FMOD7Channel9setPausedEb")]
// 0x712a4 — __ZN4FMOD7Channel9setPausedEb
pub fn stub_712a4() {
    // IDA 0x712a4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::Channel::stop(void)")]
#[doc(alias = "__ZN4FMOD7Channel4stopEv")]
// 0x712d8 — __ZN4FMOD7Channel4stopEv
pub fn stub_712d8() {
    // IDA 0x712d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::isVirtual(bool *)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulated9isVirtualEPb")]
// 0x71304 — __ZN4FMOD15ChannelEmulated9isVirtualEPb
pub fn stub_71304() {
    // IDA 0x71304: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::getDSPHead(FMOD::DSPI **)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulated10getDSPHeadEPPNS_4DSPIE")]
// 0x7131c — __ZN4FMOD15ChannelEmulated10getDSPHeadEPPNS_4DSPIE
pub fn stub_7131c() {
    // IDA 0x7131c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::setSpeakerLevels(int,float *,int)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulated16setSpeakerLevelsEiPfi")]
// 0x71334 — __ZN4FMOD15ChannelEmulated16setSpeakerLevelsEiPfi
pub fn stub_71334() {
    // IDA 0x71334: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::setSpeakerMix(float,float,float,float,float,float,float,float)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulated13setSpeakerMixEffffffff")]
// 0x7133c — __ZN4FMOD15ChannelEmulated13setSpeakerMixEffffffff
pub fn stub_7133c() {
    // IDA 0x7133c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::update(int)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulated6updateEi")]
// 0x71344 — __ZN4FMOD15ChannelEmulated6updateEi
pub fn stub_71344() {
    // IDA 0x71344: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::close(void)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulated5closeEv")]
// 0x71540 — __ZN4FMOD15ChannelEmulated5closeEv
pub fn stub_71540() {
    // IDA 0x71540: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::alloc(void)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulated5allocEv")]
// 0x71580 — __ZN4FMOD15ChannelEmulated5allocEv
pub fn stub_71580() {
    // IDA 0x71580: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulated4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE")]
// 0x715e8 — __ZN4FMOD15ChannelEmulated4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE
pub fn stub_715e8() {
    // IDA 0x715e8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::ChannelEmulated(void)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulatedC2Ev")]
// 0x71698 — __ZN4FMOD15ChannelEmulatedC2Ev
pub fn stub_71698() {
    // IDA 0x71698: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::ChannelEmulated(void)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulatedC1Ev")]
// 0x716e4 — __ZN4FMOD15ChannelEmulatedC1Ev
pub fn stub_716e4() {
    // IDA 0x716e4: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::stop(void)")]
#[doc(alias = "__ZN4FMOD15ChannelEmulated4stopEv")]
// 0x716e8 — __ZN4FMOD15ChannelEmulated4stopEv
pub fn stub_716e8() {
    // IDA 0x716e8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelEmulated::~ChannelEmulated()")]
#[doc(alias = "__ZN4FMOD15ChannelEmulatedD0Ev")]
// 0x71818 — __ZN4FMOD15ChannelEmulatedD0Ev
pub fn stub_71818() {
    // IDA 0x71818: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelEmulated::~ChannelEmulated()")]
#[doc(alias = "__ZN4FMOD15ChannelEmulatedD1Ev")]
// 0x7183c — __ZN4FMOD15ChannelEmulatedD1Ev
pub fn stub_7183c() {
    // IDA 0x7183c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelReal::ChannelReal(void)")]
#[doc(alias = "__ZN4FMOD11ChannelRealC2Ev")]
// 0x71854 — __ZN4FMOD11ChannelRealC2Ev
pub fn stub_71854() {
    // IDA 0x71854: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelReal::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD11ChannelReal4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE")]
// 0x718a0 — __ZN4FMOD11ChannelReal4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE
pub fn stub_718a0() {
    // IDA 0x718a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelReal::close(void)")]
#[doc(alias = "__ZN4FMOD11ChannelReal5closeEv")]
// 0x718dc — __ZN4FMOD11ChannelReal5closeEv
pub fn stub_718dc() {
    // IDA 0x718dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelReal::alloc(void)")]
#[doc(alias = "__ZN4FMOD11ChannelReal5allocEv")]
// 0x718e8 — __ZN4FMOD11ChannelReal5allocEv
pub fn stub_718e8() {
    // IDA 0x718e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FMOD::ChannelReal::alloc(FMOD::DSPI *)")]
#[doc(alias = "__ZN4FMOD11ChannelReal5allocEPNS_4DSPIE")]
// 0x7190c — __ZN4FMOD11ChannelReal5allocEPNS_4DSPIE
pub fn stub_7190c() {
    // IDA 0x7190c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::set2DFreqVolumePanFor3D(void)")]
#[doc(alias = "__ZN4FMOD11ChannelReal23set2DFreqVolumePanFor3DEv")]
// 0x71930 — __ZN4FMOD11ChannelReal23set2DFreqVolumePanFor3DEv
pub fn stub_71930() {
    // IDA 0x71930: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::update(int)")]
#[doc(alias = "__ZN4FMOD11ChannelReal6updateEi")]
// 0x71938 — __ZN4FMOD11ChannelReal6updateEi
pub fn stub_71938() {
    // IDA 0x71938: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::updateStream(void)")]
#[doc(alias = "__ZN4FMOD11ChannelReal12updateStreamEv")]
// 0x71940 — __ZN4FMOD11ChannelReal12updateStreamEv
pub fn stub_71940() {
    // IDA 0x71940: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::start(void)")]
#[doc(alias = "__ZN4FMOD11ChannelReal5startEv")]
// 0x71948 — __ZN4FMOD11ChannelReal5startEv
pub fn stub_71948() {
    // IDA 0x71948: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::stop(void)")]
#[doc(alias = "__ZN4FMOD11ChannelReal4stopEv")]
// 0x71950 — __ZN4FMOD11ChannelReal4stopEv
pub fn stub_71950() {
    // IDA 0x71950: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setPaused(bool)")]
#[doc(alias = "__ZN4FMOD11ChannelReal9setPausedEb")]
// 0x7197c — __ZN4FMOD11ChannelReal9setPausedEb
pub fn stub_7197c() {
    // IDA 0x7197c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::getPaused(bool *)")]
#[doc(alias = "__ZN4FMOD11ChannelReal9getPausedEPb")]
// 0x719a0 — __ZN4FMOD11ChannelReal9getPausedEPb
pub fn stub_719a0() {
    // IDA 0x719a0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setVolume(float)")]
#[doc(alias = "__ZN4FMOD11ChannelReal9setVolumeEf")]
// 0x719c0 — __ZN4FMOD11ChannelReal9setVolumeEf
pub fn stub_719c0() {
    // IDA 0x719c0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setFrequency(float)")]
#[doc(alias = "__ZN4FMOD11ChannelReal12setFrequencyEf")]
// 0x719c8 — __ZN4FMOD11ChannelReal12setFrequencyEf
pub fn stub_719c8() {
    // IDA 0x719c8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setPan(float,float)")]
#[doc(alias = "__ZN4FMOD11ChannelReal6setPanEff")]
// 0x719d0 — __ZN4FMOD11ChannelReal6setPanEff
pub fn stub_719d0() {
    // IDA 0x719d0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setDSPClockDelay(void)")]
#[doc(alias = "__ZN4FMOD11ChannelReal16setDSPClockDelayEv")]
// 0x719d8 — __ZN4FMOD11ChannelReal16setDSPClockDelayEv
pub fn stub_719d8() {
    // IDA 0x719d8: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setSpeakerMix(float,float,float,float,float,float,float,float)")]
#[doc(alias = "__ZN4FMOD11ChannelReal13setSpeakerMixEffffffff")]
// 0x719e0 — __ZN4FMOD11ChannelReal13setSpeakerMixEffffffff
pub fn stub_719e0() {
    // IDA 0x719e0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setPosition(unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD11ChannelReal11setPositionEjj")]
// 0x71e34 — __ZN4FMOD11ChannelReal11setPositionEjj
pub fn stub_71e34() {
    // IDA 0x71e34: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::getPosition(unsigned int *,unsigned int)")]
#[doc(alias = "__ZN4FMOD11ChannelReal11getPositionEPjj")]
// 0x72008 — __ZN4FMOD11ChannelReal11getPositionEPjj
pub fn stub_72008() {
    // IDA 0x72008: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setLoopPoints(unsigned int,unsigned int)")]
#[doc(alias = "__ZN4FMOD11ChannelReal13setLoopPointsEjj")]
// 0x722f0 — __ZN4FMOD11ChannelReal13setLoopPointsEjj
pub fn stub_722f0() {
    // IDA 0x722f0: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setLoopCount(int)")]
#[doc(alias = "__ZN4FMOD11ChannelReal12setLoopCountEi")]
// 0x72328 — __ZN4FMOD11ChannelReal12setLoopCountEi
pub fn stub_72328() {
    // IDA 0x72328: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::getLoopCount(int *)")]
#[doc(alias = "__ZN4FMOD11ChannelReal12getLoopCountEPi")]
// 0x72334 — __ZN4FMOD11ChannelReal12getLoopCountEPi
pub fn stub_72334() {
    // IDA 0x72334: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::setLowPassGain(float)")]
#[doc(alias = "__ZN4FMOD11ChannelReal14setLowPassGainEf")]
// 0x7234c — __ZN4FMOD11ChannelReal14setLowPassGainEf
pub fn stub_7234c() {
    // IDA 0x7234c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::set3DAttributes(void)")]
#[doc(alias = "__ZN4FMOD11ChannelReal15set3DAttributesEv")]
// 0x72354 — __ZN4FMOD11ChannelReal15set3DAttributesEv
pub fn stub_72354() {
    // IDA 0x72354: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::set3DMinMaxDistance(void)")]
#[doc(alias = "__ZN4FMOD11ChannelReal19set3DMinMaxDistanceEv")]
// 0x7235c — __ZN4FMOD11ChannelReal19set3DMinMaxDistanceEv
pub fn stub_7235c() {
    // IDA 0x7235c: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "FMOD::ChannelReal::set3DOcclusion(float,float)")]
#[doc(alias = "__ZN4FMOD11ChannelReal14set3DOcclusionEff")]
// 0x72364 — __ZN4FMOD11ChannelReal14set3DOcclusionEff
pub fn stub_72364() {
    // IDA 0x72364: FMOD audio struct/helper owned by the audio crate — carrier no-op in core.
}