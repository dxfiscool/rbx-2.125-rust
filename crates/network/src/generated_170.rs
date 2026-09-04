//! Auto-generated skeletons for rbx-network — RakNet|Network|Replicat EA-sorted asc
//! Filter: RakNet|Network|Replicat -> 5197 funcs (cs), 1050 remaining before batch; batch EA-sorted asc next 150 filtered
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +150 stubs | range 0xa6eab4..0xa85188 | existing 4147 -> 4297 total (filtered EA-sorted asc, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xa6eab4 — __ZN6RakNet9RakString6AssignEPKcPv
// demangled: RakNet::RakString::Assign(char const*,void *)
// type: int __fastcall(RakNet::RakString *this, const char *__format, va_list)
#[doc(alias = "RakNet::RakString::Assign(char const*,void *)")]
pub fn stub_a6eab4(formatted: &str) -> String {
 // IDA 0xa6eab4: store the formatted text.
 crate::socket::rak_string_format(formatted)
}

// 0xa6ec58 — __ZN6RakNet9RakStringC1EPKcz
// demangled: RakNet::RakString::RakString(char const*,...)
// type: RakNet::RakString *(RakNet::RakString *this, const char *, ...)
#[doc(alias = "RakNet::RakString::RakString(char const*,...)")]
pub fn stub_a6ec58(formatted: &str) -> String {
 // IDA 0xa6ec58: format through Assign.
 crate::socket::rak_string_format(formatted)
}

// 0xa6ec7c — __ZN6RakNet9RakStringD1Ev
// demangled: RakNet::RakString::~RakString()
// type: void __fastcall(RakNet::RakString *__hidden this)
#[doc(alias = "RakNet::RakString::~RakString()")]
pub fn stub_a6ec7c(text: String) {
 // IDA 0xa6ec7c: frees; Rust drops it.
 drop(text);
}

// 0xa6ec8c — __ZN6RakNet9RakString4FreeEv
// demangled: RakNet::RakString::Free(void)
// type: void __fastcall(RakNet::SimpleMutex ***this)
#[doc(alias = "RakNet::RakString::Free(void)")]
pub fn stub_a6ec8c(s: &mut String) {
 // IDA 0xa6ec8c: release ending empty.
 crate::socket::rak_string_free(s)
}

// 0xa6eed4 — __ZN6RakNet9RakStringaSERKS0_
// demangled: RakNet::RakString::operator=(RakNet::RakString const&)
// type: RakNet::RakString *__fastcall(RakNet::RakString *, RakNet::SimpleMutex ***)
#[doc(alias = "RakNet::RakString::operator=(RakNet::RakString const&)")]
pub fn stub_a6eed4(dst: &mut String, src: &str) {
 // IDA 0xa6eed4: copy the text.
 crate::socket::rak_string_assign(dst, src)
}

// 0xa6ef14 — __ZN6RakNet9RakString8AllocateEm
// demangled: RakNet::RakString::Allocate(unsigned long)
// type: void __fastcall(RakNet::RakString *this, unsigned int)
#[doc(alias = "RakNet::RakString::Allocate(unsigned long)")]
pub fn stub_a6ef14(s: &mut String, capacity: usize) {
 // IDA 0xa6ef14: reserve through the pool.
 crate::socket::rak_string_allocate(s, capacity)
}

// 0xa6f1ac — __ZN6RakNet9RakString14IPAddressMatchEPKc
// demangled: RakNet::RakString::IPAddressMatch(char const*)
// type: bool __fastcall(RakNet::RakString *this, const char *__s)
#[doc(alias = "RakNet::RakString::IPAddressMatch(char const*)")]
pub fn stub_a6f1ac(pattern: &str, addr: &str) -> bool {
 // IDA 0xa6f1ac: prefix walk with star escape.
 crate::socket::ip_address_match(pattern, addr)
}

// 0xa6f210 — __ZN6RakNet9RakString17FreeMemoryNoMutexEv
// demangled: RakNet::RakString::FreeMemoryNoMutex(void)
// type: void __fastcall(RakNet::RakString *this)
#[doc(alias = "RakNet::RakString::FreeMemoryNoMutex(void)")]
pub fn stub_a6f210() {
 // IDA 0xa6f210: drain the global pool.
 crate::socket::rak_string_free_pool()
}

// 0xa6f328 — __ZNK6RakNet9RakString9SerializeEPNS_9BitStreamE
// demangled: RakNet::RakString::Serialize(RakNet::BitStream *)const
// type: RakNet::BitStream *__fastcall(RakNet::RakString *this, RakNet::BitStream *)
#[doc(alias = "RakNet::RakString::Serialize(RakNet::BitStream *)const")]
pub fn stub_a6f328(stream: &mut crate::bitstream::BitStream, s: &str) {
 // IDA 0xa6f328: u16 length plus aligned bytes.
 crate::socket::rak_string_serialize(stream, s)
}

// 0xa6f358 — __ZN6RakNet9RakString11DeserializeEPNS_9BitStreamE
// demangled: RakNet::RakString::Deserialize(RakNet::BitStream *)
// type: int __fastcall(RakNet::SimpleMutex ***this, RakNet::BitStream *)
#[doc(alias = "RakNet::RakString::Deserialize(RakNet::BitStream *)")]
pub fn stub_a6f358(stream: &mut crate::bitstream::BitStream) -> Option<String> {
 // IDA 0xa6f358: length then aligned bytes.
 crate::socket::rak_string_deserialize(stream)
}

// 0xa6f3c0 — __ZN14DataStructures4ListIPN6RakNet9RakString12SharedStringEED1Ev
// demangled: DataStructures::List<RakNet::RakString::SharedString *>::~List()
// type: int __fastcall(int)
#[doc(alias = "DataStructures::List<RakNet::RakString::SharedString *>::~List()")]
pub fn stub_a6f3c0() {
 // IDA 0xa6f3c0: node release stays engine-side.
 crate::socket::rak_string_list_drop()
}

// 0xa6fa3c — __ZN6RakNet9RakThread6CreateEPFPvS1_ES1_i
// demangled: RakNet::RakThread::Create(void * (*)(void *),void *,int)
// type: int __fastcall(RakNet::RakThread *this, void *(__fastcall *)(void *), int, int)
#[doc(alias = "RakNet::RakThread::Create(void * (*)(void *),void *,int)")]
pub fn stub_a6fa3c() {
 // IDA 0xa6fa3c: thread spawn stays engine-side.
 crate::socket::spawn_rak_thread()
}

// 0xa70260 — __ZN6RakNet12RakNetRandomC1Ev
// demangled: RakNet::RakNetRandom::RakNetRandom(void)
// type: int __fastcall(int this)
#[doc(alias = "RakNet::RakNetRandom::RakNetRandom(void)")]
pub fn stub_a70260() -> crate::socket::RakNetRandom {
 // IDA 0xa70260: use count starts at -1.
 crate::socket::RakNetRandom::new()
}

// 0xa70270 — __ZN6RakNet12RakNetRandomD1Ev
// demangled: RakNet::RakNetRandom::~RakNetRandom()
// type: void __fastcall(RakNet::RakNetRandom *__hidden this)
#[doc(alias = "RakNet::RakNetRandom::~RakNetRandom()")]
pub fn stub_a70270(rng: crate::socket::RakNetRandom) {
 // IDA 0xa70270: frees; Rust drops it.
 drop(rng);
}

// 0xa70278 — __ZN6RakNet12RakNetRandom6SeedMTEj
// demangled: RakNet::RakNetRandom::SeedMT(unsigned int)
// type: unsigned int *__fastcall(unsigned int *this, unsigned int)
#[doc(alias = "RakNet::RakNetRandom::SeedMT(unsigned int)")]
pub fn stub_a70278(rng: &mut crate::socket::RakNetRandom, seed: u32) {
 // IDA 0xa70278: multiply-only chain with 0x10DCD.
 rng.seed_mt(seed)
}

// 0xa702a4 — __ZN6RakNet12RakNetRandom8RandomMTEv
// demangled: RakNet::RakNetRandom::RandomMT(void)
// type: unsigned int __fastcall(RakNet::RakNetRandom *this)
#[doc(alias = "RakNet::RakNetRandom::RandomMT(void)")]
pub fn stub_a702a4(rng: &mut crate::socket::RakNetRandom) -> u32 {
 // IDA 0xa702a4: countdown with twist refill.
 rng.random_mt()
}

// 0xa7090c — __ZN6RakNet22SplitPacketChannelCompERKtRKPNS_18SplitPacketChannelE
// demangled: RakNet::SplitPacketChannelComp(unsigned short const&,RakNet::SplitPacketChannel * const&)
// type: int __fastcall(unsigned __int16 *, int)
#[doc(alias = "RakNet::SplitPacketChannelComp(unsigned short const&,RakNet::SplitPacketChannel * const&)")]
pub fn stub_a7090c() -> ! {
    todo!("0xa7090c RakNet::SplitPacketChannelComp(unsigned short const&,RakNet::SplitPacketChannel * const&)")
}

// 0xa7092c — __ZN6RakNet16ReliabilityLayerC1Ev
// demangled: RakNet::ReliabilityLayer::ReliabilityLayer(void)
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ReliabilityLayer(void)")]
pub fn stub_a7092c() -> ! {
    todo!("0xa7092c RakNet::ReliabilityLayer::ReliabilityLayer(void)")
}

// 0xa70938 — __ZN6RakNet16ReliabilityLayerC2Ev
// demangled: RakNet::ReliabilityLayer::ReliabilityLayer(void)
// type: RakNet::ReliabilityLayer *__fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ReliabilityLayer(void)")]
pub fn stub_a70938() -> ! {
    todo!("0xa70938 RakNet::ReliabilityLayer::ReliabilityLayer(void)")
}

// 0xa7142c — __ZN6RakNet16ReliabilityLayer19InitializeVariablesEv
// demangled: RakNet::ReliabilityLayer::InitializeVariables(void)
// type: void __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::InitializeVariables(void)")]
pub fn stub_a7142c() -> ! {
    todo!("0xa7142c RakNet::ReliabilityLayer::InitializeVariables(void)")
}

// 0xa715f8 — __ZN6RakNet16ReliabilityLayerD1Ev
// demangled: RakNet::ReliabilityLayer::~ReliabilityLayer()
// type: void __fastcall(RakNet::ReliabilityLayer *__hidden this)
#[doc(alias = "RakNet::ReliabilityLayer::~ReliabilityLayer()")]
pub fn stub_a715f8() -> ! {
    todo!("0xa715f8 RakNet::ReliabilityLayer::~ReliabilityLayer()")
}

// 0xa71604 — __ZN6RakNet16ReliabilityLayerD2Ev
// demangled: RakNet::ReliabilityLayer::~ReliabilityLayer()
// type: void __fastcall(RakNet::ReliabilityLayer *__hidden this)
#[doc(alias = "RakNet::ReliabilityLayer::~ReliabilityLayer()")]
pub fn stub_a71604() -> ! {
    todo!("0xa71604 RakNet::ReliabilityLayer::~ReliabilityLayer()")
}

// 0xa723c0 — __ZN6RakNet16ReliabilityLayer5ResetEbib
// demangled: RakNet::ReliabilityLayer::Reset(bool,int,bool)
// type: _QWORD *__fastcall(RakNet::ReliabilityLayer *this, int, int, bool)
#[doc(alias = "RakNet::ReliabilityLayer::Reset(bool,int,bool)")]
pub fn stub_a723c0() -> ! {
    todo!("0xa723c0 RakNet::ReliabilityLayer::Reset(bool,int,bool)")
}

// 0xa723f8 — __ZN6RakNet16ReliabilityLayer14SetTimeoutTimeEj
// demangled: RakNet::ReliabilityLayer::SetTimeoutTime(unsigned int)
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "RakNet::ReliabilityLayer::SetTimeoutTime(unsigned int)")]
pub fn stub_a723f8() -> ! {
    todo!("0xa723f8 RakNet::ReliabilityLayer::SetTimeoutTime(unsigned int)")
}

