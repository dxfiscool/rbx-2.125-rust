//! network generated_network_next105 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator|RakPeer|BitStream (5119 matched, 3548 in global set, this shard: 100 of 1571 fresh EA-sorted asc)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: 100 stubs | range 0xa5c12c..0xa63cf8 | rbx_core::SharedPtr (not boost::shared_ptr) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xa5c12c — __ZNK6RakNet13SystemAddressneERKS0_
// type: bool __fastcall(int, int)
#[doc(alias = "RakNet::SystemAddress::operator!=(RakNet::SystemAddress const&)const")]
pub fn stub_a5c12c(a: &crate::socket::SystemAddress, b: &crate::socket::SystemAddress) -> bool {
 // IDA 0xa5c12c: negated equality.
 a.not_equals(b)
}

// 0xa5c154 — __ZNK6RakNet13SystemAddress12GetIPVersionEv
// type: int __fastcall(RakNet::SystemAddress *this)
#[doc(alias = "RakNet::SystemAddress::GetIPVersion(void)const")]
pub fn stub_a5c154(addr: &crate::socket::SystemAddress) -> u32 {
 // IDA 0xa5c154: 4 for IPv4, else 6.
 addr.ip_version()
}

// 0xa5c160 — __ZNK6RakNet13SystemAddress10GetIPPROTOEv
// type: int __fastcall(RakNet::SystemAddress *this)
#[doc(alias = "RakNet::SystemAddress::GetIPPROTO(void)const")]
pub fn stub_a5c160(_addr: &crate::socket::SystemAddress) -> u32 {
 // IDA 0xa5c160: always 0.
 0
}

// 0xa5c164 — __ZN6RakNet13SystemAddress13SetToLoopbackEh
// type: int __fastcall(RakNet::SystemAddress *this, int)
#[doc(alias = "RakNet::SystemAddress::SetToLoopback(unsigned char)")]
pub fn stub_a5c164(addr: &mut crate::socket::SystemAddress, ipv4: bool) {
 // IDA 0xa5c164: loopback for the family.
 addr.set_to_loopback(ipv4)
}

// 0xa5c18c — __ZNK6RakNet13SystemAddress12ToString_OldEbPcc
// type: char *__fastcall(RakNet::SystemAddress *this, int, char *, char)
#[doc(alias = "RakNet::SystemAddress::ToString_Old(bool,char *,char)const")]
pub fn stub_a5c18c(addr: &crate::socket::SystemAddress, print_port: bool, delimiter: char) -> String {
 // IDA 0xa5c18c: sentinel or dotted text.
 addr.to_string_old(print_port, delimiter)
}

// 0xa5c220 — __ZN6RakNet13SystemAddressC1EPKct
// type: RakNet::SystemAddress *__fastcall(RakNet::SystemAddress *this, const char *, unsigned int)
#[doc(alias = "RakNet::SystemAddress::SystemAddress(char const*,unsigned short)")]
pub fn stub_a5c220(host: &str, port: u16, resolve: &mut dyn FnMut(&str) -> Option<u32>) -> crate::socket::SystemAddress {
 // IDA 0xa5c220: family, address, port images.
 crate::socket::SystemAddress::from_host_port(host, port, resolve)
}

// 0xa5c250 — __ZN6RakNet13SystemAddress22FromStringExplicitPortEPKcti
// type: int __fastcall(RakNet::SystemAddress *this, const char *, unsigned int, int)
#[doc(alias = "RakNet::SystemAddress::FromStringExplicitPort(char const*,unsigned short,int)")]
pub fn stub_a5c250(host: &str, port: u16, resolve: &mut dyn FnMut(&str) -> Option<u32>) -> crate::socket::SystemAddress {
 // IDA 0xa5c250: address plus explicit port.
 crate::socket::SystemAddress::from_string_explicit_port(host, port, resolve)
}

// 0xa5c274 — __ZN6RakNet13SystemAddress15FixForIPVersionERKS0_
// type: int __fastcall(RakNet::SystemAddress *this, const RakNet::SystemAddress *)
#[doc(alias = "RakNet::SystemAddress::FixForIPVersion(RakNet::SystemAddress const&)")]
pub fn stub_a5c274(addr: &mut crate::socket::SystemAddress, other: &crate::socket::SystemAddress) {
 // IDA 0xa5c274: v6 loopback to v4 against IPv4.
 addr.fix_for_ip_version(other)
}

// 0xa5c320 — __ZN6RakNet13SystemAddress16SetBinaryAddressEPKcc
// type: int __fastcall(RakNet::SystemAddress *this, RakNet::SocketLayer *__s, unsigned __int8)
#[doc(alias = "RakNet::SystemAddress::SetBinaryAddress(char const*,char)")]
pub fn stub_a5c320(addr: &mut crate::socket::SystemAddress, host: &str, resolve: &mut dyn FnMut(&str) -> Option<u32>) {
 // IDA 0xa5c320: dotted, localhost, or DNS parse.
 addr.set_binary_address(host, resolve)
}

// 0xa5c498 — __ZN6RakNet13SystemAddress8CopyPortERKS0_
// type: int __fastcall(int this, const RakNet::SystemAddress *)
#[doc(alias = "RakNet::SystemAddress::CopyPort(RakNet::SystemAddress const&)")]
pub fn stub_a5c498(dst: &mut crate::socket::SystemAddress, src: &crate::socket::SystemAddress) {
 // IDA 0xa5c498: port and debug images.
 dst.copy_port(src)
}

// 0xa5c4a4 — __ZNK6RakNet10RakNetGUIDeqERKS0_
// type: bool __fastcall(__int64 *, __int64 *)
#[doc(alias = "RakNet::RakNetGUID::operator==(RakNet::RakNetGUID const&)const")]
pub fn stub_a5c4a4(a: &crate::socket::RakNetGuid, b: &crate::socket::RakNetGuid) -> bool {
 // IDA 0xa5c4a4: guid equality.
 !a.not_equal(b)
}

// 0xa5cb00 — __ZN6RakNet7RakPeerC2Ev
// type: RakNet::RakPeer *__fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::RakPeer(void)")]
pub fn stub_a5cb00() -> crate::socket::RakPeer {
 // IDA 0xa5cb00: default construct.
 crate::socket::RakPeer::new()
}

