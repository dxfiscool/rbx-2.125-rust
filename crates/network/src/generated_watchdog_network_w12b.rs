//! Auto-generated skeletons for rbx-network — watchdog w12b Network/RakNet/Replicator
//! Filter: demangled/mangled contains RBX::Network|RBX::Replicator|RakNet|Replicator, EA-sorted asc, continue after w12 (0xa0a4ec), take 120
//! NOTE: /tmp/global_eas.txt covers all 4797 network EAs in ida/export.json; stubs are UNIQUE vs crates/network/src (distinct EAs, no overlap with existing stubs), global overlap unavoidable
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0xa0ac84..0xa279b4 | EA-sorted asc distinct within crate
//! SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; // 0xADDR mangled + #[doc(alias)] + todo!("0xADDR")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0xa0ac84 — __ZN3RBX7Network13AbuseReporter15processRequestsEN5boost10shared_ptrINS1_4dataEEESs
// type: int __fastcall(struct _Unwind_Exception **, std::string *)
#[doc(alias = "RBX::Network::AbuseReporter::processRequests(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string)")]
#[doc(alias = "__ZN3RBX7Network13AbuseReporter15processRequestsEN5boost10shared_ptrINS1_4dataEEESs")]
pub fn stub_0xa0ac84() -> ! { todo!("0xa0ac84 __ZN3RBX7Network13AbuseReporter15processRequestsEN5boost10shared_ptrINS1_4dataEEESs") }

// 0xa0ba5c — __ZN3RBX7Network13AbuseReporter3addERNS0_11AbuseReportEN5boost10shared_ptrINS0_6PlayerEEERKSt4listINS0_11ChatMessageESaIS9_EE
// type: void __fastcall(int, _QWORD *, __int32 *, pthread_mutex_t **, int, pthread_mutex_t *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, boost::mutex *, char, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "RBX::Network::AbuseReporter::add(RBX::Network::AbuseReport &,boost::shared_ptr<RBX::Network::Player>,std::list<RBX::Network::ChatMessage,std::allocator<RBX::Network::ChatMessage>> const&)")]
#[doc(alias = "__ZN3RBX7Network13AbuseReporter3addERNS0_11AbuseReportEN5boost10shared_ptrINS0_6PlayerEEERKSt4listINS0_11ChatMessageESaIS9_EE")]
pub fn stub_0xa0ba5c() -> ! { todo!("0xa0ba5c __ZN3RBX7Network13AbuseReporter3addERNS0_11AbuseReportEN5boost10shared_ptrINS0_6PlayerEEERKSt4listINS0_11ChatMessageESaIS9_EE") }

// 0xa0c044 — __ZL12writeMessageRKN3RBX7Network11AbuseReport7MessageEP10XmlElement
// type: void __fastcall(int *, int)
#[doc(alias = "writeMessage(RBX::Network::AbuseReport::Message const&,XmlElement *)")]
#[doc(alias = "__ZL12writeMessageRKN3RBX7Network11AbuseReport7MessageEP10XmlElement")]
pub fn stub_0xa0c044() -> ! { todo!("0xa0c044 __ZL12writeMessageRKN3RBX7Network11AbuseReport7MessageEP10XmlElement") }

// 0xa0c340 — __ZN3RBX7Network7Players11reportAbuseEPNS0_6PlayerERKSs
// type: void __fastcall(RBX::Network::Players *this, RBX::Network::Player *, const std::string *)
#[doc(alias = "RBX::Network::Players::reportAbuse(RBX::Network::Player *,std::string const&)")]
#[doc(alias = "__ZN3RBX7Network7Players11reportAbuseEPNS0_6PlayerERKSs")]
pub fn stub_0xa0c340() -> ! { todo!("0xa0c340 __ZN3RBX7Network7Players11reportAbuseEPNS0_6PlayerERKSs") }

// 0xa0d110 — __ZN3RBX7Network7Players9checkChatERKSs
// type: void __fastcall(RBX::Network::Players *this, const std::string *)
#[doc(alias = "RBX::Network::Players::checkChat(std::string const&)")]
#[doc(alias = "__ZN3RBX7Network7Players9checkChatERKSs")]
pub fn stub_0xa0d110() -> ! { todo!("0xa0d110 __ZN3RBX7Network7Players9checkChatERKSs") }

// 0xa0d400 — __ZN3RBX7Network7Players15getGuidRegistryEv
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this)
#[doc(alias = "RBX::Network::Players::getGuidRegistry(void)")]
#[doc(alias = "__ZN3RBX7Network7Players15getGuidRegistryEv")]
pub fn stub_0xa0d400() -> ! { todo!("0xa0d400 __ZN3RBX7Network7Players15getGuidRegistryEv") }

// 0xa0d488 — __ZN3RBX7Network7Players22raiseChatMessageSignalERKNS0_11ChatMessageE
// type: void __fastcall(RBX::Network::Players *this, struct _Unwind_Exception *, int, int)
#[doc(alias = "RBX::Network::Players::raiseChatMessageSignal(RBX::Network::ChatMessage const&)")]
#[doc(alias = "__ZN3RBX7Network7Players22raiseChatMessageSignalERKNS0_11ChatMessageE")]
pub fn stub_0xa0d488() -> ! { todo!("0xa0d488 __ZN3RBX7Network7Players22raiseChatMessageSignalERKNS0_11ChatMessageE") }

// 0xa0ded8 — __ZN3RBX7Network7Players24raisePlayerChattedSignalERKNS0_11ChatMessageE
// type: void __fastcall(RBX::Network::Players *this, const RBX::Network::ChatMessage *)
#[doc(alias = "RBX::Network::Players::raisePlayerChattedSignal(RBX::Network::ChatMessage const&)")]
#[doc(alias = "__ZN3RBX7Network7Players24raisePlayerChattedSignalERKNS0_11ChatMessageE")]
pub fn stub_0xa0ded8() -> ! { todo!("0xa0ded8 __ZN3RBX7Network7Players24raisePlayerChattedSignalERKNS0_11ChatMessageE") }