// 0xa72400 — __ZN6RakNet16ReliabilityLayer14GetTimeoutTimeEv
// demangled: RakNet::ReliabilityLayer::GetTimeoutTime(void)
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::GetTimeoutTime(void)")]
pub fn stub_a72400() -> ! {
    todo!("0xa72400 RakNet::ReliabilityLayer::GetTimeoutTime(void)")
}

// 0xa72408 — __ZN6RakNet16ReliabilityLayer20FreeThreadSafeMemoryEv
// demangled: RakNet::ReliabilityLayer::FreeThreadSafeMemory(void)
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::FreeThreadSafeMemory(void)")]
pub fn stub_a72408() -> ! {
    todo!("0xa72408 RakNet::ReliabilityLayer::FreeThreadSafeMemory(void)")
}

// 0xa72d5c — __ZN6RakNet16ReliabilityLayer24ClearPacketsAndDatagramsEv
// demangled: RakNet::ReliabilityLayer::ClearPacketsAndDatagrams(void)
// type: unsigned int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ClearPacketsAndDatagrams(void)")]
pub fn stub_a72d5c() -> ! {
    todo!("0xa72d5c RakNet::ReliabilityLayer::ClearPacketsAndDatagrams(void)")
}

// 0xa72e94 — __ZN6RakNet16ReliabilityLayer38HandleSocketReceiveFromConnectedPlayerEPKcjRNS_13SystemAddressERN14DataStructures4ListIPNS_16PluginInterface2EEEiiPNS_12RakNetRandomEtjyRNS_9BitStreamE
// demangled: RakNet::ReliabilityLayer::HandleSocketReceiveFromConnectedPlayer(char const*,unsigned int,RakNet::SystemAddress &,DataStructures::List<RakNet::PluginInterface2 *> &,int,int,RakNet::RakNetRandom *,unsigned short,unsigned int,unsigned long long,RakNet::BitStream &)
// type: int __fastcall(int, unsigned __int8 *, unsigned int, _DWORD *, _DWORD *, int, int, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, unsigned __int64, RakNet::BitStream *)
#[doc(alias = "RakNet::ReliabilityLayer::HandleSocketReceiveFromConnectedPlayer(char const*,unsigned int,RakNet::SystemAddress &,DataStructures::List<RakNet::PluginInterface2 *> &,int,int,RakNet::RakNetRandom *,unsigned short,unsigned int,unsigned long long,RakNet::BitStream &)")]
pub fn stub_a72e94() -> ! {
    todo!("0xa72e94 RakNet::ReliabilityLayer::HandleSocketReceiveFromConnectedPlayer(char const*,unsigned int,RakNet::SystemAddress &,DataStructures::List<RakNet::PluginInterface2 *> &,int,int,RakNet::RakNetRandom *,unsigned short,unsigned int,unsigned long long,RakNet::BitStream &)")
}

// 0xa74514 — __ZN6RakNet16ReliabilityLayer57RemovePacketFromResendListAndDeleteOlderReliableSequencedENS_8uint24_tEyRN14DataStructures4ListIPNS_16PluginInterface2EEERKNS_13SystemAddressE
// demangled: RakNet::ReliabilityLayer::RemovePacketFromResendListAndDeleteOlderReliableSequenced(RakNet::uint24_t,unsigned long long,DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::SystemAddress const&)
// type: int __fastcall(int, _DWORD *, unsigned __int64, _DWORD *, _DWORD *)
#[doc(alias = "RakNet::ReliabilityLayer::RemovePacketFromResendListAndDeleteOlderReliableSequenced(RakNet::uint24_t,unsigned long long,DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::SystemAddress const&)")]
pub fn stub_a74514() -> ! {
    todo!("0xa74514 RakNet::ReliabilityLayer::RemovePacketFromResendListAndDeleteOlderReliableSequenced(RakNet::uint24_t,unsigned long long,DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::SystemAddress const&)")
}

// 0xa74750 — __ZN6RakNet16ReliabilityLayer33CreateInternalPacketFromBitStreamEPNS_9BitStreamEy
// demangled: RakNet::ReliabilityLayer::CreateInternalPacketFromBitStream(RakNet::BitStream *,unsigned long long)
// type: int __fastcall(RakNet::ReliabilityLayer *this, RakNet::BitStream *, unsigned __int64)
#[doc(alias = "RakNet::ReliabilityLayer::CreateInternalPacketFromBitStream(RakNet::BitStream *,unsigned long long)")]
pub fn stub_a74750() -> ! {
    todo!("0xa74750 RakNet::ReliabilityLayer::CreateInternalPacketFromBitStream(RakNet::BitStream *,unsigned long long)")
}

// 0xa749fc — __ZN6RakNet16ReliabilityLayer25InsertIntoSplitPacketListEPNS_14InternalPacketEy
// demangled: RakNet::ReliabilityLayer::InsertIntoSplitPacketList(RakNet::InternalPacket *,unsigned long long)
// type: unsigned int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::InsertIntoSplitPacketList(RakNet::InternalPacket *,unsigned long long)")]
pub fn stub_a749fc() -> ! {
    todo!("0xa749fc RakNet::ReliabilityLayer::InsertIntoSplitPacketList(RakNet::InternalPacket *,unsigned long long)")
}

// 0xa74c88 — __ZN6RakNet16ReliabilityLayer30BuildPacketFromSplitPacketListEtyiRNS_13SystemAddressEPNS_12RakNetRandomEtjRNS_9BitStreamE
// demangled: RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList(unsigned short,unsigned long long,int,RakNet::SystemAddress &,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)
// type: int __fastcall(RakNet::ReliabilityLayer *this, unsigned int, unsigned __int64, int, RakNet::SystemAddress *, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, RakNet::BitStream *)
#[doc(alias = "RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList(unsigned short,unsigned long long,int,RakNet::SystemAddress &,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")]
pub fn stub_a74c88() -> ! {
    todo!("0xa74c88 RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList(unsigned short,unsigned long long,int,RakNet::SystemAddress &,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")
}

// 0xa74d64 — __ZN6RakNet16ReliabilityLayer7ReceiveEPPh
// demangled: RakNet::ReliabilityLayer::Receive(unsigned char **)
// type: int __fastcall(RakNet::ReliabilityLayer *this, unsigned __int8 **)
#[doc(alias = "RakNet::ReliabilityLayer::Receive(unsigned char **)")]
pub fn stub_a74d64() -> ! {
    todo!("0xa74d64 RakNet::ReliabilityLayer::Receive(unsigned char **)")
}

// 0xa74dc0 — __ZN6RakNet16ReliabilityLayer4SendEPcj14PacketPriority17PacketReliabilityhbiyj
// demangled: RakNet::ReliabilityLayer::Send(char *,unsigned int,PacketPriority,PacketReliability,unsigned char,bool,int,unsigned long long,unsigned int)
// type: int __fastcall(int, const void *, int, unsigned int, unsigned int, unsigned int, int, int, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::Send(char *,unsigned int,PacketPriority,PacketReliability,unsigned char,bool,int,unsigned long long,unsigned int)")]
pub fn stub_a74dc0() -> ! {
    todo!("0xa74dc0 RakNet::ReliabilityLayer::Send(char *,unsigned int,PacketPriority,PacketReliability,unsigned char,bool,int,unsigned long long,unsigned int)")
}

// 0xa75100 — __ZN6RakNet16ReliabilityLayer11SplitPacketEPNS_14InternalPacketE
// demangled: RakNet::ReliabilityLayer::SplitPacket(RakNet::InternalPacket *)
// type: int __fastcall(int, int)
#[doc(alias = "RakNet::ReliabilityLayer::SplitPacket(RakNet::InternalPacket *)")]
pub fn stub_a75100() -> ! {
    todo!("0xa75100 RakNet::ReliabilityLayer::SplitPacket(RakNet::InternalPacket *)")
}

// 0xa75548 — __ZN6RakNet16ReliabilityLayer6UpdateEiRNS_13SystemAddressEiyjRN14DataStructures4ListIPNS_16PluginInterface2EEEPNS_12RakNetRandomEtjRNS_9BitStreamE
// demangled: RakNet::ReliabilityLayer::Update(int,RakNet::SystemAddress &,int,unsigned long long,unsigned int,DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)
// type: bool __fastcall(int, RakNet::SocketLayer *, sockaddr *, int, unsigned __int64, int, _DWORD *, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, void **)
#[doc(alias = "RakNet::ReliabilityLayer::Update(int,RakNet::SystemAddress &,int,unsigned long long,unsigned int,DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")]
pub fn stub_a75548() -> ! {
    todo!("0xa75548 RakNet::ReliabilityLayer::Update(int,RakNet::SystemAddress &,int,unsigned long long,unsigned int,DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")
}