// 0xa5d8f0 — __ZN6RakNet7RakPeerD0Ev
// type: void __fastcall(RakNet::RakPeer *__hidden this)
#[doc(alias = "RakNet::RakPeer::~RakPeer()")]
pub fn stub_a5d8f0(peer: crate::socket::RakPeer) {
 // IDA 0xa5d8f0: frees; Rust drops it.
 drop(peer);
}

// 0xa5d990 — __ZN6RakNet7RakPeerD1Ev
// type: void __fastcall(RakNet::RakPeer *__hidden this)
#[doc(alias = "RakNet::RakPeer::~RakPeer()")]
pub fn stub_a5d990(peer: crate::socket::RakPeer) {
 // IDA 0xa5d990: frees; Rust drops it.
 drop(peer);
}

// 0xa5d99c — __ZN6RakNet7RakPeerD2Ev
// type: void __fastcall(RakNet::RakPeer *__hidden this)
#[doc(alias = "RakNet::RakPeer::~RakPeer()")]
pub fn stub_a5d99c(peer: crate::socket::RakPeer) {
 // IDA 0xa5d99c: frees; Rust drops it.
 drop(peer);
}

// 0xa5e3c0 — __ZN6RakNet7RakPeer7StartupEtPNS_16SocketDescriptorEji
// type: int __fastcall(RakNet::RakPeer *this, unsigned int, RakNet::SocketDescriptor *, unsigned int, int)
#[doc(alias = "RakNet::RakPeer::Startup(unsigned short,RakNet::SocketDescriptor *,unsigned int,int)")]
pub fn stub_a5e3c0(peer: &mut crate::socket::RakPeer, active: bool, has_descriptors: bool, port_in_use: bool, bind_ok: bool, threads_ok: bool) -> u32 {
 // IDA 0xa5e3c0: bind plus threads, coded result.
 peer.startup(active, has_descriptors, port_in_use, bind_ok, threads_ok)
}

// 0xa5eab8 — __ZN6RakNet7RakPeer15DerefAllSocketsEv
// type: void __fastcall(RakNet::RakPeer *this, int, int, int)
#[doc(alias = "RakNet::RakPeer::DerefAllSockets(void)")]
pub fn stub_a5eab8() {
 // IDA 0xa5eab8: socket release stays engine-side.
 crate::socket::RakPeer::deref_all_sockets()
}

// 0xa5ebd4 — __ZN6RakNet7RakPeer21ClearBufferedCommandsEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::ClearBufferedCommands(void)")]
pub fn stub_a5ebd4() {
 // IDA 0xa5ebd4: queue release stays engine-side.
 crate::socket::RakPeer::clear_buffered_commands()
}

// 0xa5eca0 — __ZN6RakNet7RakPeer20ClearBufferedPacketsEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::ClearBufferedPackets(void)")]
pub fn stub_a5eca0() {
 // IDA 0xa5eca0: queue release stays engine-side.
 crate::socket::RakPeer::clear_buffered_packets()
}

// 0xa5ed50 — __ZN6RakNet17UpdateNetworkLoopEPv
// type: int __fastcall(RakNet *this, void *)
#[doc(alias = "RakNet::UpdateNetworkLoop(void *)")]
pub fn stub_a5ed50() {
 // IDA 0xa5ed50: network thread entry stays engine-side.
 crate::socket::RakPeer::update_network_loop()
}

// 0xa5ee80 — __ZN6RakNet12RecvFromLoopEPv
// type: int __fastcall(RakNet::SocketLayer **this, void *)
#[doc(alias = "RakNet::RecvFromLoop(void *)")]
pub fn stub_a5ee80() {
 // IDA 0xa5ee80: receive thread entry stays engine-side.
 crate::socket::RakPeer::recv_from_loop()
}

// 0xa5efa0 — __ZN6RakNet7RakPeer18InitializeSecurityEPKcS2_b
// type: int __fastcall(RakNet::RakPeer *this, const char *, const char *, bool)
#[doc(alias = "RakNet::RakPeer::InitializeSecurity(char const*,char const*,bool)")]
pub fn stub_a5efa0(peer: &mut crate::socket::RakPeer) -> u32 {
 // IDA 0xa5efa0: hardcoded return 0.
 peer.initialize_security()
}

// 0xa5efa4 — __ZN6RakNet7RakPeer15DisableSecurityEv
// type: void __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::DisableSecurity(void)")]
pub fn stub_a5efa4(peer: &mut crate::socket::RakPeer) {
 // IDA 0xa5efa4: empty.
 peer.disable_security()
}

// 0xa5efa8 — __ZN6RakNet7RakPeer26AddToSecurityExceptionListEPKc
// type: void __fastcall(RakNet::RakPeer *this, const char *)
#[doc(alias = "RakNet::RakPeer::AddToSecurityExceptionList(char const*)")]
pub fn stub_a5efa8(peer: &mut crate::socket::RakPeer, addr: &str) {
 // IDA 0xa5efa8: push the pattern.
 peer.add_to_security_exception_list(addr)
}

// 0xa5f08c — __ZN6RakNet7RakPeer31RemoveFromSecurityExceptionListEPKc
// type: void __fastcall(RakNet::RakPeer *this, const char *)
#[doc(alias = "RakNet::RakPeer::RemoveFromSecurityExceptionList(char const*)")]
pub fn stub_a5f08c(peer: &mut crate::socket::RakPeer, addr: Option<&str>) {
 // IDA 0xa5f08c: drop matches, or clear for null.
 peer.remove_from_security_exception_list(addr)
}

// 0xa5f230 — __ZN6RakNet7RakPeer25IsInSecurityExceptionListEPKc
// type: int __fastcall(RakNet::RakPeer *this, const char *)
#[doc(alias = "RakNet::RakPeer::IsInSecurityExceptionList(char const*)")]
pub fn stub_a5f230(peer: &crate::socket::RakPeer, addr: &str) -> bool {
 // IDA 0xa5f230: wildcard scan.
 peer.is_in_security_exception_list(addr)
}