// 0xa0ee1c — __ZN3RBX7Network7Players14addChatMessageERKNS0_11ChatMessageE
// type: void __fastcall(std::_List_node_base **this, const RBX::Network::ChatMessage *)
#[doc(alias = "RBX::Network::Players::addChatMessage(RBX::Network::ChatMessage const&)")]
#[doc(alias = "__ZN3RBX7Network7Players14addChatMessageERKNS0_11ChatMessageE")]
pub fn stub_0xa0ee1c() -> ! { todo!("0xa0ee1c __ZN3RBX7Network7Players14addChatMessageERKNS0_11ChatMessageE") }

// 0xa12c94 — __ZNK3RBX7Network7Players17isMessageFilteredERKSsS3_
// type: bool __fastcall(RBX::Network::Players *this, const std::string *, const std::string *)
#[doc(alias = "RBX::Network::Players::isMessageFiltered(std::string const&,std::string const&)const")]
#[doc(alias = "__ZNK3RBX7Network7Players17isMessageFilteredERKSsS3_")]
pub fn stub_0xa12c94() -> ! { todo!("0xa12c94 __ZNK3RBX7Network7Players17isMessageFilteredERKSsS3_") }

// 0xa12fb0 — __ZNK3RBX7Network7Players14getLoadDataUrlEi
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getLoadDataUrl(int)const")]
#[doc(alias = "__ZNK3RBX7Network7Players14getLoadDataUrlEi")]
pub fn stub_0xa12fb0(template: &str, user_id: i32) -> String { // IDA 0xa12fb0: empty template throws `"No LoadData url set"`.
    crate::player::load_data_url(template, user_id)
}

// 0xa13104 — __ZNK3RBX7Network7Players14getSaveDataUrlEi
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getSaveDataUrl(int)const")]
#[doc(alias = "__ZNK3RBX7Network7Players14getSaveDataUrlEi")]
pub fn stub_0xa13104() -> ! { todo!("0xa13104 __ZNK3RBX7Network7Players14getSaveDataUrlEi") }

// 0xa13258 — __ZNK3RBX7Network7Players25getSaveLeaderboardDataUrlEi
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getSaveLeaderboardDataUrl(int)const")]
#[doc(alias = "__ZNK3RBX7Network7Players25getSaveLeaderboardDataUrlEi")]
pub fn stub_0xa13258() -> ! { todo!("0xa13258 __ZNK3RBX7Network7Players25getSaveLeaderboardDataUrlEi") }

// 0xa133ac — __ZNK3RBX7Network7Players17hasLeaderboardKeyERKSs
// type: bool __fastcall(RBX::Network::Players *this, const void **)
#[doc(alias = "RBX::Network::Players::hasLeaderboardKey(std::string const&)const")]
#[doc(alias = "__ZNK3RBX7Network7Players17hasLeaderboardKeyERKSs")]
pub fn stub_0xa133ac() -> ! { todo!("0xa133ac __ZNK3RBX7Network7Players17hasLeaderboardKeyERKSs") }

// 0xa13478 — __ZNK3RBX7Network7Players19beginLeaderboardKeyEv
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::beginLeaderboardKey(void)const")]
#[doc(alias = "__ZNK3RBX7Network7Players19beginLeaderboardKeyEv")]
pub fn stub_0xa13478() -> ! { todo!("0xa13478 __ZNK3RBX7Network7Players19beginLeaderboardKeyEv") }

// 0xa13498 — __ZNK3RBX7Network7Players17endLeaderboardKeyEv
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::endLeaderboardKey(void)const")]
#[doc(alias = "__ZNK3RBX7Network7Players17endLeaderboardKeyEv")]
pub fn stub_0xa13498() -> ! { todo!("0xa13498 __ZNK3RBX7Network7Players17endLeaderboardKeyEv") }

// 0xa1349c — __ZN3RBX7Network7Players16friendEventFiredEiiNS_13FriendService15FriendEventTypeE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Players::friendEventFired(int,int,RBX::FriendService::FriendEventType)")]
#[doc(alias = "__ZN3RBX7Network7Players16friendEventFiredEiiNS_13FriendService15FriendEventTypeE")]
pub fn stub_0xa1349c() -> ! { todo!("0xa1349c __ZN3RBX7Network7Players16friendEventFiredEiiNS_13FriendService15FriendEventTypeE") }

// 0xa13c7c — __ZN3RBX7Network7Players13getPlayerByIDEi
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getPlayerByID(int)")]
#[doc(alias = "__ZN3RBX7Network7Players13getPlayerByIDEi")]
pub fn stub_0xa13c7c() -> ! { todo!("0xa13c7c __ZN3RBX7Network7Players13getPlayerByIDEi") }

// 0xa14074 — __ZN3RBX7Network7Players19friendStatusChangedEiiNS_13FriendService12FriendStatusE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Players::friendStatusChanged(int,int,RBX::FriendService::FriendStatus)")]
#[doc(alias = "__ZN3RBX7Network7Players19friendStatusChangedEiiNS_13FriendService12FriendStatusE")]
pub fn stub_0xa14074() -> ! { todo!("0xa14074 __ZN3RBX7Network7Players19friendStatusChangedEiiNS_13FriendService12FriendStatusE") }

// 0xa14640 — __ZN3RBX7Network7Players20friendServiceRequestEbN5boost8weak_ptrINS0_6PlayerEEEi
// type: void __fastcall(RBX::ServiceProvider *, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Players::friendServiceRequest(bool,boost::weak_ptr<RBX::Network::Player>,int)")]
#[doc(alias = "__ZN3RBX7Network7Players20friendServiceRequestEbN5boost8weak_ptrINS0_6PlayerEEEi")]
pub fn stub_0xa14640() -> ! { todo!("0xa14640 __ZN3RBX7Network7Players20friendServiceRequestEbN5boost8weak_ptrINS0_6PlayerEEEi") }

// 0xa14aa0 — __ZNK3RBX7Network7Players11askAddChildEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Players *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX7Network7Players11askAddChildEPKNS_8InstanceE")]
pub fn stub_0xa14aa0() -> ! { todo!("0xa14aa0 __ZNK3RBX7Network7Players11askAddChildEPKNS_8InstanceE") }

