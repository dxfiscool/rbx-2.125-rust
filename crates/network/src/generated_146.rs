//! Auto-generated skeletons for rbx-network — RBX::Network|RakNet filtered EA-sorted ascending
//! Filter: RakNet|RBX::Network -> 4479 funcs, 2939 already stubbed (1540 remaining before batch); continuing filtered ascending (smallest missing first)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0xa0ef50..0xa23424 | existing 2939 -> 3039 filtered total (out of 4479), rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xa0ef50 — __ZN3RBX7Network7Players13OnReceiveChatEPNS0_6PlayerEPN6RakNet16RakPeerInterfaceEPNS4_6PacketEh
// demangled: RBX::Network::Players::OnReceiveChat(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *,unsigned char)
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *, pthread_mutex_t *, int, int, int)
#[doc(alias = "RBX::Network::Players::OnReceiveChat(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *,unsigned char)")]
pub fn stub_a0ef50() -> ! {
    todo!("0xa0ef50 RBX::Network::Players::OnReceiveChat(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *,unsigned char)")
}

// 0xa11b88 — __ZN3RBX7Network7Players18contentFilterAsyncESsSsPN6RakNet6PacketE
// demangled: RBX::Network::Players::contentFilterAsync(std::string,std::string,RakNet::Packet *)
// type: void __fastcall(int, const std::string *, const std::string *, int, int, int, int, int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Network::Players::contentFilterAsync(std::string,std::string,RakNet::Packet *)")]
pub fn stub_a11b88() -> ! {
    todo!("0xa11b88 RBX::Network::Players::contentFilterAsync(std::string,std::string,RakNet::Packet *)")
}

// 0xa12108 — __ZN3RBX7Network7Players20OnReceiveReportAbuseEPNS0_6PlayerEPN6RakNet16RakPeerInterfaceEPNS4_6PacketE
// demangled: RBX::Network::Players::OnReceiveReportAbuse(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *)
// type: int __fastcall(pthread_mutex_t *, int, int, int)
#[doc(alias = "RBX::Network::Players::OnReceiveReportAbuse(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *)")]
pub fn stub_a12108() -> ! {
    todo!("0xa12108 RBX::Network::Players::OnReceiveReportAbuse(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *)")
}

// 0xa12c94 — __ZNK3RBX7Network7Players17isMessageFilteredERKSsS3_
// demangled: RBX::Network::Players::isMessageFiltered(std::string const&,std::string const&)const
// type: bool __fastcall(RBX::Network::Players *this, const std::string *, const std::string *)
#[doc(alias = "RBX::Network::Players::isMessageFiltered(std::string const&,std::string const&)const")]
pub fn stub_a12c94() -> ! {
    todo!("0xa12c94 RBX::Network::Players::isMessageFiltered(std::string const&,std::string const&)const")
}

// 0xa12fb0 — __ZNK3RBX7Network7Players14getLoadDataUrlEi
// demangled: RBX::Network::Players::getLoadDataUrl(int)const
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getLoadDataUrl(int)const")]
pub fn stub_a12fb0(template: &str, user_id: i32) -> String {
    // IDA 0xa12fb0: empty template throws `"No LoadData url set"`; else the template formatted with the user id.
    crate::player::load_data_url(template, user_id)
}

// 0xa13104 — __ZNK3RBX7Network7Players14getSaveDataUrlEi
// demangled: RBX::Network::Players::getSaveDataUrl(int)const
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getSaveDataUrl(int)const")]
pub fn stub_a13104() -> ! {
    todo!("0xa13104 RBX::Network::Players::getSaveDataUrl(int)const")
}

// 0xa13258 — __ZNK3RBX7Network7Players25getSaveLeaderboardDataUrlEi
// demangled: RBX::Network::Players::getSaveLeaderboardDataUrl(int)const
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getSaveLeaderboardDataUrl(int)const")]
pub fn stub_a13258() -> ! {
    todo!("0xa13258 RBX::Network::Players::getSaveLeaderboardDataUrl(int)const")
}

// 0xa133ac — __ZNK3RBX7Network7Players17hasLeaderboardKeyERKSs
// demangled: RBX::Network::Players::hasLeaderboardKey(std::string const&)const
// type: bool __fastcall(RBX::Network::Players *this, const void **)
#[doc(alias = "RBX::Network::Players::hasLeaderboardKey(std::string const&)const")]
pub fn stub_a133ac() -> ! {
    todo!("0xa133ac RBX::Network::Players::hasLeaderboardKey(std::string const&)const")
}

// 0xa13478 — __ZNK3RBX7Network7Players19beginLeaderboardKeyEv
// demangled: RBX::Network::Players::beginLeaderboardKey(void)const
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::beginLeaderboardKey(void)const")]
pub fn stub_a13478() -> ! {
    todo!("0xa13478 RBX::Network::Players::beginLeaderboardKey(void)const")
}