// 0xa5f28c — __ZN6RakNet7RakPeer29SetMaximumIncomingConnectionsEt
// type: int __fastcall(int this, unsigned __int16)
#[doc(alias = "RakNet::RakPeer::SetMaximumIncomingConnections(unsigned short)")]
pub fn stub_a5f28c(peer: &mut crate::socket::RakPeer, max: u16) {
 // IDA 0xa5f28c: store the limit.
 peer.set_maximum_incoming_connections(max)
}

// 0xa5f290 — __ZNK6RakNet7RakPeer29GetMaximumIncomingConnectionsEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::GetMaximumIncomingConnections(void)const")]
pub fn stub_a5f290(peer: &crate::socket::RakPeer) -> u16 {
 // IDA 0xa5f290: load the limit.
 peer.maximum_incoming_connections()
}

// 0xa5f294 — __ZNK6RakNet7RakPeer19NumberOfConnectionsEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::NumberOfConnections(void)const")]
pub fn stub_a5f294(peer: &crate::socket::RakPeer, count_actives: &mut dyn FnMut() -> u16) -> u16 {
 // IDA 0xa5f294: active-system enumeration stays engine-side.
 peer.number_of_connections(count_actives)
}

// 0xa5f37c — __ZN6RakNet7RakPeer19SetIncomingPasswordEPKci
// type: char *__fastcall(char *this, size_t __n, int)
#[doc(alias = "RakNet::RakPeer::SetIncomingPassword(char const*,int)")]
pub fn stub_a5f37c(peer: &mut crate::socket::RakPeer, data: Option<&[u8]>) {
 // IDA 0xa5f37c: capped password store.
 peer.set_incoming_password(data)
}

// 0xa5f3a4 — __ZN6RakNet7RakPeer19GetIncomingPasswordEPcPi
// type: int __fastcall(int this, char *__dst, int *)
#[doc(alias = "RakNet::RakPeer::GetIncomingPassword(char *,int *)")]
pub fn stub_a5f3a4(peer: &crate::socket::RakPeer, out: Option<&mut Vec<u8>>, len: &mut usize) -> usize {
 // IDA 0xa5f3a4: bounded copy plus length report.
 peer.incoming_password(out, len)
}

// 0xa5f3d8 — __ZN6RakNet7RakPeer7ConnectEPKctS2_iPNS_9PublicKeyEjjjj
// type: int __fastcall(int, int, int, int, int, int, unsigned int)
#[doc(alias = "RakNet::RakPeer::Connect(char const*,unsigned short,char const*,int,RakNet::PublicKey *,unsigned int,unsigned int,unsigned int,unsigned int)")]
pub fn stub_a5f3d8() -> ! {
    todo!("0xa5f3d8 RakNet::RakPeer::Connect(char const*,unsigned short,char const*,int,RakNet::PublicKey *,unsigned int,unsigned int,unsigned int,unsigned int)")
}

// 0xa5f460 — __ZN6RakNet7RakPeer21SendConnectionRequestEPKctS2_iPNS_9PublicKeyEjjjjj
// type: int __fastcall(int, const char *, unsigned int, const void *, size_t, int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::SendConnectionRequest(char const*,unsigned short,char const*,int,RakNet::PublicKey *,unsigned int,unsigned int,unsigned int,unsigned int,unsigned int)")]
pub fn stub_a5f460() -> ! {
    todo!("0xa5f460 RakNet::RakPeer::SendConnectionRequest(char const*,unsigned short,char const*,int,RakNet::PublicKey *,unsigned int,unsigned int,unsigned int,unsigned int,unsigned int)")
}

// 0xa5f754 — __ZN6RakNet7RakPeer17ConnectWithSocketEPKctS2_iNS_14RakNetSmartPtrINS_12RakNetSocketEEEPNS_9PublicKeyEjjj
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, void *, void *, RakNet::RakNetSocket *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RakNet::RakPeer::ConnectWithSocket(char const*,unsigned short,char const*,int,RakNet::RakNetSmartPtr<RakNet::RakNetSocket>,RakNet::PublicKey *,unsigned int,unsigned int,unsigned int)")]
pub fn stub_a5f754() -> ! {
    todo!("0xa5f754 RakNet::RakPeer::ConnectWithSocket(char const*,unsigned short,char const*,int,RakNet::RakNetSmartPtr<RakNet::RakNetSocket>,RakNet::PublicKey *,unsigned int,unsigned int,unsigned int)")
}

// 0xa5f8cc — __ZN6RakNet7RakPeer21SendConnectionRequestEPKctS2_iPNS_9PublicKeyEjjjjjNS_14RakNetSmartPtrINS_12RakNetSocketEEE
// type: int __fastcall(int, const char *, unsigned int, const void *, size_t, int, int, int, int, int, int, _DWORD *)
#[doc(alias = "RakNet::RakPeer::SendConnectionRequest(char const*,unsigned short,char const*,int,RakNet::PublicKey *,unsigned int,unsigned int,unsigned int,unsigned int,unsigned int,RakNet::RakNetSmartPtr<RakNet::RakNetSocket>)")]
pub fn stub_a5f8cc() -> ! {
    todo!("0xa5f8cc RakNet::RakPeer::SendConnectionRequest(char const*,unsigned short,char const*,int,RakNet::PublicKey *,unsigned int,unsigned int,unsigned int,unsigned int,unsigned int,RakNet::RakNetSmartPtr<RakNet::RakNetSocket>)")
}

// 0xa5fc00 — __ZN6RakNet7RakPeer8ShutdownEjh14PacketPriority
// type: int __fastcall(int, unsigned int, char, int)
#[doc(alias = "RakNet::RakPeer::Shutdown(unsigned int,unsigned char,PacketPriority)")]
pub fn stub_a5fc00(peer: &mut crate::socket::RakPeer, block_ms: u32, notify: &mut dyn FnMut(), detach: &mut dyn FnMut(), clear: &mut dyn FnMut()) {
 // IDA 0xa5fc00: notify, detach, reset.
 peer.shutdown(block_ms, notify, detach, clear)
}