// 0xa7641c — __ZN6RakNet16ReliabilityLayer10AckTimeoutEy
// demangled: RakNet::ReliabilityLayer::AckTimeout(unsigned long long)
// type: int __fastcall(RakNet::ReliabilityLayer *this, unsigned __int64)
#[doc(alias = "RakNet::ReliabilityLayer::AckTimeout(unsigned long long)")]
pub fn stub_a7641c() -> ! {
    todo!("0xa7641c RakNet::ReliabilityLayer::AckTimeout(unsigned long long)")
}

// 0xa76468 — __ZN6RakNet16ReliabilityLayer8SendACKsEiRNS_13SystemAddressEyPNS_12RakNetRandomEtjRNS_9BitStreamE
// demangled: RakNet::ReliabilityLayer::SendACKs(int,RakNet::SystemAddress &,unsigned long long,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)
// type: int __fastcall(RakNet::ReliabilityLayer *this, RakNet::SocketLayer *, sockaddr *, unsigned __int64, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, void **)
#[doc(alias = "RakNet::ReliabilityLayer::SendACKs(int,RakNet::SystemAddress &,unsigned long long,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")]
pub fn stub_a76468() -> ! {
    todo!("0xa76468 RakNet::ReliabilityLayer::SendACKs(int,RakNet::SystemAddress &,unsigned long long,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")
}

// 0xa765e0 — __ZN6RakNet16ReliabilityLayer24ResetPacketsAndDatagramsEv
// demangled: RakNet::ReliabilityLayer::ResetPacketsAndDatagrams(void)
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ResetPacketsAndDatagrams(void)")]
pub fn stub_a765e0() -> ! {
    todo!("0xa765e0 RakNet::ReliabilityLayer::ResetPacketsAndDatagrams(void)")
}

// 0xa766b8 — __ZN6RakNet16ReliabilityLayer12PushDatagramEv
// demangled: RakNet::ReliabilityLayer::PushDatagram(void)
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::PushDatagram(void)")]
pub fn stub_a766b8() -> ! {
    todo!("0xa766b8 RakNet::ReliabilityLayer::PushDatagram(void)")
}

// 0xa76828 — __ZN6RakNet16ReliabilityLayer10PushPacketEyPNS_14InternalPacketEb
// demangled: RakNet::ReliabilityLayer::PushPacket(unsigned long long,RakNet::InternalPacket *,bool)
// type: void __fastcall(_DWORD *, unsigned __int64, int, char)
#[doc(alias = "RakNet::ReliabilityLayer::PushPacket(unsigned long long,RakNet::InternalPacket *,bool)")]
pub fn stub_a76828() -> ! {
    todo!("0xa76828 RakNet::ReliabilityLayer::PushPacket(unsigned long long,RakNet::InternalPacket *,bool)")
}

// 0xa7696c — __ZN6RakNet16ReliabilityLayer25AddFirstToDatagramHistoryENS_8uint24_tES1_y
// demangled: RakNet::ReliabilityLayer::AddFirstToDatagramHistory(RakNet::uint24_t,RakNet::uint24_t,unsigned long long)
// type: _DWORD *__fastcall(int, int, _DWORD *, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::AddFirstToDatagramHistory(RakNet::uint24_t,RakNet::uint24_t,unsigned long long)")]
pub fn stub_a7696c() -> ! {
    todo!("0xa7696c RakNet::ReliabilityLayer::AddFirstToDatagramHistory(RakNet::uint24_t,RakNet::uint24_t,unsigned long long)")
}

// 0xa76a68 — __ZN6RakNet16ReliabilityLayer34WriteToBitStreamFromInternalPacketEPNS_9BitStreamEPKNS_14InternalPacketEy
// demangled: RakNet::ReliabilityLayer::WriteToBitStreamFromInternalPacket(RakNet::BitStream *,RakNet::InternalPacket const*,unsigned long long)
// type: int __fastcall(int, RakNet::BitStream *this, int)
#[doc(alias = "RakNet::ReliabilityLayer::WriteToBitStreamFromInternalPacket(RakNet::BitStream *,RakNet::InternalPacket const*,unsigned long long)")]
pub fn stub_a76a68() -> ! {
    todo!("0xa76a68 RakNet::ReliabilityLayer::WriteToBitStreamFromInternalPacket(RakNet::BitStream *,RakNet::InternalPacket const*,unsigned long long)")
}

// 0xa76b88 — __ZN6RakNet16ReliabilityLayer25AddFirstToDatagramHistoryENS_8uint24_tEy
// demangled: RakNet::ReliabilityLayer::AddFirstToDatagramHistory(RakNet::uint24_t,unsigned long long)
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::AddFirstToDatagramHistory(RakNet::uint24_t,unsigned long long)")]
pub fn stub_a76b88() -> ! {
    todo!("0xa76b88 RakNet::ReliabilityLayer::AddFirstToDatagramHistory(RakNet::uint24_t,unsigned long long)")
}

// 0xa76c68 — __ZN6RakNet16ReliabilityLayer21IsOutgoingDataWaitingEv
// demangled: RakNet::ReliabilityLayer::IsOutgoingDataWaiting(void)
// type: bool __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::IsOutgoingDataWaiting(void)")]
pub fn stub_a76c68() -> ! {
    todo!("0xa76c68 RakNet::ReliabilityLayer::IsOutgoingDataWaiting(void)")
}

// 0xa76c84 — __ZN6RakNet16ReliabilityLayer14AreAcksWaitingEv
// demangled: RakNet::ReliabilityLayer::AreAcksWaiting(void)
// type: bool __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::AreAcksWaiting(void)")]
pub fn stub_a76c84() -> ! {
    todo!("0xa76c84 RakNet::ReliabilityLayer::AreAcksWaiting(void)")
}

// 0xa76c90 — __ZN6RakNet16ReliabilityLayer31SetSplitMessageProgressIntervalEi
// demangled: RakNet::ReliabilityLayer::SetSplitMessageProgressInterval(int)
// type: int __fastcall(int this, int)
#[doc(alias = "RakNet::ReliabilityLayer::SetSplitMessageProgressInterval(int)")]
pub fn stub_a76c90() -> ! {
    todo!("0xa76c90 RakNet::ReliabilityLayer::SetSplitMessageProgressInterval(int)")
}

// 0xa76c94 — __ZN6RakNet16ReliabilityLayer20SetUnreliableTimeoutEj
// demangled: RakNet::ReliabilityLayer::SetUnreliableTimeout(unsigned int)
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "RakNet::ReliabilityLayer::SetUnreliableTimeout(unsigned int)")]
pub fn stub_a76c94() -> ! {
    todo!("0xa76c94 RakNet::ReliabilityLayer::SetUnreliableTimeout(unsigned int)")
}

// 0xa76ca4 — __ZN6RakNet16ReliabilityLayer30BuildPacketFromSplitPacketListEPNS_18SplitPacketChannelEy
// demangled: RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList(RakNet::SplitPacketChannel *,unsigned long long)
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList(RakNet::SplitPacketChannel *,unsigned long long)")]
pub fn stub_a76ca4() -> ! {
    todo!("0xa76ca4 RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList(RakNet::SplitPacketChannel *,unsigned long long)")
}

// 0xa76e6c — __ZNK6RakNet16ReliabilityLayer16IsDeadConnectionEv
// demangled: RakNet::ReliabilityLayer::IsDeadConnection(void)const
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::IsDeadConnection(void)const")]
pub fn stub_a76e6c() -> ! {
    todo!("0xa76e6c RakNet::ReliabilityLayer::IsDeadConnection(void)const")
}

// 0xa76e74 — __ZN6RakNet16ReliabilityLayer13GetStatisticsEPNS_16RakNetStatisticsE
// demangled: RakNet::ReliabilityLayer::GetStatistics(RakNet::RakNetStatistics *)
// type: int __fastcall(int, int)
#[doc(alias = "RakNet::ReliabilityLayer::GetStatistics(RakNet::RakNetStatistics *)")]
pub fn stub_a76e74() -> ! {
    todo!("0xa76e74 RakNet::ReliabilityLayer::GetStatistics(RakNet::RakNetStatistics *)")
}

// 0xa77058 — __ZN20DatagramHeaderFormat11DeserializeEPN6RakNet9BitStreamE
// demangled: DatagramHeaderFormat::Deserialize(RakNet::BitStream *)
// type: _DWORD __fastcall(DatagramHeaderFormat *__hidden this, RakNet::BitStream *)
#[doc(alias = "DatagramHeaderFormat::Deserialize(RakNet::BitStream *)")]
pub fn stub_a77058() -> ! {
    todo!("0xa77058 DatagramHeaderFormat::Deserialize(RakNet::BitStream *)")
}

// 0xa771e8 — __ZN14DataStructures9RangeListIN6RakNet8uint24_tEE11DeserializeEPNS1_9BitStreamE
// demangled: DataStructures::RangeList<RakNet::uint24_t>::Deserialize(RakNet::BitStream *)
// type: int __fastcall(_DWORD *, RakNet::BitStream *, int, int)
#[doc(alias = "DataStructures::RangeList<RakNet::uint24_t>::Deserialize(RakNet::BitStream *)")]
pub fn stub_a771e8() -> ! {
    todo!("0xa771e8 DataStructures::RangeList<RakNet::uint24_t>::Deserialize(RakNet::BitStream *)")
}

// 0xa772b8 — __ZN14DataStructures5QueueIPN6RakNet14InternalPacketEE4PushERKS3_PKcj
// demangled: DataStructures::Queue<RakNet::InternalPacket *>::Push(RakNet::InternalPacket * const&,char const*,unsigned int)
// type: void __fastcall(int **, int *)
#[doc(alias = "DataStructures::Queue<RakNet::InternalPacket *>::Push(RakNet::InternalPacket * const&,char const*,unsigned int)")]
pub fn stub_a772b8() -> ! {
    todo!("0xa772b8 DataStructures::Queue<RakNet::InternalPacket *>::Push(RakNet::InternalPacket * const&,char const*,unsigned int)")
}