// 0xa13498 — __ZNK3RBX7Network7Players17endLeaderboardKeyEv
// demangled: RBX::Network::Players::endLeaderboardKey(void)const
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::endLeaderboardKey(void)const")]
pub fn stub_a13498() -> ! {
    todo!("0xa13498 RBX::Network::Players::endLeaderboardKey(void)const")
}

// 0xa1349c — __ZN3RBX7Network7Players16friendEventFiredEiiNS_13FriendService15FriendEventTypeE
// demangled: RBX::Network::Players::friendEventFired(int,int,RBX::FriendService::FriendEventType)
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Players::friendEventFired(int,int,RBX::FriendService::FriendEventType)")]
pub fn stub_a1349c() -> ! {
    todo!("0xa1349c RBX::Network::Players::friendEventFired(int,int,RBX::FriendService::FriendEventType)")
}

// 0xa13c7c — __ZN3RBX7Network7Players13getPlayerByIDEi
// demangled: RBX::Network::Players::getPlayerByID(int)
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getPlayerByID(int)")]
pub fn stub_a13c7c() -> ! {
    todo!("0xa13c7c RBX::Network::Players::getPlayerByID(int)")
}

// 0xa14074 — __ZN3RBX7Network7Players19friendStatusChangedEiiNS_13FriendService12FriendStatusE
// demangled: RBX::Network::Players::friendStatusChanged(int,int,RBX::FriendService::FriendStatus)
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Players::friendStatusChanged(int,int,RBX::FriendService::FriendStatus)")]
pub fn stub_a14074() -> ! {
    todo!("0xa14074 RBX::Network::Players::friendStatusChanged(int,int,RBX::FriendService::FriendStatus)")
}

// 0xa14640 — __ZN3RBX7Network7Players20friendServiceRequestEbN5boost8weak_ptrINS0_6PlayerEEEi
// demangled: RBX::Network::Players::friendServiceRequest(bool,boost::weak_ptr<RBX::Network::Player>,int)
// type: void __fastcall(RBX::ServiceProvider *, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::Players::friendServiceRequest(bool,rbx_core::WeakPtr<RBX::Network::Player>,int)")]
pub fn stub_a14640() -> ! {
    todo!("0xa14640 RBX::Network::Players::friendServiceRequest(bool,boost::weak_ptr<RBX::Network::Player>,int)")
}

// 0xa14aa0 — __ZNK3RBX7Network7Players11askAddChildEPKNS_8InstanceE
// demangled: RBX::Network::Players::askAddChild(RBX::Instance const*)const
// type: bool __fastcall(RBX::Network::Players *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::askAddChild(RBX::Instance const*)const")]
pub fn stub_a14aa0() -> ! {
    todo!("0xa14aa0 RBX::Network::Players::askAddChild(RBX::Instance const*)const")
}

// 0xa14bec — __ZN3RBX7Network7Players18findLocalCharacterEPNS_8InstanceE
// demangled: RBX::Network::Players::findLocalCharacter(RBX::Instance *)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findLocalCharacter(RBX::Instance *)")]
pub fn stub_a14bec() -> ! {
    todo!("0xa14bec RBX::Network::Players::findLocalCharacter(RBX::Instance *)")
}

// 0xa14c18 — __ZN3RBX7Network7Players15findLocalPlayerEPNS_8InstanceE
// demangled: RBX::Network::Players::findLocalPlayer(RBX::Instance *)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findLocalPlayer(RBX::Instance *)")]
pub fn stub_a14c18() -> ! {
    todo!("0xa14c18 RBX::Network::Players::findLocalPlayer(RBX::Instance *)")
}

// 0xa14c40 — __ZN3RBX7Network7Players23findConstLocalCharacterEPKNS_8InstanceE
// demangled: RBX::Network::Players::findConstLocalCharacter(RBX::Instance const*)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findConstLocalCharacter(RBX::Instance const*)")]
pub fn stub_a14c40() -> ! {
    todo!("0xa14c40 RBX::Network::Players::findConstLocalCharacter(RBX::Instance const*)")
}

// 0xa14c6c — __ZN3RBX7Network7Players20findConstLocalPlayerEPKNS_8InstanceE
// demangled: RBX::Network::Players::findConstLocalPlayer(RBX::Instance const*)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findConstLocalPlayer(RBX::Instance const*)")]
pub fn stub_a14c6c() -> ! {
    todo!("0xa14c6c RBX::Network::Players::findConstLocalPlayer(RBX::Instance const*)")
}

// 0xa14c94 — __ZN3RBX7Network7Players18findAncestorPlayerEPKNS_8InstanceE
// demangled: RBX::Network::Players::findAncestorPlayer(RBX::Instance const*)
// type: void __fastcall(RBX::Network::Players *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findAncestorPlayer(RBX::Instance const*)")]
pub fn stub_a14c94() -> ! {
    todo!("0xa14c94 RBX::Network::Players::findAncestorPlayer(RBX::Instance const*)")
}