// 0xa60494 — __ZN6RakNet7RakPeer24NotifyAndFlagForShutdownENS_13SystemAddressEbh14PacketPriority
// type: int __fastcall(int, int, int, int, int, int, int, char, int)
#[doc(alias = "RakNet::RakPeer::NotifyAndFlagForShutdown(RakNet::SystemAddress,bool,unsigned char,PacketPriority)")]
pub fn stub_a60494() -> ! {
    todo!("0xa60494 RakNet::RakPeer::NotifyAndFlagForShutdown(RakNet::SystemAddress,bool,unsigned char,PacketPriority)")
}

// 0xa606a0 — __ZN6RakNet7RakPeer28ClearRequestedConnectionListEv
// type: void __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::ClearRequestedConnectionList(void)")]
pub fn stub_a606a0() {
 // IDA 0xa606a0: table release stays engine-side.
 crate::socket::RakPeer::clear_requested_connection_list()
}

// 0xa60878 — __ZN6RakNet7RakPeer23ClearRemoteSystemLookupEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::ClearRemoteSystemLookup(void)")]
pub fn stub_a60878() {
 // IDA 0xa60878: lookup release stays engine-side.
 crate::socket::RakPeer::clear_remote_system_lookup()
}

// 0xa60958 — __ZNK6RakNet7RakPeer17GetConnectionListEPNS_13SystemAddressEPt
// type: int __fastcall(RakNet::RakPeer *this, RakNet::SystemAddress *, unsigned __int16 *)
#[doc(alias = "RakNet::RakPeer::GetConnectionList(RakNet::SystemAddress *,unsigned short *)const")]
pub fn stub_a60958(active: bool, remotes: &[crate::socket::SystemAddress], capacity: usize) -> Vec<crate::socket::SystemAddress> {
 // IDA 0xa60958: capped active list.
 crate::socket::RakPeer::connection_list(active, remotes, capacity)
}

// 0xa60ab0 — __ZN6RakNet7RakPeer18GetNextSendReceiptEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::GetNextSendReceipt(void)")]
pub fn stub_a60ab0(peer: &crate::socket::RakPeer) -> u32 {
 // IDA 0xa60ab0: load the receipt counter.
 peer.next_send_receipt()
}

// 0xa60ad0 — __ZN6RakNet7RakPeer24IncrementNextSendReceiptEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::IncrementNextSendReceipt(void)")]
pub fn stub_a60ad0(peer: &mut crate::socket::RakPeer) -> u32 {
 // IDA 0xa60ad0: bump the receipt counter.
 peer.increment_next_send_receipt()
}

// 0xa60af8 — __ZN6RakNet7RakPeer4SendEPKci14PacketPriority17PacketReliabilitycNS_13AddressOrGUIDEbj
// type: int __fastcall(int, int, int, int, int, int, __int64 *, int, int)
#[doc(alias = "RakNet::RakPeer::Send(char const*,int,PacketPriority,PacketReliability,char,RakNet::AddressOrGUID,bool,unsigned int)")]
pub fn stub_a60af8() -> ! {
    todo!("0xa60af8 RakNet::RakPeer::Send(char const*,int,PacketPriority,PacketReliability,char,RakNet::AddressOrGUID,bool,unsigned int)")
}

// 0xa60cac — __ZN6RakNet7RakPeer12SendBufferedEPKcj14PacketPriority17PacketReliabilitycNS_13AddressOrGUIDEbNS0_18RemoteSystemStruct11ConnectModeEj
// type: int __fastcall(int, const void *, int, int, int, char, int, char, int, unsigned int)
#[doc(alias = "RakNet::RakPeer::SendBuffered(char const*,unsigned int,PacketPriority,PacketReliability,char,RakNet::AddressOrGUID,bool,RakNet::RakPeer::RemoteSystemStruct::ConnectMode,unsigned int)")]
pub fn stub_a60cac() -> ! {
    todo!("0xa60cac RakNet::RakPeer::SendBuffered(char const*,unsigned int,PacketPriority,PacketReliability,char,RakNet::AddressOrGUID,bool,RakNet::RakPeer::RemoteSystemStruct::ConnectMode,unsigned int)")
}

// 0xa60dec — __ZN6RakNet7RakPeer12SendLoopbackEPKci
// type: int __fastcall(RakNet::RakPeer *this, const char *, int)
#[doc(alias = "RakNet::RakPeer::SendLoopback(char const*,int)")]
pub fn stub_a60dec(data: Option<&[u8]>, push: &mut dyn FnMut(&[u8])) {
 // IDA 0xa60dec: queue the loopback packet.
 crate::socket::RakPeer::send_loopback(data, push)
}

// 0xa60f00 — __ZN6RakNet7RakPeer4SendEPKNS_9BitStreamE14PacketPriority17PacketReliabilitycNS_13AddressOrGUIDEbj
// type: unsigned int __fastcall(int, int *, int, int, char, int, int, int)
#[doc(alias = "RakNet::RakPeer::Send(RakNet::BitStream const*,PacketPriority,PacketReliability,char,RakNet::AddressOrGUID,bool,unsigned int)")]
pub fn stub_a60f00() -> ! {
    todo!("0xa60f00 RakNet::RakPeer::Send(RakNet::BitStream const*,PacketPriority,PacketReliability,char,RakNet::AddressOrGUID,bool,unsigned int)")
}

// 0xa610c4 — __ZN6RakNet7RakPeer8SendListEPPKcPKii14PacketPriority17PacketReliabilitycNS_13AddressOrGUIDEbj
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::SendList(char const**,int const*,int,PacketPriority,PacketReliability,char,RakNet::AddressOrGUID,bool,unsigned int)")]
pub fn stub_a610c4() -> ! {
    todo!("0xa610c4 RakNet::RakPeer::SendList(char const**,int const*,int,PacketPriority,PacketReliability,char,RakNet::AddressOrGUID,bool,unsigned int)")
}