// 0xa7738c — __ZN14DataStructures9RangeListIN6RakNet8uint24_tEE6InsertES2_
// demangled: DataStructures::RangeList<RakNet::uint24_t>::Insert(RakNet::uint24_t)
// type: void __fastcall(int *, unsigned int *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "DataStructures::RangeList<RakNet::uint24_t>::Insert(RakNet::uint24_t)")]
pub fn stub_a7738c() -> ! {
    todo!("0xa7738c DataStructures::RangeList<RakNet::uint24_t>::Insert(RakNet::uint24_t)")
}

// 0xa77784 — __ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE3PopEj
// demangled: DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::Pop(unsigned int)
// type: int __fastcall(int *, unsigned int)
#[doc(alias = "DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::Pop(unsigned int)")]
pub fn stub_a77784() -> ! {
    todo!("0xa77784 DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::Pop(unsigned int)")
}

// 0xa77950 — __ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE4PushERKyRKS3_PKcj
// demangled: DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::Push(unsigned long long const&,RakNet::InternalPacket * const&,char const*,unsigned int)
// type: unsigned int __fastcall(char **, int *, int *)
#[doc(alias = "DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::Push(unsigned long long const&,RakNet::InternalPacket * const&,char const*,unsigned int)")]
pub fn stub_a77950() -> ! {
    todo!("0xa77950 DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::Push(unsigned long long const&,RakNet::InternalPacket * const&,char const*,unsigned int)")
}

// 0xa77a84 — __ZN20DatagramHeaderFormat9SerializeEPN6RakNet9BitStreamE
// demangled: DatagramHeaderFormat::Serialize(RakNet::BitStream *)
// type: int __fastcall(DatagramHeaderFormat *this, RakNet::BitStream *)
#[doc(alias = "DatagramHeaderFormat::Serialize(RakNet::BitStream *)")]
pub fn stub_a77a84() -> ! {
    todo!("0xa77a84 DatagramHeaderFormat::Serialize(RakNet::BitStream *)")
}

// 0xa77b3c — __ZN14DataStructures9RangeListIN6RakNet8uint24_tEE9SerializeEPNS1_9BitStreamEjb
// demangled: DataStructures::RangeList<RakNet::uint24_t>::Serialize(RakNet::BitStream *,unsigned int,bool)
// type: int __fastcall(int *, RakNet::BitStream *, unsigned int, int)
#[doc(alias = "DataStructures::RangeList<RakNet::uint24_t>::Serialize(RakNet::BitStream *,unsigned int,bool)")]
pub fn stub_a77b3c() -> ! {
    todo!("0xa77b3c DataStructures::RangeList<RakNet::uint24_t>::Serialize(RakNet::BitStream *,unsigned int,bool)")
}

// 0xa77d60 — __ZN6RakNet9BitStream5WriteINS_8uint24_tEEEvRKT_
// demangled: void RakNet::BitStream::Write<RakNet::uint24_t>(RakNet::uint24_t const&)
// type: void __fastcall(RakNet::BitStream *this, _BYTE *, int, int, int)
#[doc(alias = "void RakNet::BitStream::Write<RakNet::uint24_t>(RakNet::uint24_t const&)")]
pub fn stub_a77d60(stream: &mut crate::bitstream::BitStream, value: u32) {
    // IDA 0xa77d60: align-up, then the low 3 bytes in host order — no `ReverseBytes` on this path.
    stream.write_uint24(value);
}

// 0xa77ea4 — __ZN6RakNet9BitStream4ReadINS_8uint24_tEEEbRT_
// demangled: bool RakNet::BitStream::Read<RakNet::uint24_t>(RakNet::uint24_t &)
// type: int __fastcall(_DWORD *, _BYTE *)
#[doc(alias = "bool RakNet::BitStream::Read<RakNet::uint24_t>(RakNet::uint24_t &)")]
pub fn stub_a77ea4(stream: &mut crate::bitstream::BitStream) -> Option<u32> {
    // IDA 0xa77ea4: align-up (consumed even on failure), 24-bit bounds check, then 3 bytes little-endian.
    stream.read_uint24()
}

// 0xa77ff4 — __ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE10PushSeriesERKyRKS3_PKcj
// demangled: DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::PushSeries(unsigned long long const&,RakNet::InternalPacket * const&,char const*,unsigned int)
// type: unsigned int __fastcall(int, int *, int *)
#[doc(alias = "DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::PushSeries(unsigned long long const&,RakNet::InternalPacket * const&,char const*,unsigned int)")]
pub fn stub_a77ff4() -> ! {
    todo!("0xa77ff4 DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::PushSeries(unsigned long long const&,RakNet::InternalPacket * const&,char const*,unsigned int)")
}

// 0xa781a4 — __ZN14DataStructures11OrderedListItPN6RakNet18SplitPacketChannelEXadL_ZNS1_22SplitPacketChannelCompERKtRKS3_EEE6InsertES5_S7_bPKcjPFiS5_S7_E
// demangled: DataStructures::OrderedList<unsigned short,RakNet::SplitPacketChannel *,&RakNet::SplitPacketChannelComp>::Insert(unsigned short const&,RakNet::SplitPacketChannel * const&,bool,char const*,unsigned int,int (*)(unsigned short const&,RakNet::SplitPacketChannel * const&))
// type: unsigned int __fastcall(int **, int, int *, int, int, int, int (__fastcall *)(int, int))
#[doc(alias = "DataStructures::OrderedList<unsigned short,RakNet::SplitPacketChannel *,&RakNet::SplitPacketChannelComp>::Insert(unsigned short const&,RakNet::SplitPacketChannel * const&,bool,char const*,unsigned int,int (*)(unsigned short const&,RakNet::SplitPacketChannel * const&))")]
pub fn stub_a781a4() -> ! {
    todo!("0xa781a4 DataStructures::OrderedList<unsigned short,RakNet::SplitPacketChannel *,&RakNet::SplitPacketChannelComp>::Insert(unsigned short const&,RakNet::SplitPacketChannel * const&,bool,char const*,unsigned int,int (*)(unsigned short const&,RakNet::SplitPacketChannel * const&))")
}

// 0xa7828c — __ZN14DataStructures10MemoryPoolIN6RakNet14InternalPacketEE8AllocateEPKcj
// demangled: DataStructures::MemoryPool<RakNet::InternalPacket>::Allocate(char const*,unsigned int)
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacket>::Allocate(char const*,unsigned int)")]
pub fn stub_a7828c() -> ! {
    todo!("0xa7828c DataStructures::MemoryPool<RakNet::InternalPacket>::Allocate(char const*,unsigned int)")
}

// 0xa783b4 — __ZN14DataStructures10MemoryPoolIN6RakNet14InternalPacketEE7ReleaseEPS2_PKcj
// demangled: DataStructures::MemoryPool<RakNet::InternalPacket>::Release(RakNet::InternalPacket*,char const*,unsigned int)
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacket>::Release(RakNet::InternalPacket*,char const*,unsigned int)")]
pub fn stub_a783b4() -> ! {
    todo!("0xa783b4 DataStructures::MemoryPool<RakNet::InternalPacket>::Release(RakNet::InternalPacket*,char const*,unsigned int)")
}

// 0xa7848c — __ZN14DataStructures10MemoryPoolIN6RakNet16ReliabilityLayer17MessageNumberNodeEE7ReleaseEPS3_PKcj
// demangled: DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Release(RakNet::ReliabilityLayer::MessageNumberNode*,char const*,unsigned int)
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Release(RakNet::ReliabilityLayer::MessageNumberNode*,char const*,unsigned int)")]
pub fn stub_a7848c() -> ! {
    todo!("0xa7848c DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Release(RakNet::ReliabilityLayer::MessageNumberNode*,char const*,unsigned int)")
}

// 0xa78560 — __ZN14DataStructures5QueueIN6RakNet16ReliabilityLayer19DatagramHistoryNodeEE4PushERKS3_PKcj
// demangled: DataStructures::Queue<RakNet::ReliabilityLayer::DatagramHistoryNode>::Push(RakNet::ReliabilityLayer::DatagramHistoryNode const&,char const*,unsigned int)
// type: void __fastcall(_DWORD *, __int64 *)
#[doc(alias = "DataStructures::Queue<RakNet::ReliabilityLayer::DatagramHistoryNode>::Push(RakNet::ReliabilityLayer::DatagramHistoryNode const&,char const*,unsigned int)")]
pub fn stub_a78560() -> ! {
    todo!("0xa78560 DataStructures::Queue<RakNet::ReliabilityLayer::DatagramHistoryNode>::Push(RakNet::ReliabilityLayer::DatagramHistoryNode const&,char const*,unsigned int)")
}

// 0xa78670 — __ZN14DataStructures10MemoryPoolIN6RakNet16ReliabilityLayer17MessageNumberNodeEE8AllocateEPKcj
// demangled: DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Allocate(char const*,unsigned int)
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Allocate(char const*,unsigned int)")]
pub fn stub_a78670() -> ! {
    todo!("0xa78670 DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Allocate(char const*,unsigned int)")
}

// 0xa7879c — __ZN14DataStructures10MemoryPoolIN6RakNet28InternalPacketRefCountedDataEE8AllocateEPKcj
// demangled: DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Allocate(char const*,unsigned int)
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Allocate(char const*,unsigned int)")]
pub fn stub_a7879c() -> ! {
    todo!("0xa7879c DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Allocate(char const*,unsigned int)")
}

// 0xa788c8 — __ZN14DataStructures10MemoryPoolIN6RakNet28InternalPacketRefCountedDataEE7ReleaseEPS2_PKcj
// demangled: DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Release(RakNet::InternalPacketRefCountedData*,char const*,unsigned int)
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Release(RakNet::InternalPacketRefCountedData*,char const*,unsigned int)")]
pub fn stub_a788c8() -> ! {
    todo!("0xa788c8 DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Release(RakNet::InternalPacketRefCountedData*,char const*,unsigned int)")
}

// 0xa7899c — __ZN14DataStructures4ListIPN6RakNet18SplitPacketChannelEE6InsertERKS3_jPKcj
// demangled: DataStructures::List<RakNet::SplitPacketChannel *>::Insert(RakNet::SplitPacketChannel * const&,unsigned int,char const*,unsigned int)
// type: unsigned int __fastcall(int, _DWORD *, int)
#[doc(alias = "DataStructures::List<RakNet::SplitPacketChannel *>::Insert(RakNet::SplitPacketChannel * const&,unsigned int,char const*,unsigned int)")]
pub fn stub_a7899c() -> ! {
    todo!("0xa7899c DataStructures::List<RakNet::SplitPacketChannel *>::Insert(RakNet::SplitPacketChannel * const&,unsigned int,char const*,unsigned int)")
}