// 0xa1526c — __ZN3RBX7Network7Players22getPlayerFromCharacterEPNS_8InstanceE
// demangled: RBX::Network::Players::getPlayerFromCharacter(RBX::Instance *)
// type: int __fastcall(RBX::Network::Players *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::getPlayerFromCharacter(RBX::Instance *)")]
pub fn stub_a1526c() -> ! {
    todo!("0xa1526c RBX::Network::Players::getPlayerFromCharacter(RBX::Instance *)")
}

// 0xa15560 — __ZN3RBX7Network7Players20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Network::Players::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int, int, int, int, int, int, __guard *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::Players::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_a15560() -> ! {
    todo!("0xa15560 RBX::Network::Players::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)")
}

// 0xa15700 — __ZN3RBX7Network7Players15onChildRemovingEPNS_8InstanceE
// demangled: RBX::Network::Players::onChildRemoving(RBX::Instance *)
// type: void __fastcall(RBX::Network::Players *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::onChildRemoving(RBX::Instance *)")]
pub fn stub_a15700() -> ! {
    todo!("0xa15700 RBX::Network::Players::onChildRemoving(RBX::Instance *)")
}

// 0xa16238 — __ZN3RBX7Network7Players25reportScriptSecurityErrorEiSsSsSs
// demangled: RBX::Network::Players::reportScriptSecurityError(int,std::string,std::string,std::string)
#[doc(alias = "RBX::Network::Players::reportScriptSecurityError(int,std::string,std::string,std::string)")]
pub fn stub_a16238() -> ! {
    todo!("0xa16238 RBX::Network::Players::reportScriptSecurityError(int,std::string,std::string,std::string)")
}

// 0xa1624c — __ZN3RBX7Network7Players24remoteInsertResultHelperEN5boost8weak_ptrIS1_EENS2_10shared_ptrINS_8InstanceEEEN3G3D7Vector3E
// demangled: RBX::Network::Players::remoteInsertResultHelper(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3)
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::Players::remoteInsertResultHelper(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3)")]
pub fn stub_a1624c() -> ! {
    todo!("0xa1624c RBX::Network::Players::remoteInsertResultHelper(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3)")
}

// 0xa16648 — __ZN3RBX7Network7Players18remoteInsertResultEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3E
// demangled: RBX::Network::Players::remoteInsertResult(boost::shared_ptr<RBX::Instance>,G3D::Vector3)
// type: void __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::Players::remoteInsertResult(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3)")]
pub fn stub_a16648() -> ! {
    todo!("0xa16648 RBX::Network::Players::remoteInsertResult(boost::shared_ptr<RBX::Instance>,G3D::Vector3)")
}

// 0xa168dc — __ZN3RBX7Network7Players12remoteInsertEiSsN3G3D7Vector3E
// demangled: RBX::Network::Players::remoteInsert(int,std::string,G3D::Vector3)
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, const std::string *, struct _Unwind_Exception *, int, int)
#[doc(alias = "RBX::Network::Players::remoteInsert(int,std::string,G3D::Vector3)")]
pub fn stub_a168dc() -> ! {
    todo!("0xa168dc RBX::Network::Players::remoteInsert(int,std::string,G3D::Vector3)")
}

// 0xa16cb0 — __ZN3RBX7Network7Players10killPlayerEi
// demangled: RBX::Network::Players::killPlayer(int)
// type: void __fastcall(RBX::Network::Players *this, int)
#[doc(alias = "RBX::Network::Players::killPlayer(int)")]
pub fn stub_a16cb0() -> ! {
    todo!("0xa16cb0 RBX::Network::Players::killPlayer(int)")
}

// 0xa16fa4 — __ZN3RBX7Network7Players16disconnectPlayerERNS_8InstanceEi
// demangled: RBX::Network::Players::disconnectPlayer(RBX::Instance &,int)
// type: void __fastcall(RBX::Network::Players *this, RBX::Instance *, int)
#[doc(alias = "RBX::Network::Players::disconnectPlayer(RBX::Instance &,int)")]
pub fn stub_a16fa4() -> ! {
    todo!("0xa16fa4 RBX::Network::Players::disconnectPlayer(RBX::Instance &,int)")
}

// 0xa172e4 — __ZN3RBX7Network7Players16disconnectPlayerEi
// demangled: RBX::Network::Players::disconnectPlayer(int)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, int)
#[doc(alias = "RBX::Network::Players::disconnectPlayer(int)")]
pub fn stub_a172e4() -> ! {
    todo!("0xa172e4 RBX::Network::Players::disconnectPlayer(int)")
}

// 0xa17304 — __ZN3RBX7Network7Players21disconnectPlayerLocalEi
// demangled: RBX::Network::Players::disconnectPlayerLocal(int)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, int)
#[doc(alias = "RBX::Network::Players::disconnectPlayerLocal(int)")]
pub fn stub_a17304() -> ! {
    todo!("0xa17304 RBX::Network::Players::disconnectPlayerLocal(int)")
}