// 0xa611b8 — __ZN6RakNet7RakPeer16SendBufferedListEPPKcPKii14PacketPriority17PacketReliabilitycNS_13AddressOrGUIDEbNS0_18RemoteSystemStruct11ConnectModeEj
// type: void __fastcall(__int64 *, const void **, size_t *, int, int, int, char, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::SendBufferedList(char const**,int const*,int,PacketPriority,PacketReliability,char,RakNet::AddressOrGUID,bool,RakNet::RakPeer::RemoteSystemStruct::ConnectMode,unsigned int)")]
pub fn stub_a611b8() -> ! {
    todo!("0xa611b8 RakNet::RakPeer::SendBufferedList(char const**,int const*,int,PacketPriority,PacketReliability,char,RakNet::AddressOrGUID,bool,RakNet::RakPeer::RemoteSystemStruct::ConnectMode,unsigned int)")
}

// 0xa613c4 — __ZN6RakNet7RakPeer7ReceiveEv
// type: const RakNet::SystemAddress *__fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::Receive(void)")]
pub fn stub_a613c4(next: Option<u32>) -> Option<u32> {
 // IDA 0xa613c4: pop or null.
 crate::socket::RakPeer::receive(next)
}

// 0xa61520 — __ZNK6RakNet7RakPeer22ShiftIncomingTimestampEPhRKNS_13SystemAddressE
// type: int __fastcall(RakNet::RakPeer *this, unsigned __int8 *, const RakNet::SystemAddress *)
#[doc(alias = "RakNet::RakPeer::ShiftIncomingTimestamp(unsigned char *,RakNet::SystemAddress const&)const")]
pub fn stub_a61520() -> ! {
    todo!("0xa61520 RakNet::RakPeer::ShiftIncomingTimestamp(unsigned char *,RakNet::SystemAddress const&)const")
}

// 0xa61698 — __ZN6RakNet7RakPeer19CallPluginCallbacksERN14DataStructures4ListIPNS_16PluginInterface2EEEPNS_6PacketE
// type: unsigned int __fastcall(int, _DWORD *, int)
#[doc(alias = "RakNet::RakPeer::CallPluginCallbacks(DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::Packet *)")]
pub fn stub_a61698() -> ! {
    todo!("0xa61698 RakNet::RakPeer::CallPluginCallbacks(DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::Packet *)")
}

// 0xa61810 — __ZN6RakNet7RakPeer16DeallocatePacketEPNS_6PacketE
// type: int __fastcall(int result, RakNet **this)
#[doc(alias = "RakNet::RakPeer::DeallocatePacket(RakNet::Packet *)")]
pub fn stub_a61810() {
 // IDA 0xa61810: packet release stays engine-side.
 crate::socket::RakPeer::deallocate_packet()
}

// 0xa61888 — __ZNK6RakNet7RakPeer23GetMaximumNumberOfPeersEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::GetMaximumNumberOfPeers(void)const")]
pub fn stub_a61888(peer: &crate::socket::RakPeer) -> u16 {
 // IDA 0xa61888: load the peer cap.
 peer.maximum_number_of_peers()
}

// 0xa6188c — __ZN6RakNet7RakPeer15CloseConnectionENS_13AddressOrGUIDEbh14PacketPriority
// type: int __fastcall(int, _DWORD *, int, int, int)
#[doc(alias = "RakNet::RakPeer::CloseConnection(RakNet::AddressOrGUID,bool,unsigned char,PacketPriority)")]
pub fn stub_a6188c() -> ! {
    todo!("0xa6188c RakNet::RakPeer::CloseConnection(RakNet::AddressOrGUID,bool,unsigned char,PacketPriority)")
}

// 0xa61a8c — __ZN6RakNet7RakPeer23CloseConnectionInternalERKNS_13AddressOrGUIDEbbh14PacketPriority
// type: int __fastcall(int, __int64 *, int, int, char, int)
#[doc(alias = "RakNet::RakPeer::CloseConnectionInternal(RakNet::AddressOrGUID const&,bool,bool,unsigned char,PacketPriority)")]
pub fn stub_a61a8c() -> ! {
    todo!("0xa61a8c RakNet::RakPeer::CloseConnectionInternal(RakNet::AddressOrGUID const&,bool,bool,unsigned char,PacketPriority)")
}

// 0xa61e58 — __ZN6RakNet7RakPeer23CancelConnectionAttemptENS_13SystemAddressE
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::CancelConnectionAttempt(RakNet::SystemAddress)")]
pub fn stub_a61e58(cancel: &mut dyn FnMut()) {
 // IDA 0xa61e58: drop the pending attempt.
 crate::socket::RakPeer::cancel_connection_attempt(cancel)
}

// 0xa62070 — __ZN6RakNet7RakPeer18GetConnectionStateENS_13AddressOrGUIDE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "RakNet::RakPeer::GetConnectionState(RakNet::AddressOrGUID)")]
pub fn stub_a62070(address_known: bool, direct_match: bool, index: i32, active: bool, state: u32) -> u32 {
 // IDA 0xa62070: hit, index, activity, mapped state.
 crate::socket::RakPeer::connection_state(address_known, direct_match, index, active, state)
}

// 0xa62178 — __ZNK6RakNet7RakPeer25GetIndexFromSystemAddressENS_13SystemAddressEb
// type: unsigned int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::GetIndexFromSystemAddress(RakNet::SystemAddress,bool)const")]
pub fn stub_a62178(remotes: &[(crate::socket::SystemAddress, bool)], addr: &crate::socket::SystemAddress, unassigned: &crate::socket::SystemAddress, hint: Option<usize>) -> i32 {
 // IDA 0xa62178: hint, active match, any match.
 crate::socket::RakPeer::index_from_address(remotes, addr, unassigned, hint)
}

// 0xa622d8 — __ZN6RakNet7RakPeer16GetIndexFromGuidENS_10RakNetGUIDE
// type: unsigned int __fastcall(int, unsigned int, unsigned int, int)
#[doc(alias = "RakNet::RakPeer::GetIndexFromGuid(RakNet::RakNetGUID)")]
pub fn stub_a622d8(remotes: &[(u64, bool)], guid: u64, unassigned: u64, hint: Option<usize>) -> i32 {
 // IDA 0xa622d8: guid version of the index scan.
 crate::socket::RakPeer::index_from_guid(remotes, guid, unassigned, hint)
}