// 0xa14bec — __ZN3RBX7Network7Players18findLocalCharacterEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findLocalCharacter(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX7Network7Players18findLocalCharacterEPNS_8InstanceE")]
pub fn stub_0xa14bec() -> ! { todo!("0xa14bec __ZN3RBX7Network7Players18findLocalCharacterEPNS_8InstanceE") }

// 0xa14c18 — __ZN3RBX7Network7Players15findLocalPlayerEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findLocalPlayer(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX7Network7Players15findLocalPlayerEPNS_8InstanceE")]
pub fn stub_0xa14c18() -> ! { todo!("0xa14c18 __ZN3RBX7Network7Players15findLocalPlayerEPNS_8InstanceE") }

// 0xa14c40 — __ZN3RBX7Network7Players23findConstLocalCharacterEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findConstLocalCharacter(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX7Network7Players23findConstLocalCharacterEPKNS_8InstanceE")]
pub fn stub_0xa14c40() -> ! { todo!("0xa14c40 __ZN3RBX7Network7Players23findConstLocalCharacterEPKNS_8InstanceE") }

// 0xa14c6c — __ZN3RBX7Network7Players20findConstLocalPlayerEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findConstLocalPlayer(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX7Network7Players20findConstLocalPlayerEPKNS_8InstanceE")]
pub fn stub_0xa14c6c() -> ! { todo!("0xa14c6c __ZN3RBX7Network7Players20findConstLocalPlayerEPKNS_8InstanceE") }

// 0xa14c94 — __ZN3RBX7Network7Players18findAncestorPlayerEPKNS_8InstanceE
// type: void __fastcall(RBX::Network::Players *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findAncestorPlayer(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX7Network7Players18findAncestorPlayerEPKNS_8InstanceE")]
pub fn stub_0xa14c94() -> ! { todo!("0xa14c94 __ZN3RBX7Network7Players18findAncestorPlayerEPKNS_8InstanceE") }

// 0xa1526c — __ZN3RBX7Network7Players22getPlayerFromCharacterEPNS_8InstanceE
// type: int __fastcall(RBX::Network::Players *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::getPlayerFromCharacter(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX7Network7Players22getPlayerFromCharacterEPNS_8InstanceE")]
pub fn stub_0xa1526c() -> ! { todo!("0xa1526c __ZN3RBX7Network7Players22getPlayerFromCharacterEPNS_8InstanceE") }

// 0xa15560 — __ZN3RBX7Network7Players20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int, int, int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "RBX::Network::Players::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX7Network7Players20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0xa15560() -> ! { todo!("0xa15560 __ZN3RBX7Network7Players20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE") }

// 0xa15700 — __ZN3RBX7Network7Players15onChildRemovingEPNS_8InstanceE
// type: void __fastcall(RBX::Network::Players *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::onChildRemoving(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX7Network7Players15onChildRemovingEPNS_8InstanceE")]
pub fn stub_0xa15700() -> ! { todo!("0xa15700 __ZN3RBX7Network7Players15onChildRemovingEPNS_8InstanceE") }

// 0xa16238 — __ZN3RBX7Network7Players25reportScriptSecurityErrorEiSsSsSs
// type: 
#[doc(alias = "RBX::Network::Players::reportScriptSecurityError(int,std::string,std::string,std::string)")]
#[doc(alias = "__ZN3RBX7Network7Players25reportScriptSecurityErrorEiSsSsSs")]
pub fn stub_0xa16238() -> ! { todo!("0xa16238 __ZN3RBX7Network7Players25reportScriptSecurityErrorEiSsSsSs") }

// 0xa1624c — __ZN3RBX7Network7Players24remoteInsertResultHelperEN5boost8weak_ptrIS1_EENS2_10shared_ptrINS_8InstanceEEEN3G3D7Vector3E
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::Players::remoteInsertResultHelper(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3)")]
#[doc(alias = "__ZN3RBX7Network7Players24remoteInsertResultHelperEN5boost8weak_ptrIS1_EENS2_10shared_ptrINS_8InstanceEEEN3G3D7Vector3E")]
pub fn stub_0xa1624c() -> ! { todo!("0xa1624c __ZN3RBX7Network7Players24remoteInsertResultHelperEN5boost8weak_ptrIS1_EENS2_10shared_ptrINS_8InstanceEEEN3G3D7Vector3E") }

// 0xa16648 — __ZN3RBX7Network7Players18remoteInsertResultEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3E
// type: void __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Players::remoteInsertResult(boost::shared_ptr<RBX::Instance>,G3D::Vector3)")]
#[doc(alias = "__ZN3RBX7Network7Players18remoteInsertResultEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3E")]
pub fn stub_0xa16648() -> ! { todo!("0xa16648 __ZN3RBX7Network7Players18remoteInsertResultEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3E") }

// 0xa16cb0 — __ZN3RBX7Network7Players10killPlayerEi
// type: void __fastcall(RBX::Network::Players *this, int)
#[doc(alias = "RBX::Network::Players::killPlayer(int)")]
#[doc(alias = "__ZN3RBX7Network7Players10killPlayerEi")]
pub fn stub_0xa16cb0() -> ! { todo!("0xa16cb0 __ZN3RBX7Network7Players10killPlayerEi") }

// 0xa16fa4 — __ZN3RBX7Network7Players16disconnectPlayerERNS_8InstanceEi
// type: void __fastcall(RBX::Network::Players *this, RBX::Instance *, int)
#[doc(alias = "RBX::Network::Players::disconnectPlayer(RBX::Instance &,int)")]
#[doc(alias = "__ZN3RBX7Network7Players16disconnectPlayerERNS_8InstanceEi")]
pub fn stub_0xa16fa4() -> ! { todo!("0xa16fa4 __ZN3RBX7Network7Players16disconnectPlayerERNS_8InstanceEi") }

// 0xa172e4 — __ZN3RBX7Network7Players16disconnectPlayerEi
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, int)
#[doc(alias = "RBX::Network::Players::disconnectPlayer(int)")]
#[doc(alias = "__ZN3RBX7Network7Players16disconnectPlayerEi")]
pub fn stub_0xa172e4() -> ! { todo!("0xa172e4 __ZN3RBX7Network7Players16disconnectPlayerEi") }