// 0xa17324 — __ZN3RBX7Network7Players16onRemoteSysStatsEiRKSsS3_b
// demangled: RBX::Network::Players::onRemoteSysStats(int,std::string const&,std::string const&,bool)
// type: void __fastcall(RBX::Network::Players *this, uint32_t, const std::string *, const std::string *, int)
#[doc(alias = "RBX::Network::Players::onRemoteSysStats(int,std::string const&,std::string const&,bool)")]
pub fn stub_a17324() -> ! {
    todo!("0xa17324 RBX::Network::Players::onRemoteSysStats(int,std::string const&,std::string const&,bool)")
}

// 0xa18bc4 — __ZN3RBX7Network7Players12onChildAddedEPNS_8InstanceE
// demangled: RBX::Network::Players::onChildAdded(RBX::Instance *)
// type: void __fastcall(RBX::Network::Players *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::onChildAdded(RBX::Instance *)")]
pub fn stub_a18bc4() -> ! {
    todo!("0xa18bc4 RBX::Network::Players::onChildAdded(RBX::Instance *)")
}

// 0xa1a480 — __ZN3RBX7Network7Players17buildClientRegionERNS_7Region2E
// demangled: RBX::Network::Players::buildClientRegion(RBX::Region2 &)
// type: RBX::Network::Player *__fastcall(RBX::Network::Player **this, RBX::Region2 *)
#[doc(alias = "RBX::Network::Players::buildClientRegion(RBX::Region2 &)")]
pub fn stub_a1a480() -> ! {
    todo!("0xa1a480 RBX::Network::Players::buildClientRegion(RBX::Region2 &)")
}

// 0xa1a504 — __ZN3RBX7Network7Players21renderDPhysicsRegionsEPNS_5AdornE
// demangled: RBX::Network::Players::renderDPhysicsRegions(RBX::Adorn *)
// type: void __fastcall(RBX::Network::Players *this, RBX::Adorn *)
#[doc(alias = "RBX::Network::Players::renderDPhysicsRegions(RBX::Adorn *)")]
pub fn stub_a1a504() -> ! {
    todo!("0xa1a504 RBX::Network::Players::renderDPhysicsRegions(RBX::Adorn *)")
}

// 0xa1a77c — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)")]
pub fn stub_a1a77c() -> ! {
    todo!("0xa1a77c RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)")
}

// 0xa1a788 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)")]
pub fn stub_a1a788() -> ! {
    todo!("0xa1a788 RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)")
}

// 0xa1a9b0 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)")]
pub fn stub_a1a9b0() -> ! {
    todo!("0xa1a9b0 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)")
}

// 0xa1a9bc — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)")]
pub fn stub_a1a9bc() -> ! {
    todo!("0xa1a9bc RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)")
}

// 0xa1abe4 — __ZN3RBX15StringConverterINS_7Network7Players10ChatOptionEE14convertToValueERKSsRS3_
// demangled: RBX::StringConverter<RBX::Network::Players::ChatOption>::convertToValue(std::string const&,RBX::Network::Players::ChatOption&)
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "RBX::StringConverter<RBX::Network::Players::ChatOption>::convertToValue(std::string const&,RBX::Network::Players::ChatOption&)")]
pub fn stub_a1abe4() -> ! {
    todo!("0xa1abe4 RBX::StringConverter<RBX::Network::Players::ChatOption>::convertToValue(std::string const&,RBX::Network::Players::ChatOption&)")
}

// 0xa1adb8 — __ZNK3RBX7Network7Players13getNumPlayersEv
// demangled: RBX::Network::Players::getNumPlayers(void)const
// type: int __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getNumPlayers(void)const")]
pub fn stub_a1adb8() -> ! {
    todo!("0xa1adb8 RBX::Network::Players::getNumPlayers(void)const")
}

// 0xa1ae1c — __ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEiED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Network::Players,int>::~PropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,int>::~PropDescriptor()")]
pub fn stub_a1ae1c() -> ! {
    todo!("0xa1ae1c RBX::Reflection::PropDescriptor<RBX::Network::Players,int>::~PropDescriptor()")
}

// 0xa1ae40 — __ZNK3RBX7Network7Players13getMaxPlayersEv
// demangled: RBX::Network::Players::getMaxPlayers(void)const
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getMaxPlayers(void)const")]
pub fn stub_a1ae40() -> ! {
    todo!("0xa1ae40 RBX::Network::Players::getMaxPlayers(void)const")
}

// 0xa1ae48 — __ZN3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEED1Ev
// demangled: RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::~RefPropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::~RefPropDescriptor()")]
pub fn stub_a1ae48() -> ! {
    todo!("0xa1ae48 RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::~RefPropDescriptor()")
}