// 0xa78a2c — __ZN14DataStructures4ListINS_9RangeNodeIN6RakNet8uint24_tEEEE6InsertERKS4_jPKcj
// demangled: DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert(DataStructures::RangeNode<RakNet::uint24_t> const&,unsigned int,char const*,unsigned int)
// type: int __fastcall(_DWORD *, _DWORD *, int)
#[doc(alias = "DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert(DataStructures::RangeNode<RakNet::uint24_t> const&,unsigned int,char const*,unsigned int)")]
pub fn stub_a78a2c() -> ! {
    todo!("0xa78a2c DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert(DataStructures::RangeNode<RakNet::uint24_t> const&,unsigned int,char const*,unsigned int)")
}

// 0xa78b08 — __ZN14DataStructures4ListINS_9RangeNodeIN6RakNet8uint24_tEEEE6InsertERKS4_PKcj
// demangled: DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert(DataStructures::RangeNode<RakNet::uint24_t> const&,char const*,unsigned int)
// type: int __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert(DataStructures::RangeNode<RakNet::uint24_t> const&,char const*,unsigned int)")]
pub fn stub_a78b08() -> ! {
    todo!("0xa78b08 DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert(DataStructures::RangeNode<RakNet::uint24_t> const&,char const*,unsigned int)")
}

// 0xa78bbc — __ZN14DataStructures5QueueIN6RakNet10BPSTracker13TimeAndValue2EE4PushERKS3_PKcj
// demangled: DataStructures::Queue<RakNet::BPSTracker::TimeAndValue2>::Push(RakNet::BPSTracker::TimeAndValue2 const&,char const*,unsigned int)
// type: _QWORD *__fastcall(int *, _QWORD *)
#[doc(alias = "DataStructures::Queue<RakNet::BPSTracker::TimeAndValue2>::Push(RakNet::BPSTracker::TimeAndValue2 const&,char const*,unsigned int)")]
pub fn stub_a78bbc() -> ! {
    todo!("0xa78bbc DataStructures::Queue<RakNet::BPSTracker::TimeAndValue2>::Push(RakNet::BPSTracker::TimeAndValue2 const&,char const*,unsigned int)")
}

// 0xa79900 — __ZN6RakNet13SignaledEventC1Ev
// demangled: RakNet::SignaledEvent::SignaledEvent(void)
// type: RakNet::SignaledEvent *__fastcall(RakNet::SignaledEvent *this)
#[doc(alias = "RakNet::SignaledEvent::SignaledEvent(void)")]
pub fn stub_a79900() -> ! {
    todo!("0xa79900 RakNet::SignaledEvent::SignaledEvent(void)")
}

// 0xa79914 — __ZN6RakNet13SignaledEventD1Ev
// demangled: RakNet::SignaledEvent::~SignaledEvent()
// type: void __fastcall(RakNet::SignaledEvent *__hidden this)
#[doc(alias = "RakNet::SignaledEvent::~SignaledEvent()")]
pub fn stub_a79914() -> ! {
    todo!("0xa79914 RakNet::SignaledEvent::~SignaledEvent()")
}

// 0xa79924 — __ZN6RakNet13SignaledEvent9InitEventEv
// demangled: RakNet::SignaledEvent::InitEvent(void)
// type: int __fastcall(RakNet::SignaledEvent *this)
#[doc(alias = "RakNet::SignaledEvent::InitEvent(void)")]
pub fn stub_a79924() -> ! {
    todo!("0xa79924 RakNet::SignaledEvent::InitEvent(void)")
}

// 0xa79954 — __ZN6RakNet13SignaledEvent10CloseEventEv
// demangled: RakNet::SignaledEvent::CloseEvent(void)
// type: int __fastcall(RakNet::SignaledEvent *this)
#[doc(alias = "RakNet::SignaledEvent::CloseEvent(void)")]
pub fn stub_a79954() -> ! {
    todo!("0xa79954 RakNet::SignaledEvent::CloseEvent(void)")
}

// 0xa7997c — __ZN6RakNet13SignaledEvent8SetEventEv
// demangled: RakNet::SignaledEvent::SetEvent(void)
// type: int __fastcall(pthread_cond_t *this)
#[doc(alias = "RakNet::SignaledEvent::SetEvent(void)")]
pub fn stub_a7997c() -> ! {
    todo!("0xa7997c RakNet::SignaledEvent::SetEvent(void)")
}

// 0xa7999c — __ZN6RakNet13SignaledEvent11WaitOnEventEi
// demangled: RakNet::SignaledEvent::WaitOnEvent(int)
// type: int __fastcall(RakNet::SignaledEvent *this, int)
#[doc(alias = "RakNet::SignaledEvent::WaitOnEvent(int)")]
pub fn stub_a7999c() -> ! {
    todo!("0xa7999c RakNet::SignaledEvent::WaitOnEvent(int)")
}

// 0xa7a0b4 — __ZN6RakNet11SimpleMutexC1Ev
// demangled: RakNet::SimpleMutex::SimpleMutex(void)
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *this)
#[doc(alias = "RakNet::SimpleMutex::SimpleMutex(void)")]
pub fn stub_a7a0b4() -> ! {
    todo!("0xa7a0b4 RakNet::SimpleMutex::SimpleMutex(void)")
}

// 0xa7a0c4 — __ZN6RakNet11SimpleMutexD1Ev
// demangled: RakNet::SimpleMutex::~SimpleMutex()
// type: void __fastcall(pthread_mutex_t *this)
#[doc(alias = "RakNet::SimpleMutex::~SimpleMutex()")]
pub fn stub_a7a0c4() -> ! {
    todo!("0xa7a0c4 RakNet::SimpleMutex::~SimpleMutex()")
}

// 0xa7a0d4 — __ZN6RakNet11SimpleMutex4LockEv
// demangled: RakNet::SimpleMutex::Lock(void)
// type: int __fastcall(pthread_mutex_t *this)
#[doc(alias = "RakNet::SimpleMutex::Lock(void)")]
pub fn stub_a7a0d4() -> ! {
    todo!("0xa7a0d4 RakNet::SimpleMutex::Lock(void)")
}

// 0xa7a0e0 — __ZN6RakNet11SimpleMutex6UnlockEv
// demangled: RakNet::SimpleMutex::Unlock(void)
// type: int __fastcall(pthread_mutex_t *this)
#[doc(alias = "RakNet::SimpleMutex::Unlock(void)")]
pub fn stub_a7a0e0() -> ! {
    todo!("0xa7a0e0 RakNet::SimpleMutex::Unlock(void)")
}

// 0xa7a700 — __ZN6RakNet11SocketLayer11IsPortInUseEtPKct
// demangled: RakNet::SocketLayer::IsPortInUse(unsigned short,char const*,unsigned short)
// type: int __fastcall(RakNet::SocketLayer *this, const char *, const char *, unsigned __int16)
#[doc(alias = "RakNet::SocketLayer::IsPortInUse(unsigned short,char const*,unsigned short)")]
pub fn stub_a7a700() -> ! {
    todo!("0xa7a700 RakNet::SocketLayer::IsPortInUse(unsigned short,char const*,unsigned short)")
}

// 0xa7a788 — __ZN6RakNet11SocketLayer16SetDoNotFragmentEiii
// demangled: RakNet::SocketLayer::SetDoNotFragment(int,int,int)
// type: void __fastcall(RakNet::SocketLayer *this, int, int, int)
#[doc(alias = "RakNet::SocketLayer::SetDoNotFragment(int,int,int)")]
pub fn stub_a7a788() -> ! {
    todo!("0xa7a788 RakNet::SocketLayer::SetDoNotFragment(int,int,int)")
}

// 0xa7a78c — __ZN6RakNet11SocketLayer21CreateBoundSocket_OldEtbPKcjj
// demangled: RakNet::SocketLayer::CreateBoundSocket_Old(unsigned short,bool,char const*,unsigned int,unsigned int)
// type: int __fastcall(RakNet::SocketLayer *this, unsigned __int16, const char *, const char *, unsigned int, unsigned int)
#[doc(alias = "RakNet::SocketLayer::CreateBoundSocket_Old(unsigned short,bool,char const*,unsigned int,unsigned int)")]
pub fn stub_a7a78c() -> ! {
    todo!("0xa7a78c RakNet::SocketLayer::CreateBoundSocket_Old(unsigned short,bool,char const*,unsigned int,unsigned int)")
}

// 0xa7a898 — __ZN6RakNet11SocketLayer17CreateBoundSocketEtbPKcjjt
// demangled: RakNet::SocketLayer::CreateBoundSocket(unsigned short,bool,char const*,unsigned int,unsigned int,unsigned short)
// type: int __fastcall(RakNet::SocketLayer *this, unsigned __int16, const char *, const char *, unsigned int, unsigned int, unsigned __int16)
#[doc(alias = "RakNet::SocketLayer::CreateBoundSocket(unsigned short,bool,char const*,unsigned int,unsigned int,unsigned short)")]
pub fn stub_a7a898() -> ! {
    todo!("0xa7a898 RakNet::SocketLayer::CreateBoundSocket(unsigned short,bool,char const*,unsigned int,unsigned int,unsigned short)")
}

// 0xa7a8ac — __ZN6RakNet11SocketLayer14DomainNameToIPEPKc
// demangled: RakNet::SocketLayer::DomainNameToIP(char const*)
// type: char *__fastcall(RakNet::SocketLayer *this, const char *)
#[doc(alias = "RakNet::SocketLayer::DomainNameToIP(char const*)")]
pub fn stub_a7a8ac() -> ! {
    todo!("0xa7a8ac RakNet::SocketLayer::DomainNameToIP(char const*)")
}