// 0xa17304 — __ZN3RBX7Network7Players21disconnectPlayerLocalEi
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, int)
#[doc(alias = "RBX::Network::Players::disconnectPlayerLocal(int)")]
#[doc(alias = "__ZN3RBX7Network7Players21disconnectPlayerLocalEi")]
pub fn stub_0xa17304() -> ! { todo!("0xa17304 __ZN3RBX7Network7Players21disconnectPlayerLocalEi") }

// 0xa17324 — __ZN3RBX7Network7Players16onRemoteSysStatsEiRKSsS3_b
// type: void __fastcall(RBX::Network::Players *this, uint32_t, const std::string *, const std::string *, int)
#[doc(alias = "RBX::Network::Players::onRemoteSysStats(int,std::string const&,std::string const&,bool)")]
#[doc(alias = "__ZN3RBX7Network7Players16onRemoteSysStatsEiRKSsS3_b")]
pub fn stub_0xa17324() -> ! { todo!("0xa17324 __ZN3RBX7Network7Players16onRemoteSysStatsEiRKSsS3_b") }

// 0xa18bc4 — __ZN3RBX7Network7Players12onChildAddedEPNS_8InstanceE
// type: void __fastcall(RBX::Network::Players *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX7Network7Players12onChildAddedEPNS_8InstanceE")]
pub fn stub_0xa18bc4() -> ! { todo!("0xa18bc4 __ZN3RBX7Network7Players12onChildAddedEPNS_8InstanceE") }

// 0xa1a480 — __ZN3RBX7Network7Players17buildClientRegionERNS_7Region2E
// type: RBX::Network::Player *__fastcall(RBX::Network::Player **this, RBX::Region2 *)
#[doc(alias = "RBX::Network::Players::buildClientRegion(RBX::Region2 &)")]
#[doc(alias = "__ZN3RBX7Network7Players17buildClientRegionERNS_7Region2E")]
pub fn stub_0xa1a480() -> ! { todo!("0xa1a480 __ZN3RBX7Network7Players17buildClientRegionERNS_7Region2E") }

// 0xa1a504 — __ZN3RBX7Network7Players21renderDPhysicsRegionsEPNS_5AdornE
// type: void __fastcall(RBX::Network::Players *this, RBX::Adorn *)
#[doc(alias = "RBX::Network::Players::renderDPhysicsRegions(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX7Network7Players21renderDPhysicsRegionsEPNS_5AdornE")]
pub fn stub_0xa1a504() -> ! { todo!("0xa1a504 __ZN3RBX7Network7Players21renderDPhysicsRegionsEPNS_5AdornE") }

// 0xa1a77c — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEEC1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEEC1Ev")]
pub fn stub_0xa1a77c() -> ! { todo!("0xa1a77c __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEEC1Ev") }

// 0xa1a788 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEEC2Ev")]
pub fn stub_0xa1a788() -> ! { todo!("0xa1a788 __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEEC2Ev") }

// 0xa1a9b0 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEEC1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEEC1Ev")]
pub fn stub_0xa1a9b0() -> ! { todo!("0xa1a9b0 __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEEC1Ev") }

// 0xa1a9bc — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEEC2Ev")]
pub fn stub_0xa1a9bc() -> ! { todo!("0xa1a9bc __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEEC2Ev") }

// 0xa1abe4 — __ZN3RBX15StringConverterINS_7Network7Players10ChatOptionEE14convertToValueERKSsRS3_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "RBX::StringConverter<RBX::Network::Players::ChatOption>::convertToValue(std::string const&,RBX::Network::Players::ChatOption&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_7Network7Players10ChatOptionEE14convertToValueERKSsRS3_")]
pub fn stub_0xa1abe4() -> ! { todo!("0xa1abe4 __ZN3RBX15StringConverterINS_7Network7Players10ChatOptionEE14convertToValueERKSsRS3_") }

// 0xa1adb8 — __ZNK3RBX7Network7Players13getNumPlayersEv
// type: int __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getNumPlayers(void)const")]
#[doc(alias = "__ZNK3RBX7Network7Players13getNumPlayersEv")]
pub fn stub_0xa1adb8() -> ! { todo!("0xa1adb8 __ZNK3RBX7Network7Players13getNumPlayersEv") }

// 0xa1ae1c — __ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEiED1Ev")]
pub fn stub_0xa1ae1c() -> ! { todo!("0xa1ae1c __ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEiED1Ev") }

// 0xa1ae40 — __ZNK3RBX7Network7Players13getMaxPlayersEv
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getMaxPlayers(void)const")]
#[doc(alias = "__ZNK3RBX7Network7Players13getMaxPlayersEv")]
pub fn stub_0xa1ae40() -> ! { todo!("0xa1ae40 __ZNK3RBX7Network7Players13getMaxPlayersEv") }

// 0xa1ae48 — __ZN3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEED1Ev")]
pub fn stub_0xa1ae48() -> ! { todo!("0xa1ae48 __ZN3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEED1Ev") }

// 0xa1ae74 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(int),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED1Ev")]
pub fn stub_0xa1ae74() -> ! { todo!("0xa1ae74 __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED1Ev") }

// 0xa1aedc — __ZNK3RBX7Network7Players21getCharacterAutoSpawnEv
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getCharacterAutoSpawn(void)const")]
#[doc(alias = "__ZNK3RBX7Network7Players21getCharacterAutoSpawnEv")]
pub fn stub_0xa1aedc() -> ! { todo!("0xa1aedc __ZNK3RBX7Network7Players21getCharacterAutoSpawnEv") }

// 0xa1aee4 — __ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbED1Ev")]
pub fn stub_0xa1aee4() -> ! { todo!("0xa1aee4 __ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbED1Ev") }

// 0xa1af08 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsELi1EED1Ev")]
pub fn stub_0xa1af08() -> ! { todo!("0xa1af08 __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsELi1EED1Ev") }

// 0xa1afb0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev")]
pub fn stub_0xa1afb0() -> ! { todo!("0xa1afb0 __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev") }

// 0xa1afbc — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_ED1Ev")]
pub fn stub_0xa1afbc() -> ! { todo!("0xa1afbc __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_ED1Ev") }

// 0xa1b004 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev")]
pub fn stub_0xa1b004() -> ! { todo!("0xa1b004 __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev") }