// 0xa1ae74 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(int),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::~BoundFuncDesc()")]
pub fn stub_a1ae74() -> ! {
    todo!("0xa1ae74 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(int),1>::~BoundFuncDesc()")
}

// 0xa1aedc — __ZNK3RBX7Network7Players21getCharacterAutoSpawnEv
// demangled: RBX::Network::Players::getCharacterAutoSpawn(void)const
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getCharacterAutoSpawn(void)const")]
pub fn stub_a1aedc() -> ! {
    todo!("0xa1aedc RBX::Network::Players::getCharacterAutoSpawn(void)const")
}

// 0xa1aee4 — __ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::~PropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::~PropDescriptor()")]
pub fn stub_a1aee4() -> ! {
    todo!("0xa1aee4 RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::~PropDescriptor()")
}

// 0xa1af08 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_a1af08() -> ! {
    todo!("0xa1af08 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0xa1afb0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
// type: int()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
pub fn stub_a1afb0() -> ! {
    todo!("0xa1afb0 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()")
}

// 0xa1afbc — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
pub fn stub_a1afbc() -> ! {
    todo!("0xa1afbc RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")
}

// 0xa1b004 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::~EventDesc()")]
pub fn stub_a1b004() -> ! {
    todo!("0xa1b004 RBX::Reflection::EventDesc<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::~EventDesc()")
}

// 0xa1b04c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()
// type: int()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")]
pub fn stub_a1b04c() -> ! {
    todo!("0xa1b04c RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")
}

// 0xa1b058 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC1EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Network::Players::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, __guard *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Network::Players::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_a1b058() -> ! {
    todo!("0xa1b058 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Network::Players::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xa1b218 — __ZN3RBX7Network7Players10getPlayersEv
// demangled: RBX::Network::Players::getPlayers(void)
// type: _DWORD *__fastcall(_DWORD *this, int)
#[doc(alias = "RBX::Network::Players::getPlayers(void)")]
pub fn stub_a1b218() -> ! {
    todo!("0xa1b218 RBX::Network::Players::getPlayers(void)")
}

// 0xa1b26c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_a1b26c() -> ! {
    todo!("0xa1b26c RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")
}

// 0xa1b2b4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_a1b2b4() -> ! {
    todo!("0xa1b2b4 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0xa1b2c0 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()")]
pub fn stub_a1b2c0() -> ! {
    todo!("0xa1b2c0 RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()")
}

// 0xa1b308 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
pub fn stub_a1b308() -> ! {
    todo!("0xa1b308 RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")
}

// 0xa1b350 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(RBX::Network::Players::ChatOption),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(RBX::Network::Players::ChatOption),1>::~BoundFuncDesc()")]
pub fn stub_a1b350() -> ! {
    todo!("0xa1b350 RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(RBX::Network::Players::ChatOption),1>::~BoundFuncDesc()")
}

// 0xa1b3b8 — __ZNK3RBX7Network7Players14getClassicChatEv
// demangled: RBX::Network::Players::getClassicChat(void)const
// type: bool __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getClassicChat(void)const")]
pub fn stub_a1b3b8() -> ! {
    todo!("0xa1b3b8 RBX::Network::Players::getClassicChat(void)const")
}

// 0xa1b3cc — __ZNK3RBX7Network7Players13getBubbleChatEv
// demangled: RBX::Network::Players::getBubbleChat(void)const
// type: bool __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getBubbleChat(void)const")]
pub fn stub_a1b3cc() -> ! {
    todo!("0xa1b3cc RBX::Network::Players::getBubbleChat(void)const")
}

// 0xa1b3e0 — __ZN3RBX7Network12NetworkOwner10UnassignedEv
// demangled: RBX::Network::NetworkOwner::Unassigned(void)
// type: __int64 __fastcall(RBX::Network::NetworkOwner *this)
#[doc(alias = "RBX::Network::NetworkOwner::Unassigned(void)")]
pub fn stub_a1b3e0() -> ! {
    todo!("0xa1b3e0 RBX::Network::NetworkOwner::Unassigned(void)")
}

// 0xa1b6b0 — __ZN5boost4bindIN3RBX13worker_thread11work_resultENS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsS8_SsEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// demangled: boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list_av_2<boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string>::type> boost::bind<RBX::worker_thread::work_result,boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string,boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string>(RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string)
// type: void __fastcall(int, int, int *, const std::string *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string>::type> boost::bind<RBX::worker_thread::work_result,rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string,rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string>(RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string)")]
pub fn stub_a1b6b0() -> ! {
    todo!("0xa1b6b0 boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list_av_2<boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string>::type> boost::bind<RBX::worker_thread::work_result,boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string,boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string>(RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string)")
}