// 0xa7a8d0 — __ZN6RakNet11SocketLayer16RecvFromBlockingEiPNS_7RakPeerEtjPcPiPNS_13SystemAddressEPy
// demangled: RakNet::SocketLayer::RecvFromBlocking(int,RakNet::RakPeer *,unsigned short,unsigned int,char *,int *,RakNet::SystemAddress *,unsigned long long *)
// type: int __fastcall(RakNet::SocketLayer *this, int, RakNet::RakPeer *, unsigned __int16, void *, char *, int *, RakNet::SystemAddress *, unsigned __int64 *)
#[doc(alias = "RakNet::SocketLayer::RecvFromBlocking(int,RakNet::RakPeer *,unsigned short,unsigned int,char *,int *,RakNet::SystemAddress *,unsigned long long *)")]
pub fn stub_a7a8d0() -> ! {
    todo!("0xa7a8d0 RakNet::SocketLayer::RecvFromBlocking(int,RakNet::RakPeer *,unsigned short,unsigned int,char *,int *,RakNet::SystemAddress *,unsigned long long *)")
}

// 0xa7a944 — __ZN6RakNet11SocketLayer6SendToEiPKciRNS_13SystemAddressEtjS2_l
// demangled: RakNet::SocketLayer::SendTo(int,char const*,int,RakNet::SystemAddress &,unsigned short,unsigned int,char const*,long)
// type: int __fastcall(RakNet::SocketLayer *this, char *, size_t, sockaddr *, RakNet::SystemAddress *, unsigned __int16, unsigned int, const char *, int)
#[doc(alias = "RakNet::SocketLayer::SendTo(int,char const*,int,RakNet::SystemAddress &,unsigned short,unsigned int,char const*,long)")]
pub fn stub_a7a944() -> ! {
    todo!("0xa7a944 RakNet::SocketLayer::SendTo(int,char const*,int,RakNet::SystemAddress &,unsigned short,unsigned int,char const*,long)")
}

// 0xa7a9ec — __ZN6RakNet11SocketLayer9SendToTTLEiPKciRNS_13SystemAddressEi
// demangled: RakNet::SocketLayer::SendToTTL(int,char const*,int,RakNet::SystemAddress &,int)
// type: int __fastcall(RakNet::SocketLayer *this, char *, const char *, RakNet::SystemAddress *, RakNet::SystemAddress *, int)
#[doc(alias = "RakNet::SocketLayer::SendToTTL(int,char const*,int,RakNet::SystemAddress &,int)")]
pub fn stub_a7a9ec() -> ! {
    todo!("0xa7a9ec RakNet::SocketLayer::SendToTTL(int,char const*,int,RakNet::SystemAddress &,int)")
}

// 0xa7aae0 — __Z13GetMyIP_LinuxPN6RakNet13SystemAddressE
// demangled: GetMyIP_Linux(RakNet::SystemAddress *)
// type: int __fastcall(in_addr *)
#[doc(alias = "GetMyIP_Linux(RakNet::SystemAddress *)")]
pub fn stub_a7aae0() -> ! {
    todo!("0xa7aae0 GetMyIP_Linux(RakNet::SystemAddress *)")
}

// 0xa7abd8 — __ZN6RakNet11SocketLayer7GetMyIPEPNS_13SystemAddressE
// demangled: RakNet::SocketLayer::GetMyIP(RakNet::SystemAddress *)
// type: int __fastcall(in_addr *this, RakNet::SystemAddress *)
#[doc(alias = "RakNet::SocketLayer::GetMyIP(RakNet::SystemAddress *)")]
pub fn stub_a7abd8() -> ! {
    todo!("0xa7abd8 RakNet::SocketLayer::GetMyIP(RakNet::SystemAddress *)")
}

// 0xa7abe4 — __ZN6RakNet11SocketLayer16GetSystemAddressEiPNS_13SystemAddressE
// demangled: RakNet::SocketLayer::GetSystemAddress(int,RakNet::SystemAddress *)
// type: int __fastcall(RakNet::SocketLayer *this, int, RakNet::SystemAddress *)
#[doc(alias = "RakNet::SocketLayer::GetSystemAddress(int,RakNet::SystemAddress *)")]
pub fn stub_a7abe4() -> ! {
    todo!("0xa7abe4 RakNet::SocketLayer::GetSystemAddress(int,RakNet::SystemAddress *)")
}

// 0xa7b268 — __ZN6RakNet16StringCompressor12AddReferenceEv
// demangled: RakNet::StringCompressor::AddReference(void)
// type: void __fastcall(RakNet::StringCompressor *this)
#[doc(alias = "RakNet::StringCompressor::AddReference(void)")]
pub fn stub_a7b268() -> ! {
    todo!("0xa7b268 RakNet::StringCompressor::AddReference(void)")
}

// 0xa7b39c — __ZN6RakNet16StringCompressor15RemoveReferenceEv
// demangled: RakNet::StringCompressor::RemoveReference(void)
// type: void __fastcall(RakNet::StringCompressor *this)
#[doc(alias = "RakNet::StringCompressor::RemoveReference(void)")]
pub fn stub_a7b39c() -> ! {
    todo!("0xa7b39c RakNet::StringCompressor::RemoveReference(void)")
}

// 0xa7b470 — __ZN6RakNet16StringCompressor8InstanceEv
// demangled: RakNet::StringCompressor::Instance(void)
// type: int __fastcall(RakNet::StringCompressor *this)
#[doc(alias = "RakNet::StringCompressor::Instance(void)")]
pub fn stub_a7b470() -> ! {
    todo!("0xa7b470 RakNet::StringCompressor::Instance(void)")
}

// 0xa7b480 — __ZN6RakNet16StringCompressorD2Ev
// demangled: RakNet::StringCompressor::~StringCompressor()
// type: void __fastcall(RakNet::StringCompressor *__hidden this)
#[doc(alias = "RakNet::StringCompressor::~StringCompressor()")]
pub fn stub_a7b480() -> ! {
    todo!("0xa7b480 RakNet::StringCompressor::~StringCompressor()")
}

// 0xa7b594 — __ZN6RakNet16StringCompressor12EncodeStringEPKciPNS_9BitStreamEh
// demangled: RakNet::StringCompressor::EncodeString(char const*,int,RakNet::BitStream *,unsigned char)
// type: int __fastcall(RakNet::StringCompressor *this, char *, int, struct _Unwind_Exception *, int)
#[doc(alias = "RakNet::StringCompressor::EncodeString(char const*,int,RakNet::BitStream *,unsigned char)")]
pub fn stub_a7b594() -> ! {
    todo!("0xa7b594 RakNet::StringCompressor::EncodeString(char const*,int,RakNet::BitStream *,unsigned char)")
}

// 0xa7b764 — __ZN6RakNet16StringCompressor12DecodeStringEPciPNS_9BitStreamEh
// demangled: RakNet::StringCompressor::DecodeString(char *,int,RakNet::BitStream *,unsigned char)
// type: int __fastcall(RakNet::StringCompressor *this, char *, int, RakNet::BitStream *, int)
#[doc(alias = "RakNet::StringCompressor::DecodeString(char *,int,RakNet::BitStream *,unsigned char)")]
pub fn stub_a7b764() -> ! {
    todo!("0xa7b764 RakNet::StringCompressor::DecodeString(char *,int,RakNet::BitStream *,unsigned char)")
}

// 0xa7b854 — __ZN14DataStructures3MapIiPN6RakNet19HuffmanEncodingTreeEXadL_ZNS_23defaultMapKeyComparisonIiEEiRKT_S7_EEE3SetERKiRKS3_
// demangled: DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::Set(int const&,RakNet::HuffmanEncodingTree * const&)
// type: int __fastcall(_DWORD *, int *, int *)
#[doc(alias = "DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::Set(int const&,RakNet::HuffmanEncodingTree * const&)")]
pub fn stub_a7b854() -> ! {
    todo!("0xa7b854 DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::Set(int const&,RakNet::HuffmanEncodingTree * const&)")
}

// 0xa7b9b4 — __ZN6RakNet9BitStream15WriteCompressedIjEEvRKT_
// demangled: void RakNet::BitStream::WriteCompressed<unsigned int>(unsigned int const&)
// type: void __fastcall(RakNet::BitStream *, unsigned __int8 *, int, unsigned int, __guard *, int, int, int, int)
#[doc(alias = "void RakNet::BitStream::WriteCompressed<unsigned int>(unsigned int const&)")]
pub fn stub_a7b9b4(stream: &mut crate::bitstream::BitStream, value: u32) {
    // IDA 0xa7b9b4: `ReverseBytes` to big-endian, then `WriteCompressed(..., 32, 1)` (IDA 0xa55c9c).
    stream.write_compressed_u32(value);
}

// 0xa7bac8 — __ZN6RakNet9BitStream14ReadCompressedIjEEbRT_
// demangled: bool RakNet::BitStream::ReadCompressed<unsigned int>(unsigned int &)
// type: int __fastcall(RakNet::BitStream *, unsigned __int8 *, int, int, __guard *, int, int, int, int)
#[doc(alias = "bool RakNet::BitStream::ReadCompressed<unsigned int>(unsigned int &)")]
pub fn stub_a7bac8(stream: &mut crate::bitstream::BitStream) -> Option<u32> {
    // IDA 0xa7bac8: `ReadCompressed(..., 32, 1)` (IDA 0xa55d2c), then `ReverseBytes` back to host order.
    stream.read_compressed_u32()
}

// 0xa7bbf0 — __ZN14DataStructures4ListINS_3MapIiPN6RakNet19HuffmanEncodingTreeEXadL_ZNS_23defaultMapKeyComparisonIiEEiRKT_S8_EEE7MapNodeEE6InsertERKSA_jPKcj
// demangled: DataStructures::List<DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::MapNode>::Insert(DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::MapNode const&,unsigned int,char const*,unsigned int)
// type: int __fastcall(char **, _DWORD *, char *)
#[doc(alias = "DataStructures::List<DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::MapNode>::Insert(DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::MapNode const&,unsigned int,char const*,unsigned int)")]
pub fn stub_a7bbf0() -> ! {
    todo!("0xa7bbf0 DataStructures::List<DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::MapNode>::Insert(DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::MapNode const&,unsigned int,char const*,unsigned int)")
}

// 0xa7c2c4 — __ZN6RakNet11StringTableD2Ev
// demangled: RakNet::StringTable::~StringTable()
// type: void __fastcall(RakNet::StringTable *__hidden this)
#[doc(alias = "RakNet::StringTable::~StringTable()")]
pub fn stub_a7c2c4() -> ! {
    todo!("0xa7c2c4 RakNet::StringTable::~StringTable()")
}