// 0xa1b04c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED1Ev")]
pub fn stub_0xa1b04c() -> ! { todo!("0xa1b04c __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED1Ev") }

// 0xa1b058 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC1EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, __guard *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Network::Players::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC1EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xa1b058() -> ! { todo!("0xa1b058 __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC1EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE") }

// 0xa1b218 — __ZN3RBX7Network7Players10getPlayersEv
// type: _DWORD *__fastcall(_DWORD *this, int)
#[doc(alias = "RBX::Network::Players::getPlayers(void)")]
#[doc(alias = "__ZN3RBX7Network7Players10getPlayersEv")]
pub fn stub_0xa1b218() -> ! { todo!("0xa1b218 __ZN3RBX7Network7Players10getPlayersEv") }

// 0xa1b26c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED1Ev")]
pub fn stub_0xa1b26c() -> ! { todo!("0xa1b26c __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED1Ev") }

// 0xa1b2b4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED1Ev
// type: 
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED1Ev")]
pub fn stub_0xa1b2b4() -> ! { todo!("0xa1b2b4 __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED1Ev") }

// 0xa1b2c0 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_ED1Ev")]
pub fn stub_0xa1b2c0() -> ! { todo!("0xa1b2c0 __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_ED1Ev") }

// 0xa1b308 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev")]
pub fn stub_0xa1b308() -> ! { todo!("0xa1b308 __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev") }

// 0xa1b350 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(RBX::Network::Players::ChatOption),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EED1Ev")]
pub fn stub_0xa1b350() -> ! { todo!("0xa1b350 __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EED1Ev") }

// 0xa1b3b8 — __ZNK3RBX7Network7Players14getClassicChatEv
// type: bool __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getClassicChat(void)const")]
#[doc(alias = "__ZNK3RBX7Network7Players14getClassicChatEv")]
pub fn stub_0xa1b3b8() -> ! { todo!("0xa1b3b8 __ZNK3RBX7Network7Players14getClassicChatEv") }

// 0xa1b3cc — __ZNK3RBX7Network7Players13getBubbleChatEv
// type: bool __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getBubbleChat(void)const")]
#[doc(alias = "__ZNK3RBX7Network7Players13getBubbleChatEv")]
pub fn stub_0xa1b3cc() -> ! { todo!("0xa1b3cc __ZNK3RBX7Network7Players13getBubbleChatEv") }

// 0xa1b3e0 — __ZN3RBX7Network12NetworkOwner10UnassignedEv
// type: __int64 __fastcall(RBX::Network::NetworkOwner *this)
#[doc(alias = "RBX::Network::NetworkOwner::Unassigned(void)")]
#[doc(alias = "__ZN3RBX7Network12NetworkOwner10UnassignedEv")]
pub fn stub_0xa1b3e0() -> ! { todo!("0xa1b3e0 __ZN3RBX7Network12NetworkOwner10UnassignedEv") }

// 0xa1b6b0 — __ZN5boost4bindIN3RBX13worker_thread11work_resultENS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsS8_SsEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// type: void __fastcall(int, int, int *, const std::string *)
#[doc(alias = "boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list_av_2<boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string>::type> boost::bind<RBX::worker_thread::work_result,boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string,boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string>(RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string)")]
#[doc(alias = "__ZN5boost4bindIN3RBX13worker_thread11work_resultENS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsS8_SsEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")]
pub fn stub_0xa1b6b0() -> ! { todo!("0xa1b6b0 __ZN5boost4bindIN3RBX13worker_thread11work_resultENS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsS8_SsEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_") }

// 0xa1bc50 — __ZSt8for_eachISt20_List_const_iteratorIN3RBX7Network11ChatMessageEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS2_11AbuseReportENS5_10shared_ptrINS2_6PlayerEEERKS3_EENS6_5list3INS5_17reference_wrapperISA_EENS6_5valueISD_EENS5_3argILi1EEEEEEEET0_T_SR_SQ_
// type: void __fastcall(int *, int, int, int *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>> std::for_each<std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>>>(std::_List_const_iterator<RBX::Network::ChatMessage>,std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>>)")]
#[doc(alias = "__ZSt8for_eachISt20_List_const_iteratorIN3RBX7Network11ChatMessageEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS2_11AbuseReportENS5_10shared_ptrINS2_6PlayerEEERKS3_EENS6_5list3INS5_17reference_wrapperISA_EENS6_5valueISD_EENS5_3argILi1EEEEEEEET0_T_SR_SQ_")]
pub fn stub_0xa1bc50() -> ! { todo!("0xa1bc50 __ZSt8for_eachISt20_List_const_iteratorIN3RBX7Network11ChatMessageEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS2_11AbuseReportENS5_10shared_ptrINS2_6PlayerEEERKS3_EENS6_5list3INS5_17reference_wrapperISA_EENS6_5valueISD_EENS5_3argILi1EEEEEEEET0_T_SR_SQ_") }

// 0xa1bf30 — __ZN5boost4bindIvN3RBX7Network11AbuseReportENS_10shared_ptrINS2_6PlayerEEERKNS2_11ChatMessageENS_17reference_wrapperIS3_EES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_
// type: void __fastcall(int, pthread_mutex_t *, int, int, pthread_mutex_t **)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list_av_3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&,boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>>(void (RBX::Network::AbuseReport::*)(boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&),boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX7Network11AbuseReportENS_10shared_ptrINS2_6PlayerEEERKNS2_11ChatMessageENS_17reference_wrapperIS3_EES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_")]
pub fn stub_0xa1bf30() -> ! { todo!("0xa1bf30 __ZN5boost4bindIvN3RBX7Network11AbuseReportENS_10shared_ptrINS2_6PlayerEEERKNS2_11ChatMessageENS_17reference_wrapperIS3_EES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_") }