// 0xa1bc50 — __ZSt8for_eachISt20_List_const_iteratorIN3RBX7Network11ChatMessageEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS2_11AbuseReportENS5_10shared_ptrINS2_6PlayerEEERKS3_EENS6_5list3INS5_17reference_wrapperISA_EENS6_5valueISD_EENS5_3argILi1EEEEEEEET0_T_SR_SQ_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>> std::for_each<std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>>>(std::_List_const_iterator<RBX::Network::ChatMessage>,std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>>)
// type: void __fastcall(int *, int, int, int *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>> std::for_each<std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>>>(std::_List_const_iterator<RBX::Network::ChatMessage>,std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>>)")]
pub fn stub_a1bc50() -> ! {
    todo!("0xa1bc50 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>> std::for_each<std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>>>(std::_List_const_iterator<RBX::Network::ChatMessage>,std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>>)")
}

// 0xa1bf30 — __ZN5boost4bindIvN3RBX7Network11AbuseReportENS_10shared_ptrINS2_6PlayerEEERKNS2_11ChatMessageENS_17reference_wrapperIS3_EES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list_av_3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&,boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>>(void (RBX::Network::AbuseReport::*)(boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&),boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>)
// type: void __fastcall(int, pthread_mutex_t *, int, int, pthread_mutex_t **)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list_av_3<boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&,boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>>(void (RBX::Network::AbuseReport::*)(rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&),boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>)")]
pub fn stub_a1bf30() -> ! {
    todo!("0xa1bf30 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list_av_3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&,boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>>(void (RBX::Network::AbuseReport::*)(boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&),boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>)")
}

// 0xa1c8e8 — __ZN3RBX10Reflection13DescribedBase21fastSharedDynamicCastINS_7Network6PlayerENS_8InstanceEEEN5boost10shared_ptrIT_EERKNS7_IT0_EE
// demangled: boost::shared_ptr<RBX::Network::Player> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::Network::Player,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
// type: void __fastcall(_DWORD *, _DWORD *, int, int, int, int, int, int, int, __guard *, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::Network::Player,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_a1c8e8() -> ! {
    todo!("0xa1c8e8 boost::shared_ptr<RBX::Network::Player> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::Network::Player,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)")
}

// 0xa1ced4 — __ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EEclES5_S9_SsS9_
// demangled: rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::operator()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
// type: void __fastcall(_DWORD *, int, int *, std::string *, int *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_a1ced4() -> ! {
    todo!("0xa1ced4 rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::operator()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)")
}

// 0xa1d83c — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7Network11ChatMessageEEEclES6_
// demangled: rbx::signals::signal_with_args<1,void ()(RBX::Network::ChatMessage const&)>::operator()(RBX::Network::ChatMessage const&)
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::ChatMessage const&)>::operator()(RBX::Network::ChatMessage const&)")]
pub fn stub_a1d83c() -> ! {
    todo!("0xa1d83c rbx::signals::signal_with_args<1,void ()(RBX::Network::ChatMessage const&)>::operator()(RBX::Network::ChatMessage const&)")
}

// 0xa1daf8 — __ZN3RBX11shared_fromINS_7Network6PlayerEEEN5boost10shared_ptrIT_EEPS5_
// demangled: boost::shared_ptr<RBX::Network::Player> RBX::shared_from<RBX::Network::Player>(RBX::Network::Player*)
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::shared_from<RBX::Network::Player>(RBX::Network::Player*)")]
pub fn stub_a1daf8() -> ! {
    todo!("0xa1daf8 boost::shared_ptr<RBX::Network::Player> RBX::shared_from<RBX::Network::Player>(RBX::Network::Player*)")
}

// 0xa1dd8c — __ZN3RBX32shared_from_polymorphic_downcastINS_7Network6PlayerENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE
// demangled: boost::shared_ptr<RBX::Network::Player> RBX::shared_from_polymorphic_downcast<RBX::Network::Player,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)
// type: void __fastcall(int, _DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::shared_from_polymorphic_downcast<RBX::Network::Player,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
pub fn stub_a1dd8c() -> ! {
    todo!("0xa1dd8c boost::shared_ptr<RBX::Network::Player> RBX::shared_from_polymorphic_downcast<RBX::Network::Player,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")
}

// 0xa1e020 — __ZN5boost4bindIvN3RBX7Network7PlayersESsSsPN6RakNet6PacketENS_10shared_ptrIS3_EESsSsS6_EENS_3_bi6bind_tIT_NS_4_mfi3mf3ISB_T0_T1_T2_T3_EENS9_9list_av_4IT4_T5_T6_T7_E4typeEEEMSE_FSB_SF_SG_SH_ESK_SL_SM_SN_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list_av_4<boost::shared_ptr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>::type> boost::bind<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *,boost::shared_ptr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>(void (RBX::Network::Players::*)(std::string,std::string,RakNet::Packet *),boost::shared_ptr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *)
// type: void __fastcall(_DWORD *, int, int, int *, std::string *, std::string *, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>::type> boost::bind<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *,rbx_core::SharedPtr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>(void (RBX::Network::Players::*)(std::string,std::string,RakNet::Packet *),rbx_core::SharedPtr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *)")]
pub fn stub_a1e020() -> ! {
    todo!("0xa1e020 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list_av_4<boost::shared_ptr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>::type> boost::bind<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *,boost::shared_ptr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>(void (RBX::Network::Players::*)(std::string,std::string,RakNet::Packet *),boost::shared_ptr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *)")
}