// 0xa7c3dc — __ZN6RakNet11StringTable12AddReferenceEv
// demangled: RakNet::StringTable::AddReference(void)
// type: int *__fastcall(RakNet::StringTable *this)
#[doc(alias = "RakNet::StringTable::AddReference(void)")]
pub fn stub_a7c3dc() -> ! {
    todo!("0xa7c3dc RakNet::StringTable::AddReference(void)")
}

// 0xa7c414 — __ZN6RakNet11StringTable15RemoveReferenceEv
// demangled: RakNet::StringTable::RemoveReference(void)
// type: void __fastcall(RakNet::StringTable *this)
#[doc(alias = "RakNet::StringTable::RemoveReference(void)")]
pub fn stub_a7c414() -> ! {
    todo!("0xa7c414 RakNet::StringTable::RemoveReference(void)")
}

// 0xa7d1d8 — __ZN14DataStructures5QueueIPN6RakNet6PacketEE4PushERKS3_PKcj
// demangled: DataStructures::Queue<RakNet::Packet *>::Push(RakNet::Packet * const&,char const*,unsigned int)
// type: void __fastcall(int **, int *)
#[doc(alias = "DataStructures::Queue<RakNet::Packet *>::Push(RakNet::Packet * const&,char const*,unsigned int)")]
pub fn stub_a7d1d8() -> ! {
    todo!("0xa7d1d8 DataStructures::Queue<RakNet::Packet *>::Push(RakNet::Packet * const&,char const*,unsigned int)")
}

// 0xa7d2ac — __ZN14DataStructures10MemoryPoolIN6RakNet6PacketEE8AllocateEPKcj
// demangled: DataStructures::MemoryPool<RakNet::Packet>::Allocate(char const*,unsigned int)
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::Packet>::Allocate(char const*,unsigned int)")]
pub fn stub_a7d2ac() -> ! {
    todo!("0xa7d2ac DataStructures::MemoryPool<RakNet::Packet>::Allocate(char const*,unsigned int)")
}

// 0xa7d3d8 — __ZN14DataStructures10MemoryPoolIN6RakNet6PacketEE7ReleaseEPS2_PKcj
// demangled: DataStructures::MemoryPool<RakNet::Packet>::Release(RakNet::Packet*,char const*,unsigned int)
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::Packet>::Release(RakNet::Packet*,char const*,unsigned int)")]
pub fn stub_a7d3d8() -> ! {
    todo!("0xa7d3d8 DataStructures::MemoryPool<RakNet::Packet>::Release(RakNet::Packet*,char const*,unsigned int)")
}

// 0xa7e0e8 — __ZN3RBX7Network23RoundRobinPhysicsSenderC1ERNS0_10ReplicatorE
// demangled: RBX::Network::RoundRobinPhysicsSender::RoundRobinPhysicsSender(RBX::Network::Replicator &)
// type: RBX::Network::RoundRobinPhysicsSender *__fastcall(RBX::Network::RoundRobinPhysicsSender *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::RoundRobinPhysicsSender(RBX::Network::Replicator &)")]
pub fn stub_a7e0e8() -> ! {
    todo!("0xa7e0e8 RBX::Network::RoundRobinPhysicsSender::RoundRobinPhysicsSender(RBX::Network::Replicator &)")
}

// 0xa7e1d0 — __ZN3RBX7Network23RoundRobinPhysicsSender15sendPhysicsDataERN6RakNet9BitStreamEPKNS_8AssemblyE
// demangled: RBX::Network::RoundRobinPhysicsSender::sendPhysicsData(RakNet::BitStream &,RBX::Assembly const*)
// type: int __fastcall(RBX::Network::IdSerializer **this, RakNet::BitStream *, const RBX::Assembly *)
#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::sendPhysicsData(RakNet::BitStream &,RBX::Assembly const*)")]
pub fn stub_a7e1d0() -> ! {
    todo!("0xa7e1d0 RBX::Network::RoundRobinPhysicsSender::sendPhysicsData(RakNet::BitStream &,RBX::Assembly const*)")
}

// 0xa7e360 — __ZN3RBX7Network23RoundRobinPhysicsSender4stepEv
// demangled: RBX::Network::RoundRobinPhysicsSender::step(void)
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender *this, int, int, int)
#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::step(void)")]
pub fn stub_a7e360() -> ! {
    todo!("0xa7e360 RBX::Network::RoundRobinPhysicsSender::step(void)")
}

// 0xa7e468 — __ZN3RBX7Network23RoundRobinPhysicsSender10sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE
// demangled: RBX::Network::RoundRobinPhysicsSender::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)")]
pub fn stub_a7e468() -> ! {
    todo!("0xa7e468 RBX::Network::RoundRobinPhysicsSender::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)")
}

// 0xa7e9cc — __ZN3RBX11SendPhysics13reportSimJobsINS_7Network23RoundRobinPhysicsSender9JobSenderEEEiRT_RNS_13SimJobTrackerEPKNS_6SimJobEi
// demangled: int RBX::SendPhysics::reportSimJobs<RBX::Network::RoundRobinPhysicsSender::JobSender>(RBX::Network::RoundRobinPhysicsSender::JobSender &,RBX::SimJobTracker &,RBX::SimJob const*,int)
// type: int __fastcall(int, _DWORD *, RBX::SimJobTracker *, RBX::SimJob *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "int RBX::SendPhysics::reportSimJobs<RBX::Network::RoundRobinPhysicsSender::JobSender>(RBX::Network::RoundRobinPhysicsSender::JobSender &,RBX::SimJobTracker &,RBX::SimJob const*,int)")]
pub fn stub_a7e9cc() -> ! {
    todo!("0xa7e9cc int RBX::SendPhysics::reportSimJobs<RBX::Network::RoundRobinPhysicsSender::JobSender>(RBX::Network::RoundRobinPhysicsSender::JobSender &,RBX::SimJobTracker &,RBX::SimJob const*,int)")
}

// 0xa7ec08 — __ZN3RBX7Network23RoundRobinPhysicsSenderD1Ev
// demangled: RBX::Network::RoundRobinPhysicsSender::~RoundRobinPhysicsSender()
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::~RoundRobinPhysicsSender()")]
pub fn stub_a7ec08() -> ! {
    todo!("0xa7ec08 RBX::Network::RoundRobinPhysicsSender::~RoundRobinPhysicsSender()")
}

// 0xa7ecd4 — __ZN3RBX7Network23RoundRobinPhysicsSenderD0Ev
// demangled: RBX::Network::RoundRobinPhysicsSender::~RoundRobinPhysicsSender()
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::~RoundRobinPhysicsSender()")]
pub fn stub_a7ecd4() -> ! {
    todo!("0xa7ecd4 RBX::Network::RoundRobinPhysicsSender::~RoundRobinPhysicsSender()")
}

// 0xa7ef60 — __ZN3RBX7Network23RoundRobinPhysicsSender9JobSender11closePacketEv
// demangled: RBX::Network::RoundRobinPhysicsSender::JobSender::closePacket(void)
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender::JobSender *this)
#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::JobSender::closePacket(void)")]
pub fn stub_a7ef60() -> ! {
    todo!("0xa7ef60 RBX::Network::RoundRobinPhysicsSender::JobSender::closePacket(void)")
}

// 0xa7f320 — __ZN3RBX7Network23RoundRobinPhysicsSender9JobSender10openPacketEv
// demangled: RBX::Network::RoundRobinPhysicsSender::JobSender::openPacket(void)
// type: void __fastcall(RakNet **this)
#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::JobSender::openPacket(void)")]
pub fn stub_a7f320() -> ! {
    todo!("0xa7f320 RBX::Network::RoundRobinPhysicsSender::JobSender::openPacket(void)")
}

// 0xa7fbf0 — __ZN3RBX7Network6Player8loadDataEv
// demangled: RBX::Network::Player::loadData(void)
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Player::loadData(void)")]
pub fn stub_a7fbf0() -> ! {
    todo!("0xa7fbf0 RBX::Network::Player::loadData(void)")
}

// 0xa802c8 — __ZN3RBX7Network6Player8saveDataEv
// demangled: RBX::Network::Player::saveData(void)
// type: void __fastcall(RBX::Network::PersistentDataStore **this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Player::saveData(void)")]
pub fn stub_a802c8() -> ! {
    todo!("0xa802c8 RBX::Network::Player::saveData(void)")
}

// 0xa80674 — __ZN3RBX7Network6Player19saveLeaderboardDataEv
// demangled: RBX::Network::Player::saveLeaderboardData(void)
// type: void __fastcall(RBX::Network::PersistentDataStore **this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Player::saveLeaderboardData(void)")]
pub fn stub_a80674() -> ! {
    todo!("0xa80674 RBX::Network::Player::saveLeaderboardData(void)")
}

// 0xa80a28 — __ZN3RBX7Network6Player21setHasGroupBuildToolsEb
// demangled: RBX::Network::Player::setHasGroupBuildTools(bool)
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "RBX::Network::Player::setHasGroupBuildTools(bool)")]
pub fn stub_a80a28() -> ! {
    todo!("0xa80a28 RBX::Network::Player::setHasGroupBuildTools(bool)")
}

// 0xa80a50 — __ZN3RBX7Network6Player21setPersonalServerRankEi
// demangled: RBX::Network::Player::setPersonalServerRank(int)
// type: _DWORD __fastcall(RBX::Network::Player *__hidden this, int)
#[doc(alias = "RBX::Network::Player::setPersonalServerRank(int)")]
pub fn stub_a80a50() -> ! {
    todo!("0xa80a50 RBX::Network::Player::setPersonalServerRank(int)")
}