// 0xa623c8 — __ZNK6RakNet7RakPeer25GetIndexFromSystemAddressENS_13SystemAddressE
// type: int __fastcall(int, int, int)
#[doc(alias = "RakNet::RakPeer::GetIndexFromSystemAddress(RakNet::SystemAddress)const")]
pub fn stub_a623c8(remotes: &[(crate::socket::SystemAddress, bool)], addr: &crate::socket::SystemAddress, unassigned: &crate::socket::SystemAddress, hint: Option<usize>) -> i32 {
 // IDA 0xa623c8: same scan without the flag.
 crate::socket::RakPeer::index_from_address(remotes, addr, unassigned, hint)
}

// 0xa623e8 — __ZN6RakNet7RakPeer25GetSystemAddressFromIndexEi
// type: int __fastcall(int this, int, int)
#[doc(alias = "RakNet::RakPeer::GetSystemAddressFromIndex(int)")]
pub fn stub_a623e8(remotes: &[Option<crate::socket::SystemAddress>], index: i32, unassigned: crate::socket::SystemAddress) -> crate::socket::SystemAddress {
 // IDA 0xa623e8: connected slot or unassigned.
 crate::socket::RakPeer::system_address_from_index(remotes, index, unassigned)
}

// 0xa62440 — __ZN6RakNet7RakPeer16GetGUIDFromIndexEi
// type: int __fastcall(int this, int, int)
#[doc(alias = "RakNet::RakPeer::GetGUIDFromIndex(int)")]
pub fn stub_a62440(remotes: &[Option<crate::socket::RakNetGuid>], index: i32, unassigned: crate::socket::RakNetGuid) -> crate::socket::RakNetGuid {
 // IDA 0xa62440: guid mirror of the slot read.
 crate::socket::RakPeer::guid_from_index(remotes, index, unassigned)
}

// 0xa624a4 — __ZNK6RakNet7RakPeer13GetSystemListERN14DataStructures4ListINS_13SystemAddressEEERNS2_INS_10RakNetGUIDEEE
// type: unsigned int __fastcall(int, int, int)
#[doc(alias = "RakNet::RakPeer::GetSystemList(DataStructures::List<RakNet::SystemAddress> &,DataStructures::List<RakNet::RakNetGUID> &)const")]
pub fn stub_a624a4() -> ! {
    todo!("0xa624a4 RakNet::RakPeer::GetSystemList(DataStructures::List<RakNet::SystemAddress> &,DataStructures::List<RakNet::RakNetGUID> &)const")
}

// 0xa62560 — __ZN6RakNet7RakPeer12AddToBanListEPKcj
// type: unsigned int __fastcall(RakNet::RakPeer *this, const char *, unsigned int)
#[doc(alias = "RakNet::RakPeer::AddToBanList(char const*,unsigned int)")]
pub fn stub_a62560(peer: &mut crate::socket::RakPeer, addr: &str, timeout_ms: u32, now_ms: u32) {
 // IDA 0xa62560: insert or refresh the ban entry.
 peer.add_to_ban_list(addr, timeout_ms, now_ms)
}

// 0xa62698 — __ZN6RakNet7RakPeer17RemoveFromBanListEPKc
// type: void __fastcall(RakNet::RakPeer *this, const char *__s)
#[doc(alias = "RakNet::RakPeer::RemoveFromBanList(char const*)")]
pub fn stub_a62698(peer: &mut crate::socket::RakPeer, addr: Option<&str>) {
 // IDA 0xa62698: exact swap-remove.
 peer.remove_from_ban_list(addr)
}

// 0xa6273c — __ZN6RakNet7RakPeer12ClearBanListEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::ClearBanList(void)")]
pub fn stub_a6273c(peer: &mut crate::socket::RakPeer) {
 // IDA 0xa6273c: drop the whole list.
 peer.clear_ban_list()
}

// 0xa627c8 — __ZN6RakNet7RakPeer29SetLimitIPConnectionFrequencyEb
// type: int __fastcall(int this, bool)
#[doc(alias = "RakNet::RakPeer::SetLimitIPConnectionFrequency(bool)")]
pub fn stub_a627c8(peer: &mut crate::socket::RakPeer, limit: bool) {
 // IDA 0xa627c8: store the limiter flag.
 peer.set_limit_ip_connection_frequency(limit)
}

// 0xa627d0 — __ZN6RakNet7RakPeer8IsBannedEPKc
// type: int __fastcall(RakNet::RakPeer *this, const char *)
#[doc(alias = "RakNet::RakPeer::IsBanned(char const*)")]
pub fn stub_a627d0(peer: &mut crate::socket::RakPeer, addr: &str, now_ms: u32) -> bool {
 // IDA 0xa627d0: evict expired, then wildcard walk.
 peer.is_banned(addr, now_ms)
}

// 0xa628c0 — __ZN6RakNet7RakPeer4PingENS_13SystemAddressE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RakNet::RakPeer::Ping(RakNet::SystemAddress)")]
pub fn stub_a628c0(active: bool, broadcast: bool, send: &mut dyn FnMut(bool)) {
 // IDA 0xa628c0: forwards to the internal ping.
 crate::socket::RakPeer::send_ping(active, broadcast, send)
}

// 0xa628e4 — __ZN6RakNet7RakPeer12PingInternalENS_13SystemAddressEb17PacketReliability
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::PingInternal(RakNet::SystemAddress,bool,PacketReliability)")]
pub fn stub_a628e4(active: bool, broadcast: bool, send: &mut dyn FnMut(bool)) {
 // IDA 0xa628e4: timestamped ping when active.
 crate::socket::RakPeer::send_ping(active, broadcast, send)
}

// 0xa62af0 — __ZN6RakNet7RakPeer4PingEPKctbj
// type: int __fastcall(RakNet::RakPeer *this, const char *, unsigned int, int, unsigned int)
#[doc(alias = "RakNet::RakPeer::Ping(char const*,unsigned short,bool,unsigned int)")]
pub fn stub_a62af0() -> ! {
    todo!("0xa62af0 RakNet::RakPeer::Ping(char const*,unsigned short,bool,unsigned int)")
}