// 0xa1c8e8 — __ZN3RBX10Reflection13DescribedBase21fastSharedDynamicCastINS_7Network6PlayerENS_8InstanceEEEN5boost10shared_ptrIT_EERKNS7_IT0_EE
// type: void __fastcall(_DWORD *, _DWORD *, int, int, int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Network::Player> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::Network::Player,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection13DescribedBase21fastSharedDynamicCastINS_7Network6PlayerENS_8InstanceEEEN5boost10shared_ptrIT_EERKNS7_IT0_EE")]
pub fn stub_0xa1c8e8() -> ! { todo!("0xa1c8e8 __ZN3RBX10Reflection13DescribedBase21fastSharedDynamicCastINS_7Network6PlayerENS_8InstanceEEEN5boost10shared_ptrIT_EERKNS7_IT0_EE") }

// 0xa1ced4 — __ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EEclES5_S9_SsS9_
// type: void __fastcall(_DWORD *, int, int *, std::string *, int *)
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::operator()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EEclES5_S9_SsS9_")]
pub fn stub_0xa1ced4() -> ! { todo!("0xa1ced4 __ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EEclES5_S9_SsS9_") }

// 0xa1d83c — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7Network11ChatMessageEEEclES6_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::ChatMessage const&)>::operator()(RBX::Network::ChatMessage const&)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7Network11ChatMessageEEEclES6_")]
pub fn stub_0xa1d83c() -> ! { todo!("0xa1d83c __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7Network11ChatMessageEEEclES6_") }

// 0xa1daf8 — __ZN3RBX11shared_fromINS_7Network6PlayerEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
#[doc(alias = "boost::shared_ptr<RBX::Network::Player> RBX::shared_from<RBX::Network::Player>(RBX::Network::Player*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_7Network6PlayerEEEN5boost10shared_ptrIT_EEPS5_")]
pub fn stub_0xa1daf8() -> ! { todo!("0xa1daf8 __ZN3RBX11shared_fromINS_7Network6PlayerEEEN5boost10shared_ptrIT_EEPS5_") }

// 0xa1dd8c — __ZN3RBX32shared_from_polymorphic_downcastINS_7Network6PlayerENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "boost::shared_ptr<RBX::Network::Player> RBX::shared_from_polymorphic_downcast<RBX::Network::Player,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
#[doc(alias = "__ZN3RBX32shared_from_polymorphic_downcastINS_7Network6PlayerENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE")]
pub fn stub_0xa1dd8c() -> ! { todo!("0xa1dd8c __ZN3RBX32shared_from_polymorphic_downcastINS_7Network6PlayerENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE") }

// 0xa1e654 — __ZN3RBX11shared_fromINS_7Network7PlayersEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
#[doc(alias = "boost::shared_ptr<RBX::Network::Players> RBX::shared_from<RBX::Network::Players>(RBX::Network::Players*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_7Network7PlayersEEEN5boost10shared_ptrIT_EEPS5_")]
pub fn stub_0xa1e654() -> ! { todo!("0xa1e654 __ZN3RBX11shared_fromINS_7Network7PlayersEEEN5boost10shared_ptrIT_EEPS5_") }

// 0xa1eab0 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX7Network11AbuseReportEEEclES4_
// type: void __fastcall(_DWORD *, int *, int, const void *)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::AbuseReport)>::operator()(RBX::Network::AbuseReport)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX7Network11AbuseReportEEEclES4_")]
pub fn stub_0xa1eab0() -> ! { todo!("0xa1eab0 __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX7Network11AbuseReportEEEclES4_") }

// 0xa1f558 — __ZN3RBX10Reflection13DescribedBase15fastDynamicCastINS_7Network6PlayerEEEPT_PS1_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Network::Player * RBX::Reflection::DescribedBase::fastDynamicCast<RBX::Network::Player>(RBX::Reflection::DescribedBase*)")]
#[doc(alias = "__ZN3RBX10Reflection13DescribedBase15fastDynamicCastINS_7Network6PlayerEEEPT_PS1_")]
pub fn stub_0xa1f558() -> ! { todo!("0xa1f558 __ZN3RBX10Reflection13DescribedBase15fastDynamicCastINS_7Network6PlayerEEEPT_PS1_") }

// 0xa1ff60 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS2_8InstanceEEEN3G3D7Vector3ES5_NS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
// type: void __fastcall(int, int, int *, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)")]
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS2_8InstanceEEEN3G3D7Vector3ES5_NS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_")]
pub fn stub_0xa1ff60() -> ! { todo!("0xa1ff60 __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS2_8InstanceEEEN3G3D7Vector3ES5_NS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_") }

// 0xa20280 — __ZN3RBX9weak_fromINS_7Network7PlayersEEEN5boost8weak_ptrIT_EEPS5_
// type: void __fastcall(int, int)
#[doc(alias = "boost::weak_ptr<RBX::Network::Players> RBX::weak_from<RBX::Network::Players>(RBX::Network::Players*)")]
#[doc(alias = "__ZN3RBX9weak_fromINS_7Network7PlayersEEEN5boost8weak_ptrIT_EEPS5_")]
pub fn stub_0xa20280() -> ! { todo!("0xa20280 __ZN3RBX9weak_fromINS_7Network7PlayersEEEN5boost8weak_ptrIT_EEPS5_") }

// 0xa207bc — __ZN5boost4bindIvN3RBX7Network7PlayersEbNS_8weak_ptrINS2_6PlayerEEEiPS3_NS_3argILi1EEES6_NS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISD_T0_T1_T2_T3_EENSB_9list_av_4IT4_T5_T6_T7_E4typeEEEMSG_FSD_SH_SI_SJ_ESM_SN_SO_SP_
// type: void __fastcall(_DWORD *, int, int, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list_av_4<RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>>::type> boost::bind<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int,RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>>(void (RBX::Network::Players::*)(bool,boost::weak_ptr<RBX::Network::Player>,int),RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX7Network7PlayersEbNS_8weak_ptrINS2_6PlayerEEEiPS3_NS_3argILi1EEES6_NS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISD_T0_T1_T2_T3_EENSB_9list_av_4IT4_T5_T6_T7_E4typeEEEMSG_FSD_SH_SI_SJ_ESM_SN_SO_SP_")]
pub fn stub_0xa207bc() -> ! { todo!("0xa207bc __ZN5boost4bindIvN3RBX7Network7PlayersEbNS_8weak_ptrINS2_6PlayerEEEiPS3_NS_3argILi1EEES6_NS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISD_T0_T1_T2_T3_EENSB_9list_av_4IT4_T5_T6_T7_E4typeEEEMSG_FSD_SH_SI_SJ_ESM_SN_SO_SP_") }