// 0xa1e654 — __ZN3RBX11shared_fromINS_7Network7PlayersEEEN5boost10shared_ptrIT_EEPS5_
// demangled: boost::shared_ptr<RBX::Network::Players> RBX::shared_from<RBX::Network::Players>(RBX::Network::Players*)
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Players> RBX::shared_from<RBX::Network::Players>(RBX::Network::Players*)")]
pub fn stub_a1e654() -> ! {
    todo!("0xa1e654 boost::shared_ptr<RBX::Network::Players> RBX::shared_from<RBX::Network::Players>(RBX::Network::Players*)")
}

// 0xa1eab0 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX7Network11AbuseReportEEEclES4_
// demangled: rbx::signals::signal_with_args<1,void ()(RBX::Network::AbuseReport)>::operator()(RBX::Network::AbuseReport)
// type: void __fastcall(_DWORD *, int *, int, const void *)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::AbuseReport)>::operator()(RBX::Network::AbuseReport)")]
pub fn stub_a1eab0() -> ! {
    todo!("0xa1eab0 rbx::signals::signal_with_args<1,void ()(RBX::Network::AbuseReport)>::operator()(RBX::Network::AbuseReport)")
}

// 0xa1f558 — __ZN3RBX10Reflection13DescribedBase15fastDynamicCastINS_7Network6PlayerEEEPT_PS1_
// demangled: RBX::Network::Player * RBX::Reflection::DescribedBase::fastDynamicCast<RBX::Network::Player>(RBX::Reflection::DescribedBase*)
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Network::Player * RBX::Reflection::DescribedBase::fastDynamicCast<RBX::Network::Player>(RBX::Reflection::DescribedBase*)")]
pub fn stub_a1f558() -> ! {
    todo!("0xa1f558 RBX::Network::Player * RBX::Reflection::DescribedBase::fastDynamicCast<RBX::Network::Player>(RBX::Reflection::DescribedBase*)")
}

// 0xa1ff60 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS2_8InstanceEEEN3G3D7Vector3ES5_NS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
// demangled: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)
// type: void __fastcall(int, int, int *, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)")]
pub fn stub_a1ff60() -> ! {
    todo!("0xa1ff60 boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)")
}

// 0xa20280 — __ZN3RBX9weak_fromINS_7Network7PlayersEEEN5boost8weak_ptrIT_EEPS5_
// demangled: boost::weak_ptr<RBX::Network::Players> RBX::weak_from<RBX::Network::Players>(RBX::Network::Players*)
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Players> RBX::weak_from<RBX::Network::Players>(RBX::Network::Players*)")]
pub fn stub_a20280() -> ! {
    todo!("0xa20280 boost::weak_ptr<RBX::Network::Players> RBX::weak_from<RBX::Network::Players>(RBX::Network::Players*)")
}

// 0xa207bc — __ZN5boost4bindIvN3RBX7Network7PlayersEbNS_8weak_ptrINS2_6PlayerEEEiPS3_NS_3argILi1EEES6_NS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISD_T0_T1_T2_T3_EENSB_9list_av_4IT4_T5_T6_T7_E4typeEEEMSG_FSD_SH_SI_SJ_ESM_SN_SO_SP_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list_av_4<RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>>::type> boost::bind<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int,RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>>(void (RBX::Network::Players::*)(bool,boost::weak_ptr<RBX::Network::Player>,int),RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>)
// type: void __fastcall(_DWORD *, int, int, int, int *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list_av_4<RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>>::type> boost::bind<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int,RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>>(void (RBX::Network::Players::*)(bool,rbx_core::WeakPtr<RBX::Network::Player>,int),RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>)")]
pub fn stub_a207bc() -> ! {
    todo!("0xa207bc boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list_av_4<RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>>::type> boost::bind<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int,RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>>(void (RBX::Network::Players::*)(bool,boost::weak_ptr<RBX::Network::Player>,int),RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>)")
}

// 0xa20ac8 — __ZN3RBX9weak_fromINS_7Network6PlayerEEEN5boost8weak_ptrIT_EEPS5_
// demangled: boost::weak_ptr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)")]
pub fn stub_a20ac8() -> ! {
    todo!("0xa20ac8 boost::weak_ptr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)")
}

// 0xa2133c — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE7addPairES4_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::addPair(RBX::Network::Players::ChatOption,char const*)
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::addPair(RBX::Network::Players::ChatOption,char const*)")]
pub fn stub_a2133c() -> ! {
    todo!("0xa2133c RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::addPair(RBX::Network::Players::ChatOption,char const*)")
}