// 0xa62d48 — __ZN6RakNet7RakPeer14GetAveragePingENS_13AddressOrGUIDE
// type: int __fastcall(int, __int64 *)
#[doc(alias = "RakNet::RakPeer::GetAveragePing(RakNet::AddressOrGUID)")]
pub fn stub_a62d48() -> ! {
    todo!("0xa62d48 RakNet::RakPeer::GetAveragePing(RakNet::AddressOrGUID)")
}

// 0xa62dec — __ZNK6RakNet7RakPeer15GetRemoteSystemENS_13AddressOrGUIDEbb
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RakNet::RakPeer::GetRemoteSystem(RakNet::AddressOrGUID,bool,bool)const")]
pub fn stub_a62dec() -> ! {
    todo!("0xa62dec RakNet::RakPeer::GetRemoteSystem(RakNet::AddressOrGUID,bool,bool)const")
}

// 0xa62ea0 — __ZNK6RakNet7RakPeer11GetLastPingENS_13AddressOrGUIDE
// type: int __fastcall(int, __int64 *)
#[doc(alias = "RakNet::RakPeer::GetLastPing(RakNet::AddressOrGUID)const")]
pub fn stub_a62ea0() -> ! {
    todo!("0xa62ea0 RakNet::RakPeer::GetLastPing(RakNet::AddressOrGUID)const")
}

// 0xa62f3c — __ZNK6RakNet7RakPeer13GetLowestPingENS_13AddressOrGUIDE
// type: int __fastcall(int, __int64 *)
#[doc(alias = "RakNet::RakPeer::GetLowestPing(RakNet::AddressOrGUID)const")]
pub fn stub_a62f3c() -> ! {
    todo!("0xa62f3c RakNet::RakPeer::GetLowestPing(RakNet::AddressOrGUID)const")
}

// 0xa62fbc — __ZN6RakNet7RakPeer17SetOccasionalPingEb
// type: int __fastcall(int this, bool)
#[doc(alias = "RakNet::RakPeer::SetOccasionalPing(bool)")]
pub fn stub_a62fbc() -> ! {
    todo!("0xa62fbc RakNet::RakPeer::SetOccasionalPing(bool)")
}

// 0xa62fc0 — __ZN6RakNet7RakPeer22SetOfflinePingResponseEPKcj
// type: int __fastcall(RakNet::RakPeer *this, const char *, size_t)
#[doc(alias = "RakNet::RakPeer::SetOfflinePingResponse(char const*,unsigned int)")]
pub fn stub_a62fc0() -> ! {
    todo!("0xa62fc0 RakNet::RakPeer::SetOfflinePingResponse(char const*,unsigned int)")
}

// 0xa63000 — __ZN6RakNet7RakPeer22GetOfflinePingResponseEPPcPj
// type: int __fastcall(RakNet::RakPeer *this, char **, unsigned int *)
#[doc(alias = "RakNet::RakPeer::GetOfflinePingResponse(char **,unsigned int *)")]
pub fn stub_a63000() -> ! {
    todo!("0xa63000 RakNet::RakPeer::GetOfflinePingResponse(char **,unsigned int *)")
}

// 0xa63034 — __ZNK6RakNet7RakPeer13GetInternalIDENS_13SystemAddressEi
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::GetInternalID(RakNet::SystemAddress,int)const")]
pub fn stub_a63034() -> ! {
    todo!("0xa63034 RakNet::RakPeer::GetInternalID(RakNet::SystemAddress,int)const")
}

// 0xa63140 — __ZNK6RakNet7RakPeer32GetRemoteSystemFromSystemAddressENS_13SystemAddressEbb
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::GetRemoteSystemFromSystemAddress(RakNet::SystemAddress,bool,bool)const")]
pub fn stub_a63140() -> ! {
    todo!("0xa63140 RakNet::RakPeer::GetRemoteSystemFromSystemAddress(RakNet::SystemAddress,bool,bool)const")
}

// 0xa63278 — __ZNK6RakNet7RakPeer13GetExternalIDENS_13SystemAddressE
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::GetExternalID(RakNet::SystemAddress)const")]
pub fn stub_a63278() -> ! {
    todo!("0xa63278 RakNet::RakPeer::GetExternalID(RakNet::SystemAddress)const")
}

// 0xa63378 — __ZNK6RakNet7RakPeer9GetMyGUIDEv
// type: int __fastcall(int this, int)
#[doc(alias = "RakNet::RakPeer::GetMyGUID(void)const")]
pub fn stub_a63378() -> ! {
    todo!("0xa63378 RakNet::RakPeer::GetMyGUID(void)const")
}

// 0xa6338c — __ZN6RakNet7RakPeer17GetMyBoundAddressEi
// type: void __fastcall(RakNet::RakPeer *this, int, int)
#[doc(alias = "RakNet::RakPeer::GetMyBoundAddress(int)")]
pub fn stub_a6338c() -> ! {
    todo!("0xa6338c RakNet::RakPeer::GetMyBoundAddress(int)")
}

// 0xa63490 — __ZNK6RakNet7RakPeer24GetGuidFromSystemAddressENS_13SystemAddressE
// type: double *__fastcall(int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::GetGuidFromSystemAddress(RakNet::SystemAddress)const")]
pub fn stub_a63490() -> ! {
    todo!("0xa63490 RakNet::RakPeer::GetGuidFromSystemAddress(RakNet::SystemAddress)const")
}

// 0xa63574 — __ZNK6RakNet7RakPeer22GetSystemIndexFromGuidENS_10RakNetGUIDE
// type: int __fastcall(int, unsigned int, unsigned int, int)
#[doc(alias = "RakNet::RakPeer::GetSystemIndexFromGuid(RakNet::RakNetGUID)const")]
pub fn stub_a63574() -> ! {
    todo!("0xa63574 RakNet::RakPeer::GetSystemIndexFromGuid(RakNet::RakNetGUID)const")
}

// 0xa63620 — __ZNK6RakNet7RakPeer24GetSystemAddressFromGuidENS_10RakNetGUIDE
// type: int __fastcall(int, int, __int64, int)
#[doc(alias = "RakNet::RakPeer::GetSystemAddressFromGuid(RakNet::RakNetGUID)const")]
pub fn stub_a63620() -> ! {
    todo!("0xa63620 RakNet::RakPeer::GetSystemAddressFromGuid(RakNet::RakNetGUID)const")
}