// 0xa20ac8 — __ZN3RBX9weak_fromINS_7Network6PlayerEEEN5boost8weak_ptrIT_EEPS5_
// type: void __fastcall(int, int)
#[doc(alias = "boost::weak_ptr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)")]
#[doc(alias = "__ZN3RBX9weak_fromINS_7Network6PlayerEEEN5boost8weak_ptrIT_EEPS5_")]
pub fn stub_0xa20ac8() -> ! { todo!("0xa20ac8 __ZN3RBX9weak_fromINS_7Network6PlayerEEEN5boost8weak_ptrIT_EEPS5_") }

// 0xa2133c — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE7addPairES4_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::addPair(RBX::Network::Players::ChatOption,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE7addPairES4_PKc")]
pub fn stub_0xa2133c() -> ! { todo!("0xa2133c __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE7addPairES4_PKc") }

// 0xa21864 — __ZN3RBX10Reflection7Variant14genericConvertINS_7Network7Players10ChatOptionEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::Network::Players::ChatOption & RBX::Reflection::Variant::genericConvert<RBX::Network::Players::ChatOption>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_7Network7Players10ChatOptionEEERT_v")]
pub fn stub_0xa21864() -> ! { todo!("0xa21864 __ZN3RBX10Reflection7Variant14genericConvertINS_7Network7Players10ChatOptionEEERT_v") }

// 0xa21bb4 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE7addPairES4_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::addPair(RBX::Network::Players::PlayerChatType,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE7addPairES4_PKc")]
pub fn stub_0xa21bb4() -> ! { todo!("0xa21bb4 __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE7addPairES4_PKc") }

// 0xa22504 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED1Ev
// type: 
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED1Ev")]
pub fn stub_0xa22504() -> ! { todo!("0xa22504 __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED1Ev") }

// 0xa22510 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED0Ev")]
pub fn stub_0xa22510() -> ! { todo!("0xa22510 __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED0Ev") }

// 0xa225b0 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE6lookupEPKc
// type: int __fastcall(_DWORD *, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE6lookupEPKc")]
pub fn stub_0xa225b0() -> ! { todo!("0xa225b0 __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE6lookupEPKc") }

// 0xa22640 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE6lookupERKNS0_7VariantE")]
pub fn stub_0xa22640() -> ! { todo!("0xa22640 __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE6lookupERKNS0_7VariantE") }

// 0xa22744 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0xa22744() -> ! { todo!("0xa22744 __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE14convertToValueEmRNS0_7VariantE") }

// 0xa22804 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringEmRSs")]
pub fn stub_0xa22804() -> ! { todo!("0xa22804 __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringEmRSs") }

// 0xa22948 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED1Ev")]
pub fn stub_0xa22948() -> ! { todo!("0xa22948 __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED1Ev") }

// 0xa22954 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED0Ev")]
pub fn stub_0xa22954() -> ! { todo!("0xa22954 __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED0Ev") }

// 0xa229f4 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE6lookupEPKc
// type: int __fastcall(_DWORD *, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE6lookupEPKc")]
pub fn stub_0xa229f4() -> ! { todo!("0xa229f4 __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE6lookupEPKc") }

// 0xa22a84 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE6lookupERKNS0_7VariantE")]
pub fn stub_0xa22a84() -> ! { todo!("0xa22a84 __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE6lookupERKNS0_7VariantE") }

// 0xa22b88 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0xa22b88() -> ! { todo!("0xa22b88 __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE14convertToValueEmRNS0_7VariantE") }

// 0xa22c48 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringEmRSs")]
pub fn stub_0xa22c48() -> ! { todo!("0xa22c48 __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringEmRSs") }

// 0xa22d8c — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringERKS4_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(RBX::Network::Players::PlayerChatType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringERKS4_")]
pub fn stub_0xa22d8c() -> ! { todo!("0xa22d8c __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringERKS4_") }

// 0xa22f2c — __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players14PlayerChatTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX7Network7Players14PlayerChatTypeEE14construct_funcEPKcPc")]
pub fn stub_0xa22f2c() -> ! { todo!("0xa22f2c __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players14PlayerChatTypeEE14construct_funcEPKcPc") }

// 0xa22f38 — __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players14PlayerChatTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX7Network7Players14PlayerChatTypeEE13destruct_funcEPc")]
pub fn stub_0xa22f38() -> ! { todo!("0xa22f38 __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players14PlayerChatTypeEE13destruct_funcEPc") }