// 0xa80adc — __ZN3RBX7Network6Player24getWebPersonalServerRankEN5boost8functionIFvSsEEES5_
// demangled: RBX::Network::Player::getWebPersonalServerRank(boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
// type: void __fastcall(RBX::ServiceProvider *, int *, int *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::Player::getWebPersonalServerRank(boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_a80adc() -> ! {
    todo!("0xa80adc RBX::Network::Player::getWebPersonalServerRank(boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0xa80ed4 — __ZNK3RBX7Network6Player17getDataComplexityEv
// demangled: RBX::Network::Player::getDataComplexity(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getDataComplexity(void)const")]
pub fn stub_a80ed4() -> ! {
    todo!("0xa80ed4 RBX::Network::Player::getDataComplexity(void)const")
}

// 0xa80ee4 — __ZN3RBX7Network6Player22setDataComplexityLimitEi
// demangled: RBX::Network::Player::setDataComplexityLimit(int)
// type: int __fastcall(RBX::Network::Player *this, int)
#[doc(alias = "RBX::Network::Player::setDataComplexityLimit(int)")]
pub fn stub_a80ee4() -> ! {
    todo!("0xa80ee4 RBX::Network::Player::setDataComplexityLimit(int)")
}

// 0xa80f18 — __ZN3RBX7Network6Player17requestFriendshipEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Network::Player::requestFriendship(boost::shared_ptr<RBX::Instance>)
// type: void __fastcall(int, int, int, int)
#[doc(alias = "RBX::Network::Player::requestFriendship(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_a80f18() -> ! {
    todo!("0xa80f18 RBX::Network::Player::requestFriendship(boost::shared_ptr<RBX::Instance>)")
}

// 0xa81364 — __ZN3RBX7Network6Player16revokeFriendshipEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Network::Player::revokeFriendship(boost::shared_ptr<RBX::Instance>)
// type: void __fastcall(int, int, int, int)
#[doc(alias = "RBX::Network::Player::revokeFriendship(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_a81364() -> ! {
    todo!("0xa81364 RBX::Network::Player::revokeFriendship(boost::shared_ptr<RBX::Instance>)")
}

// 0xa817b0 — __ZN3RBX7Network6Player16getMouseInstanceEv
// demangled: RBX::Network::Player::getMouseInstance(void)
// type: void __fastcall(RBX::Network::Player *this, RBX::Network::Players *)
#[doc(alias = "RBX::Network::Player::getMouseInstance(void)")]
pub fn stub_a817b0() -> ! {
    todo!("0xa817b0 RBX::Network::Player::getMouseInstance(void)")
}

// 0xa81da0 — __ZN3RBX7Network6Player10loadStringESs
// demangled: RBX::Network::Player::loadString(std::string)
// type: void __fastcall(RBX::Network::PersistentDataStore *, int, const void **)
#[doc(alias = "RBX::Network::Player::loadString(std::string)")]
pub fn stub_a81da0() -> ! {
    todo!("0xa81da0 RBX::Network::Player::loadString(std::string)")
}

// 0xa82018 — __ZN3RBX7Network6Player10saveStringESsSs
// demangled: RBX::Network::Player::saveString(std::string,std::string)
// type: void __fastcall(int, const std::string *, const std::string *)
#[doc(alias = "RBX::Network::Player::saveString(std::string,std::string)")]
pub fn stub_a82018() -> ! {
    todo!("0xa82018 RBX::Network::Player::saveString(std::string,std::string)")
}

// 0xa82300 — __ZN3RBX7Network6Player11loadBooleanESs
// demangled: RBX::Network::Player::loadBoolean(std::string)
// type: int __fastcall(int, const void **, bool)
#[doc(alias = "RBX::Network::Player::loadBoolean(std::string)")]
pub fn stub_a82300() -> ! {
    todo!("0xa82300 RBX::Network::Player::loadBoolean(std::string)")
}

// 0xa82574 — __ZN3RBX7Network6Player11saveBooleanESsb
// demangled: RBX::Network::Player::saveBoolean(std::string,bool)
// type: void __fastcall(int, const std::string *, int)
#[doc(alias = "RBX::Network::Player::saveBoolean(std::string,bool)")]
pub fn stub_a82574() -> ! {
    todo!("0xa82574 RBX::Network::Player::saveBoolean(std::string,bool)")
}

// 0xa8285c — __ZN3RBX7Network6Player10loadNumberESs
// demangled: RBX::Network::Player::loadNumber(std::string)
// type: __int64 __fastcall(int, const void **, bool)
#[doc(alias = "RBX::Network::Player::loadNumber(std::string)")]
pub fn stub_a8285c() -> ! {
    todo!("0xa8285c RBX::Network::Player::loadNumber(std::string)")
}

// 0xa82ad8 — __ZN3RBX7Network6Player10saveNumberESsd
// demangled: RBX::Network::Player::saveNumber(std::string,double)
// type: void __fastcall(int, const std::string *, _BOOL4, unsigned int)
#[doc(alias = "RBX::Network::Player::saveNumber(std::string,double)")]
pub fn stub_a82ad8() -> ! {
    todo!("0xa82ad8 RBX::Network::Player::saveNumber(std::string,double)")
}

// 0xa82dc8 — __ZN3RBX7Network6Player12loadInstanceESs
// demangled: RBX::Network::Player::loadInstance(std::string)
// type: void __fastcall(RBX::Network::PersistentDataStore *, int, const void **)
#[doc(alias = "RBX::Network::Player::loadInstance(std::string)")]
pub fn stub_a82dc8() -> ! {
    todo!("0xa82dc8 RBX::Network::Player::loadInstance(std::string)")
}

// 0xa83044 — __ZN3RBX7Network6Player12saveInstanceESsN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Network::Player::saveInstance(std::string,boost::shared_ptr<RBX::Instance>)
// type: void __fastcall(int, const std::string *, int *)
#[doc(alias = "RBX::Network::Player::saveInstance(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_a83044() -> ! {
    todo!("0xa83044 RBX::Network::Player::saveInstance(std::string,boost::shared_ptr<RBX::Instance>)")
}

// 0xa835ec — __ZN3RBX7Network6Player16luaLoadCharacterEb
// demangled: RBX::Network::Player::luaLoadCharacter(bool)
// type: void __fastcall(RBX::Network::Player *this, const char *, int, const void *)
#[doc(alias = "RBX::Network::Player::luaLoadCharacter(bool)")]
pub fn stub_a835ec() -> ! {
    todo!("0xa835ec RBX::Network::Player::luaLoadCharacter(bool)")
}

// 0xa837d8 — __ZN3RBX7Network6Player15removeCharacterEv
// demangled: RBX::Network::Player::removeCharacter(void)
// type: void __fastcall(RBX::Network::Player *this, int, bool)
#[doc(alias = "RBX::Network::Player::removeCharacter(void)")]
pub fn stub_a837d8() -> ! {
    todo!("0xa837d8 RBX::Network::Player::removeCharacter(void)")
}

// 0xa83950 — __ZN3RBX7Network6Player10setUnder13Eb
// demangled: RBX::Network::Player::setUnder13(bool)
// type: int __fastcall(int this, int)
#[doc(alias = "RBX::Network::Player::setUnder13(bool)")]
pub fn stub_a83950() -> ! {
    todo!("0xa83950 RBX::Network::Player::setUnder13(bool)")
}

// 0xa83960 — __ZN3RBX7Network6Player16setSuperSafeChatEb
// demangled: RBX::Network::Player::setSuperSafeChat(bool)
// type: int __fastcall(RBX::Network::Player *this, int)
#[doc(alias = "RBX::Network::Player::setSuperSafeChat(bool)")]
pub fn stub_a83960() -> ! {
    todo!("0xa83960 RBX::Network::Player::setSuperSafeChat(bool)")
}

// 0xa83998 — __ZN3RBX7Network6Player17setMembershipTypeENS1_14MembershipTypeE
// demangled: RBX::Network::Player::setMembershipType(RBX::Network::Player::MembershipType)
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "RBX::Network::Player::setMembershipType(RBX::Network::Player::MembershipType)")]
pub fn stub_a83998() -> ! {
    todo!("0xa83998 RBX::Network::Player::setMembershipType(RBX::Network::Player::MembershipType)")
}

// 0xa839cc — __ZN3RBX7Network6Player13setAccountAgeEi
// demangled: RBX::Network::Player::setAccountAge(int)
// type: int __fastcall(RBX::Network::Player *this, int)
#[doc(alias = "RBX::Network::Player::setAccountAge(int)")]
pub fn stub_a839cc() -> ! {
    todo!("0xa839cc RBX::Network::Player::setAccountAge(int)")
}

// 0xa83a00 — __ZN3RBX7Network6Player4kickEv
// demangled: RBX::Network::Player::kick(void)
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Player::kick(void)")]
pub fn stub_a83a00() -> ! {
    todo!("0xa83a00 RBX::Network::Player::kick(void)")
}

// 0xa83bac — __ZN3RBX7Network6Player12setCharacterEPNS_13ModelInstanceE
// demangled: RBX::Network::Player::setCharacter(RBX::ModelInstance *)
// type: void __fastcall(RBX::Network::Player *this, RBX::ModelInstance *)
#[doc(alias = "RBX::Network::Player::setCharacter(RBX::ModelInstance *)")]
pub fn stub_a83bac() -> ! {
    todo!("0xa83bac RBX::Network::Player::setCharacter(RBX::ModelInstance *)")
}

// 0xa84aec — __ZN3RBX7Network6Player22setCharacterAppearanceERKSs
// demangled: RBX::Network::Player::setCharacterAppearance(std::string const&)
// type: void __fastcall(RBX::Network::Player *this, const std::string *)
#[doc(alias = "RBX::Network::Player::setCharacterAppearance(std::string const&)")]
pub fn stub_a84aec() -> ! {
    todo!("0xa84aec RBX::Network::Player::setCharacterAppearance(std::string const&)")
}

// 0xa85160 — __ZN3RBX7Network6Player29setCanLoadCharacterAppearanceEb
// demangled: RBX::Network::Player::setCanLoadCharacterAppearance(bool)
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "RBX::Network::Player::setCanLoadCharacterAppearance(bool)")]
pub fn stub_a85160() -> ! {
    todo!("0xa85160 RBX::Network::Player::setCanLoadCharacterAppearance(bool)")
}

// 0xa85188 — __ZN3RBX7Network6Player31removeCharacterAppearanceScriptEv
// demangled: RBX::Network::Player::removeCharacterAppearanceScript(void)
// type: _DWORD __fastcall(RBX::Network::Player *__hidden this)
#[doc(alias = "RBX::Network::Player::removeCharacterAppearanceScript(void)")]
pub fn stub_a85188() -> ! {
    todo!("0xa85188 RBX::Network::Player::removeCharacterAppearanceScript(void)")
}