// 0xa63750 — __ZNK6RakNet7RakPeer35GetClientPublicKeyFromSystemAddressENS_13SystemAddressEPc
// type: int()
#[doc(alias = "RakNet::RakPeer::GetClientPublicKeyFromSystemAddress(RakNet::SystemAddress,char *)const")]
pub fn stub_a63750() -> ! {
    todo!("0xa63750 RakNet::RakPeer::GetClientPublicKeyFromSystemAddress(RakNet::SystemAddress,char *)const")
}

// 0xa63754 — __ZN6RakNet7RakPeer14SetTimeoutTimeEjNS_13SystemAddressE
// type: int __fastcall(int, unsigned int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::SetTimeoutTime(unsigned int,RakNet::SystemAddress)")]
pub fn stub_a63754() -> ! {
    todo!("0xa63754 RakNet::RakPeer::SetTimeoutTime(unsigned int,RakNet::SystemAddress)")
}

// 0xa63844 — __ZN6RakNet7RakPeer14GetTimeoutTimeENS_13SystemAddressE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::GetTimeoutTime(RakNet::SystemAddress)")]
pub fn stub_a63844() -> ! {
    todo!("0xa63844 RakNet::RakPeer::GetTimeoutTime(RakNet::SystemAddress)")
}

// 0xa638fc — __ZNK6RakNet7RakPeer10GetMTUSizeENS_13SystemAddressE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RakNet::RakPeer::GetMTUSize(RakNet::SystemAddress)const")]
pub fn stub_a638fc() -> ! {
    todo!("0xa638fc RakNet::RakPeer::GetMTUSize(RakNet::SystemAddress)const")
}

// 0xa639b4 — __ZN6RakNet7RakPeer20GetNumberOfAddressesEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::GetNumberOfAddresses(void)")]
pub fn stub_a639b4() -> ! {
    todo!("0xa639b4 RakNet::RakPeer::GetNumberOfAddresses(void)")
}

// 0xa639e4 — __ZN6RakNet7RakPeer10GetLocalIPEj
// type: char *__fastcall(RakNet::RakPeer *this, unsigned int)
#[doc(alias = "RakNet::RakPeer::GetLocalIP(unsigned int)")]
pub fn stub_a639e4() -> ! {
    todo!("0xa639e4 RakNet::RakPeer::GetLocalIP(unsigned int)")
}

// 0xa63a28 — __ZN6RakNet7RakPeer9IsLocalIPEPKc
// type: int __fastcall(RakNet::RakPeer *this, const char *__s1)
#[doc(alias = "RakNet::RakPeer::IsLocalIP(char const*)")]
pub fn stub_a63a28() -> ! {
    todo!("0xa63a28 RakNet::RakPeer::IsLocalIP(char const*)")
}

// 0xa63aa8 — __ZN6RakNet7RakPeer34AllowConnectionResponseIPMigrationEb
// type: int __fastcall(int this, bool)
#[doc(alias = "RakNet::RakPeer::AllowConnectionResponseIPMigration(bool)")]
pub fn stub_a63aa8() -> ! {
    todo!("0xa63aa8 RakNet::RakPeer::AllowConnectionResponseIPMigration(bool)")
}

// 0xa63ab0 — __ZN6RakNet7RakPeer15AdvertiseSystemEPKctS2_ij
// type: int __fastcall(RakNet::RakPeer *this, const char *, int, const char *, size_t, unsigned int)
#[doc(alias = "RakNet::RakPeer::AdvertiseSystem(char const*,unsigned short,char const*,int,unsigned int)")]
pub fn stub_a63ab0() -> ! {
    todo!("0xa63ab0 RakNet::RakPeer::AdvertiseSystem(char const*,unsigned short,char const*,int,unsigned int)")
}

// 0xa63bd8 — __ZN6RakNet7RakPeer31SetSplitMessageProgressIntervalEi
// type: unsigned int __fastcall(RakNet::RakPeer *this, int)
#[doc(alias = "RakNet::RakPeer::SetSplitMessageProgressInterval(int)")]
pub fn stub_a63bd8() -> ! {
    todo!("0xa63bd8 RakNet::RakPeer::SetSplitMessageProgressInterval(int)")
}

// 0xa63c14 — __ZNK6RakNet7RakPeer31GetSplitMessageProgressIntervalEv
// type: int __fastcall(RakNet::RakPeer *this)
#[doc(alias = "RakNet::RakPeer::GetSplitMessageProgressInterval(void)const")]
pub fn stub_a63c14() -> ! {
    todo!("0xa63c14 RakNet::RakPeer::GetSplitMessageProgressInterval(void)const")
}

// 0xa63c1c — __ZN6RakNet7RakPeer20SetUnreliableTimeoutEj
// type: unsigned int __fastcall(RakNet::RakPeer *this, unsigned int)
#[doc(alias = "RakNet::RakPeer::SetUnreliableTimeout(unsigned int)")]
pub fn stub_a63c1c() -> ! {
    todo!("0xa63c1c RakNet::RakPeer::SetUnreliableTimeout(unsigned int)")
}

// 0xa63c58 — __ZN6RakNet7RakPeer7SendTTLEPKctij
// type: int __fastcall(RakNet::RakPeer *this, const char *, unsigned __int16, RakNet::SystemAddress *, unsigned int)
#[doc(alias = "RakNet::RakPeer::SendTTL(char const*,unsigned short,int,unsigned int)")]
pub fn stub_a63c58() -> ! {
    todo!("0xa63c58 RakNet::RakPeer::SendTTL(char const*,unsigned short,int,unsigned int)")
}

// 0xa63cf8 — __ZN6RakNet7RakPeer12AttachPluginEPNS_16PluginInterface2E
// type: unsigned int __fastcall(RakNet::RakPeer *this, RakNet::PluginInterface2 *)
#[doc(alias = "RakNet::RakPeer::AttachPlugin(RakNet::PluginInterface2 *)")]
pub fn stub_a63cf8() -> ! {
    todo!("0xa63cf8 RakNet::RakPeer::AttachPlugin(RakNet::PluginInterface2 *)")
}