// 0xa22f3c — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE13convertToItemERKS4_
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToItem(RBX::Network::Players::PlayerChatType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE13convertToItemERKS4_")]
pub fn stub_0xa22f3c() -> ! { todo!("0xa22f3c __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE13convertToItemERKS4_") }

// 0xa23008 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED2Ev")]
pub fn stub_0xa23008() -> ! { todo!("0xa23008 __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED2Ev") }

// 0xa23284 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringERKS4_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(RBX::Network::Players::ChatOption const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringERKS4_")]
pub fn stub_0xa23284() -> ! { todo!("0xa23284 __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringERKS4_") }

// 0xa23424 — __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players10ChatOptionEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX7Network7Players10ChatOptionEE14construct_funcEPKcPc")]
pub fn stub_0xa23424() -> ! { todo!("0xa23424 __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players10ChatOptionEE14construct_funcEPKcPc") }

// 0xa23430 — __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players10ChatOptionEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX7Network7Players10ChatOptionEE13destruct_funcEPc")]
pub fn stub_0xa23430() -> ! { todo!("0xa23430 __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players10ChatOptionEE13destruct_funcEPc") }

// 0xa23434 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE13convertToItemERKS4_
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToItem(RBX::Network::Players::ChatOption const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE13convertToItemERKS4_")]
pub fn stub_0xa23434() -> ! { todo!("0xa23434 __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE13convertToItemERKS4_") }

// 0xa23500 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED2Ev")]
pub fn stub_0xa23500() -> ! { todo!("0xa23500 __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED2Ev") }

// 0xa24e24 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Network7Players14PlayerChatTypeEEEE13initSingletonEv
// type: void()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Network7Players14PlayerChatTypeEEEE13initSingletonEv")]
pub fn stub_0xa24e24() -> ! { todo!("0xa24e24 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Network7Players14PlayerChatTypeEEEE13initSingletonEv") }

// 0xa24f08 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Network7Players10ChatOptionEEEE13initSingletonEv
// type: void()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Network7Players10ChatOptionEEEE13initSingletonEv")]
pub fn stub_0xa24f08() -> ! { todo!("0xa24f08 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Network7Players10ChatOptionEEEE13initSingletonEv") }

// 0xa24fec — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players14PlayerChatTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>,std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players14PlayerChatTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
pub fn stub_0xa24fec() -> ! { todo!("0xa24fec __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players14PlayerChatTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_") }

// 0xa251a0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players14PlayerChatTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: _Rb_tree_node_base *__fastcall(int, _DWORD *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::Players::PlayerChatType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players14PlayerChatTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")]
pub fn stub_0xa251a0() -> ! { todo!("0xa251a0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players14PlayerChatTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_") }

// 0xa25290 — __ZNSt6vectorIN3RBX7Network7Players14PlayerChatTypeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Network::Players::PlayerChatType,std::allocator<RBX::Network::Players::PlayerChatType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::Players::PlayerChatType*,std::vector<RBX::Network::Players::PlayerChatType,std::allocator<RBX::Network::Players::PlayerChatType>>>,RBX::Network::Players::PlayerChatType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX7Network7Players14PlayerChatTypeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
pub fn stub_0xa25290() -> ! { todo!("0xa25290 __ZNSt6vectorIN3RBX7Network7Players14PlayerChatTypeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_") }

// 0xa253a0 — __ZNSt6vectorIN3RBX7Network7Players14PlayerChatTypeESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *, unsigned int, _DWORD *)
#[doc(alias = "std::vector<RBX::Network::Players::PlayerChatType,std::allocator<RBX::Network::Players::PlayerChatType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::Players::PlayerChatType*,std::vector<RBX::Network::Players::PlayerChatType,std::allocator<RBX::Network::Players::PlayerChatType>>>,unsigned long,RBX::Network::Players::PlayerChatType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX7Network7Players14PlayerChatTypeESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")]
pub fn stub_0xa253a0() -> ! { todo!("0xa253a0 __ZNSt6vectorIN3RBX7Network7Players14PlayerChatTypeESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_") }

// 0xa2554c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players10ChatOptionEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>,std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players10ChatOptionEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
pub fn stub_0xa2554c() -> ! { todo!("0xa2554c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players10ChatOptionEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_") }

// 0xa25700 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players10ChatOptionEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: _Rb_tree_node_base *__fastcall(int, _DWORD *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::Players::ChatOption> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players10ChatOptionEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")]
pub fn stub_0xa25700() -> ! { todo!("0xa25700 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network7Players10ChatOptionEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_") }

// 0xa257f0 — __ZNSt6vectorIN3RBX7Network7Players10ChatOptionESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Network::Players::ChatOption,std::allocator<RBX::Network::Players::ChatOption>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::Players::ChatOption*,std::vector<RBX::Network::Players::ChatOption,std::allocator<RBX::Network::Players::ChatOption>>>,RBX::Network::Players::ChatOption const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX7Network7Players10ChatOptionESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
pub fn stub_0xa257f0() -> ! { todo!("0xa257f0 __ZNSt6vectorIN3RBX7Network7Players10ChatOptionESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_") }

// 0xa25900 — __ZNSt6vectorIN3RBX7Network7Players10ChatOptionESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *, unsigned int, _DWORD *)
#[doc(alias = "std::vector<RBX::Network::Players::ChatOption,std::allocator<RBX::Network::Players::ChatOption>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::Players::ChatOption*,std::vector<RBX::Network::Players::ChatOption,std::allocator<RBX::Network::Players::ChatOption>>>,unsigned long,RBX::Network::Players::ChatOption const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX7Network7Players10ChatOptionESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")]
pub fn stub_0xa25900() -> ! { todo!("0xa25900 __ZNSt6vectorIN3RBX7Network7Players10ChatOptionESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_") }

// 0xa27768 — __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX7Network7PlayersEbNS5_8weak_ptrINSB_6PlayerEEEiEENS6_5list4INS6_5valueIPSC_EENS5_3argILi1EEENSI_ISF_EENSL_ILi2EEEEEEEED1Ev
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX7Network7PlayersEbNS5_8weak_ptrINSB_6PlayerEEEiEENS6_5list4INS6_5valueIPSC_EENS5_3argILi1EEENSI_ISF_EENSL_ILi2EEEEEEEED1Ev")]
pub fn stub_0xa27768() -> ! { todo!("0xa27768 __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX7Network7PlayersEbNS5_8weak_ptrINSB_6PlayerEEEiEENS6_5list4INS6_5valueIPSC_EENS5_3argILi1EEENSI_ISF_EENSL_ILi2EEEEEEEED1Ev") }

// 0xa27774 — __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX7Network7PlayersEbNS5_8weak_ptrINSB_6PlayerEEEiEENS6_5list4INS6_5valueIPSC_EENS5_3argILi1EEENSI_ISF_EENSL_ILi2EEEEEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX7Network7PlayersEbNS5_8weak_ptrINSB_6PlayerEEEiEENS6_5list4INS6_5valueIPSC_EENS5_3argILi1EEENSI_ISF_EENSL_ILi2EEEEEEEED0Ev")]
pub fn stub_0xa27774() -> ! { todo!("0xa27774 __ZN3rbx7signals6signalIFvbiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX7Network7PlayersEbNS5_8weak_ptrINSB_6PlayerEEEiEENS6_5list4INS6_5valueIPSC_EENS5_3argILi1EEENSI_ISF_EENSL_ILi2EEEEEEEED0Ev") }

// 0xa279b4 — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_E4callEbi
// type: int __fastcall(int, pthread_mutex_t *, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::call(bool,int)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_E4callEbi")]
pub fn stub_0xa279b4() -> ! { todo!("0xa279b4 __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_E4callEbi") }