// 0xa21864 — __ZN3RBX10Reflection7Variant14genericConvertINS_7Network7Players10ChatOptionEEERT_v
// demangled: RBX::Network::Players::ChatOption & RBX::Reflection::Variant::genericConvert<RBX::Network::Players::ChatOption>(void)
// type: int __fastcall(int)
#[doc(alias = "RBX::Network::Players::ChatOption & RBX::Reflection::Variant::genericConvert<RBX::Network::Players::ChatOption>(void)")]
pub fn stub_a21864() -> ! {
    todo!("0xa21864 RBX::Network::Players::ChatOption & RBX::Reflection::Variant::genericConvert<RBX::Network::Players::ChatOption>(void)")
}

// 0xa21bb4 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE7addPairES4_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::addPair(RBX::Network::Players::PlayerChatType,char const*)
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::addPair(RBX::Network::Players::PlayerChatType,char const*)")]
pub fn stub_a21bb4() -> ! {
    todo!("0xa21bb4 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::addPair(RBX::Network::Players::PlayerChatType,char const*)")
}

// 0xa22504 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()")]
pub fn stub_a22504() -> ! {
    todo!("0xa22504 RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()")
}

// 0xa22510 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED0Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()")]
pub fn stub_a22510() -> ! {
    todo!("0xa22510 RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()")
}

// 0xa225b0 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE6lookupEPKc
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(char const*)const
// type: int __fastcall(_DWORD *, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(char const*)const")]
pub fn stub_a225b0() -> ! {
    todo!("0xa225b0 RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(char const*)const")
}

// 0xa22640 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE6lookupERKNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_a22640() -> ! {
    todo!("0xa22640 RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xa22744 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE14convertToValueEmRNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_a22744() -> ! {
    todo!("0xa22744 RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xa22804 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(unsigned long,std::string &)const
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(unsigned long,std::string &)const")]
pub fn stub_a22804() -> ! {
    todo!("0xa22804 RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(unsigned long,std::string &)const")
}

// 0xa22948 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")]
pub fn stub_a22948() -> ! {
    todo!("0xa22948 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")
}

// 0xa22954 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED0Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")]
pub fn stub_a22954() -> ! {
    todo!("0xa22954 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")
}

// 0xa229f4 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE6lookupEPKc
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(char const*)const
// type: int __fastcall(_DWORD *, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(char const*)const")]
pub fn stub_a229f4() -> ! {
    todo!("0xa229f4 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(char const*)const")
}

// 0xa22a84 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE6lookupERKNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_a22a84() -> ! {
    todo!("0xa22a84 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xa22b88 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE14convertToValueEmRNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_a22b88() -> ! {
    todo!("0xa22b88 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xa22c48 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(unsigned long,std::string &)const
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(unsigned long,std::string &)const")]
pub fn stub_a22c48() -> ! {
    todo!("0xa22c48 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(unsigned long,std::string &)const")
}

// 0xa22d8c — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringERKS4_
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(RBX::Network::Players::PlayerChatType const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(RBX::Network::Players::PlayerChatType const&)const")]
pub fn stub_a22d8c() -> ! {
    todo!("0xa22d8c RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(RBX::Network::Players::PlayerChatType const&)const")
}

// 0xa22f2c — __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players14PlayerChatTypeEE14construct_funcEPKcPc
// demangled: rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::construct_func(char const*,char *)
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::construct_func(char const*,char *)")]
pub fn stub_a22f2c() -> ! {
    todo!("0xa22f2c rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::construct_func(char const*,char *)")
}

// 0xa22f38 — __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players14PlayerChatTypeEE13destruct_funcEPc
// demangled: rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::destruct_func(char *)
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::destruct_func(char *)")]
pub fn stub_a22f38() -> ! {
    todo!("0xa22f38 rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::destruct_func(char *)")
}

// 0xa22f3c — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE13convertToItemERKS4_
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToItem(RBX::Network::Players::PlayerChatType const&)const
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToItem(RBX::Network::Players::PlayerChatType const&)const")]
pub fn stub_a22f3c() -> ! {
    todo!("0xa22f3c RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToItem(RBX::Network::Players::PlayerChatType const&)const")
}

// 0xa23008 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")]
pub fn stub_a23008() -> ! {
    todo!("0xa23008 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")
}

// 0xa23284 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringERKS4_
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(RBX::Network::Players::ChatOption const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(RBX::Network::Players::ChatOption const&)const")]
pub fn stub_a23284() -> ! {
    todo!("0xa23284 RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(RBX::Network::Players::ChatOption const&)const")
}

// 0xa23424 — __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players10ChatOptionEE14construct_funcEPKcPc
// demangled: rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::construct_func(char const*,char *)
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::construct_func(char const*,char *)")]
pub fn stub_a23424() -> ! {
    todo!("0xa23424 rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::construct_func(char const*,char *)")
}
