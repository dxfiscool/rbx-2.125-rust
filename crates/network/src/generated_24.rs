//! network generated_24 — RakNet + RBX::Network + Replicator + replica/remote expansion (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator|replica|remote (5974 total, 5306 prior filtered + 100 = 5406 filtered, 5619 prior unique + 100 = 5719 combined network crate stubs, shard BG8, EA-sorted ascending earliest gap, 668 remaining before batch, 568 after).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Boost types mapped: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> Weak, with // was: original.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

// 0x8d9dbc — __ZN3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEED0Ev // was: boost::shared_ptr
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::~RemoteEventDesc()")]
pub fn stub_8d9dbc() -> ! {
    todo!("0x8d9dbc RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::~RemoteEventDesc()")
}

// 0x8d9e70 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEMS2_SM_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_8d9e70() -> ! {
    todo!("0x8d9e70 RBX::Reflection::EventDescImpl<1,RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8d9fd4 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEE12isScriptableEv // was: boost::shared_ptr
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::isScriptable(void)const")]
pub fn stub_8d9fd4() -> ! {
    todo!("0x8d9fd4 RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::isScriptable(void)const")
}

// 0x8d9fdc — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEE11isBroadcastEv // was: boost::shared_ptr
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::isBroadcast(void)const")]
pub fn stub_8d9fdc() -> ! {
    todo!("0x8d9fdc RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::isBroadcast(void)const")
}

// 0x8d9fe4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEMS2_SM_E9fireEventEPNS0_11EventSourceERKSt6vectorIS7_SaIS7_EE // was: boost::shared_ptr
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_8d9fe4() -> ! {
    todo!("0x8d9fe4 RBX::Reflection::EventDescImpl<1,RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8da144 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEE9sendEventEPNS0_11EventSourceERKSt6vectorIS7_SaIS7_EE // was: boost::shared_ptr
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_8da144() -> ! {
    todo!("0x8da144 RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8da154 — __ZNK3RBX10Reflection13EventDescBaseINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEMS2_SM_E13disconnectAllEPNS0_11EventSourceE // was: boost::shared_ptr
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_8da154() -> ! {
    todo!("0x8da154 RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8db838 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEMS2_SM_EC2ESN_PKcSQ_NS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_8db838() -> ! {
    todo!("0x8db838 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8db9bc — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEMS2_SM_ED1Ev // was: boost::shared_ptr
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::~EventDesc()")]
pub fn stub_8db9bc() -> ! {
    todo!("0x8db9bc RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::~EventDesc()")
}

// 0x8db9e0 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEN3rbx13remote_signalISJ_EEMS2_SM_ED0Ev // was: boost::shared_ptr
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::~EventDesc()")]
pub fn stub_8db9e0() -> ! {
    todo!("0x8db9e0 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> RBX::MarketplaceService::*>::~EventDesc()")
}

// 0x8dba94 — __ZN3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::~RemoteEventDesc()")]
pub fn stub_8dba94() -> ! {
    todo!("0x8dba94 RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::~RemoteEventDesc()")
}

// 0x8dbb48 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_8dbb48() -> ! {
    todo!("0x8dbb48 RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8dbcac — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::isScriptable(void)const")]
pub fn stub_8dbcac() -> ! {
    todo!("0x8dbcac RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::isScriptable(void)const")
}

// 0x8dbcb4 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::isBroadcast(void)const")]
pub fn stub_8dbcb4() -> ! {
    todo!("0x8dbcb4 RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::isBroadcast(void)const")
}

// 0x8dbcbc — __ZNK3RBX10Reflection13EventDescImplILi3ENS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_8dbcbc() -> ! {
    todo!("0x8dbcbc RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8dbe8c — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_8dbe8c() -> ! {
    todo!("0x8dbe8c RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8dbe9c — __ZNK3RBX10Reflection13EventDescBaseINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_8dbe9c() -> ! {
    todo!("0x8dbe9c RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8dc8dc — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_8dc8dc() -> ! {
    todo!("0x8dc8dc RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8dcb38 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::~EventDesc()")]
pub fn stub_8dcb38() -> ! {
    todo!("0x8dcb38 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::~EventDesc()")
}

// 0x8dcb5c — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::~EventDesc()")]
pub fn stub_8dcb5c() -> ! {
    todo!("0x8dcb5c RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::~EventDesc()")
}

// 0x8dd22c — __ZN3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEED0Ev // was: boost::shared_ptr
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~RemoteEventDesc()")]
pub fn stub_8dd22c() -> ! {
    todo!("0x8dd22c RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::~RemoteEventDesc()")
}

// 0x8dd2e0 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_8dd2e0() -> ! {
    todo!("0x8dd2e0 RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x8dd444 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE12isScriptableEv // was: boost::shared_ptr
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const")]
pub fn stub_8dd444() -> ! {
    todo!("0x8dd444 RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isScriptable(void)const")
}

// 0x8dd44c — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE11isBroadcastEv // was: boost::shared_ptr
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isBroadcast(void)const")]
pub fn stub_8dd44c() -> ! {
    todo!("0x8dd44c RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::isBroadcast(void)const")
}

// 0x8dd454 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE // was: boost::shared_ptr
// type: void __fastcall(int, int, _DWORD *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_8dd454() -> ! {
    todo!("0x8dd454 RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8dd600 — __ZNK3RBX10Reflection15RemoteEventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE // was: boost::shared_ptr
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_8dd600() -> ! {
    todo!("0x8dd600 RBX::Reflection::RemoteEventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x8dd610 — __ZNK3RBX10Reflection13EventDescBaseINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE // was: boost::shared_ptr
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_8dd610() -> ! {
    todo!("0x8dd610 RBX::Reflection::EventDescBase<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x8df0c4 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_8df0c4() -> ! {
    todo!("0x8df0c4 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x8df390 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_ED1Ev // was: boost::shared_ptr
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()")]
pub fn stub_8df390() -> ! {
    todo!("0x8df390 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()")
}

// 0x8df3b4 — __ZN3RBX10Reflection9EventDescINS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_ED0Ev // was: boost::shared_ptr
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()")]
pub fn stub_8df3b4() -> ! {
    todo!("0x8df3b4 RBX::Reflection::EventDesc<RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::~EventDesc()")
}

// 0x8e02d4 — __ZN3rbx13remote_signalIFviibEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(int,int,bool)>::~remote_signal()")]
pub fn stub_8e02d4() -> ! {
    todo!("0x8e02d4 rbx::remote_signal<void ()(int,int,bool)>::~remote_signal()")
}

// 0x8e0420 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibEED2Ev // was: boost::shared_ptr
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>::~remote_signal()")]
pub fn stub_8e0420() -> ! {
    todo!("0x8e0420 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>::~remote_signal()")
}

// 0x8e056c — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS3_18MarketplaceService12CurrencyTypeEEED2Ev // was: boost::shared_ptr
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~remote_signal()")]
pub fn stub_8e056c() -> ! {
    todo!("0x8e056c rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~remote_signal()")
}

// 0x8e06b8 — __ZN3rbx13remote_signalIFvSsiiEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string,int,int)>::~remote_signal()")]
pub fn stub_8e06b8() -> ! {
    todo!("0x8e06b8 rbx::remote_signal<void ()(std::string,int,int)>::~remote_signal()")
}

// 0x8e0804 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIKNS1_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEED2Ev // was: boost::shared_ptr
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>::~remote_signal()")]
pub fn stub_8e0804() -> ! {
    todo!("0x8e0804 rbx::remote_signal<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>::~remote_signal()")
}

// 0x919e2c — __ZN3RBX14RemoteFunction12invokeServerEN5boost10shared_ptrIKNS_10Reflection5TupleEEENS1_8functionIFvS6_EEENS7_IFvSsEEE // was: boost::shared_ptr
#[doc(alias = "RBX::RemoteFunction::invokeServer(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_919e2c() -> ! {
    todo!("0x919e2c RBX::RemoteFunction::invokeServer(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x91a240 — __ZN3RBX14RemoteFunction12invokeClientEN5boost10shared_ptrINS_8InstanceEEENS2_IKNS_10Reflection5TupleEEENS1_8functionIFvS8_EEENS9_IFvSsEEE // was: boost::shared_ptr
#[doc(alias = "RBX::RemoteFunction::invokeClient(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_91a240() -> ! {
    todo!("0x91a240 RBX::RemoteFunction::invokeClient(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x91a6cc — __ZN3RBX14RemoteFunction25processDelayedInvocationsEv
// type: _DWORD __fastcall(RBX::RemoteFunction *__hidden this)
#[doc(alias = "RBX::RemoteFunction::processDelayedInvocations(void)")]
pub fn stub_91a6cc() -> ! {
    todo!("0x91a6cc RBX::RemoteFunction::processDelayedInvocations(void)")
}

// 0x91a6d4 — __ZN3RBX11RemoteEvent10fireServerEN5boost10shared_ptrIKNS_10Reflection5TupleEEE // was: boost::shared_ptr
// type: int __fastcall(int, int)
#[doc(alias = "RBX::RemoteEvent::fireServer(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_91a6d4() -> ! {
    todo!("0x91a6d4 RBX::RemoteEvent::fireServer(boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x91a9e4 — __ZN3RBX11RemoteEvent10fireClientEN5boost10shared_ptrINS_8InstanceEEENS2_IKNS_10Reflection5TupleEEE // was: boost::shared_ptr
#[doc(alias = "RBX::RemoteEvent::fireClient(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_91a9e4() -> ! {
    todo!("0x91a9e4 RBX::RemoteEvent::fireClient(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x91ad70 — __ZN3RBX11RemoteEvent14fireAllClientsEN5boost10shared_ptrIKNS_10Reflection5TupleEEE // was: boost::shared_ptr
#[doc(alias = "RBX::RemoteEvent::fireAllClients(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_91ad70() -> ! {
    todo!("0x91ad70 RBX::RemoteEvent::fireAllClients(boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x91b1a4 — __ZN3RBX14RemoteFunctionC2Ev
// type: _DWORD __fastcall(RBX::RemoteFunction *__hidden this)
#[doc(alias = "RBX::RemoteFunction::RemoteFunction(void)")]
pub fn stub_91b1a4() -> ! {
    todo!("0x91b1a4 RBX::RemoteFunction::RemoteFunction(void)")
}

// 0x91b498 — __ZN3RBX14RemoteFunction12localSuccessEiN5boost10shared_ptrIKNS_10Reflection5TupleEEE // was: boost::shared_ptr
#[doc(alias = "RBX::RemoteFunction::localSuccess(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_91b498() -> ! {
    todo!("0x91b498 RBX::RemoteFunction::localSuccess(int,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x91b654 — __ZN3RBX14RemoteFunction10localErrorEiSs
#[doc(alias = "RBX::RemoteFunction::localError(int,std::string)")]
pub fn stub_91b654() -> ! {
    todo!("0x91b654 RBX::RemoteFunction::localError(int,std::string)")
}

// 0x91b86c — __ZN3RBX14RemoteFunction22createRemoteInvocationEN5boost8functionIFvNS1_10shared_ptrIKNS_10Reflection5TupleEEEEEENS2_IFvSsEEE // was: boost::shared_ptr
#[doc(alias = "RBX::RemoteFunction::createRemoteInvocation(boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_91b86c() -> ! {
    todo!("0x91b86c RBX::RemoteFunction::createRemoteInvocation(boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x91ba74 — __ZN3RBX14RemoteFunction17localInvokeServerEN5boost10shared_ptrINS_8InstanceEEENS2_IKNS_10Reflection5TupleEEENS1_8functionIFvS8_EEENS9_IFvSsEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::RemoteFunction::localInvokeServer(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_91ba74() -> ! {
    todo!("0x91ba74 RBX::RemoteFunction::localInvokeServer(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x91c148 — __ZN3RBX14RemoteFunction17localInvokeClientEN5boost10shared_ptrIKNS_10Reflection5TupleEEENS1_8functionIFvS6_EEENS7_IFvSsEEE // was: boost::shared_ptr
// type: int __fastcall(int)
#[doc(alias = "RBX::RemoteFunction::localInvokeClient(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_91c148() -> ! {
    todo!("0x91c148 RBX::RemoteFunction::localInvokeClient(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x91c63c — __ZNK3RBX14RemoteFunction12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::RemoteFunction *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::RemoteFunction::askSetParent(RBX::Instance const*)const")]
pub fn stub_91c63c() -> ! {
    todo!("0x91c63c RBX::RemoteFunction::askSetParent(RBX::Instance const*)const")
}

// 0x91c640 — __ZN3RBX14RemoteFunction23consumeRemoteInvocationEiRNS0_16RemoteInvocationE
#[doc(alias = "RBX::RemoteFunction::consumeRemoteInvocation(int,RBX::RemoteFunction::RemoteInvocation &)")]
pub fn stub_91c640() -> ! {
    todo!("0x91c640 RBX::RemoteFunction::consumeRemoteInvocation(int,RBX::RemoteFunction::RemoteInvocation &)")
}

// 0x91c6ac — __ZN3RBX14RemoteFunction18processRemoteEventERKNS_10Reflection15EventDescriptorERKSt6vectorINS1_7VariantESaIS6_EERKNS_13SystemAddressE
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::RemoteFunction::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")]
pub fn stub_91c6ac() -> ! {
    todo!("0x91c6ac RBX::RemoteFunction::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")
}

// 0x91cc1c — __ZN3RBX14RemoteFunction13remoteSuccessENS_13SystemAddressEiN5boost10shared_ptrIKNS_10Reflection5TupleEEE // was: boost::shared_ptr
#[doc(alias = "RBX::RemoteFunction::remoteSuccess(RBX::SystemAddress,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_91cc1c() -> ! {
    todo!("0x91cc1c RBX::RemoteFunction::remoteSuccess(RBX::SystemAddress,int,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x91cd94 — __ZN3RBX14RemoteFunction11remoteErrorENS_13SystemAddressEiSs
#[doc(alias = "RBX::RemoteFunction::remoteError(RBX::SystemAddress,int,std::string)")]
pub fn stub_91cd94() -> ! {
    todo!("0x91cd94 RBX::RemoteFunction::remoteError(RBX::SystemAddress,int,std::string)")
}

// 0x91cf0c — __ZThn36_N3RBX14RemoteFunction18processRemoteEventERKNS_10Reflection15EventDescriptorERKSt6vectorINS1_7VariantESaIS6_EERKNS_13SystemAddressE
#[doc(alias = "non-virtual thunk to RBX::RemoteFunction::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")]
pub fn stub_91cf0c() -> ! {
    todo!("0x91cf0c non-virtual thunk to RBX::RemoteFunction::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")
}

// 0x91cf14 — __ZN3RBX11RemoteEventC2Ev
// type: _DWORD __fastcall(RBX::RemoteEvent *__hidden this)
#[doc(alias = "RBX::RemoteEvent::RemoteEvent(void)")]
pub fn stub_91cf14() -> ! {
    todo!("0x91cf14 RBX::RemoteEvent::RemoteEvent(void)")
}

// 0x91d110 — __ZNK3RBX11RemoteEvent12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::RemoteEvent *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::RemoteEvent::askSetParent(RBX::Instance const*)const")]
pub fn stub_91d110() -> ! {
    todo!("0x91d110 RBX::RemoteEvent::askSetParent(RBX::Instance const*)const")
}

// 0x91d114 — __ZN3RBX11RemoteEvent18processRemoteEventERKNS_10Reflection15EventDescriptorERKSt6vectorINS1_7VariantESaIS6_EERKNS_13SystemAddressE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::RemoteEvent::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")]
pub fn stub_91d114() -> ! {
    todo!("0x91d114 RBX::RemoteEvent::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")
}

// 0x91d2f4 — __ZThn36_N3RBX11RemoteEvent18processRemoteEventERKNS_10Reflection15EventDescriptorERKSt6vectorINS1_7VariantESaIS6_EERKNS_13SystemAddressE
#[doc(alias = "non-virtual thunk to RBX::RemoteEvent::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")]
pub fn stub_91d2f4() -> ! {
    todo!("0x91d2f4 non-virtual thunk to RBX::RemoteEvent::processRemoteEvent(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const&)")
}

// 0x91d460 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")]
pub fn stub_91d460() -> ! {
    todo!("0x91d460 RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")
}

// 0x91d554 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEENS4_INS_8InstanceEEES7_ES7_Li2EED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,2>::~BoundYieldFuncDesc()")]
pub fn stub_91d554() -> ! {
    todo!("0x91d554 RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,2>::~BoundYieldFuncDesc()")
}

// 0x91d85c — __ZN3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")]
pub fn stub_91d85c() -> ! {
    todo!("0x91d85c RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")
}

// 0x91d880 — __ZN3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")]
pub fn stub_91d880() -> ! {
    todo!("0x91d880 RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")
}

// 0x91d8a4 — __ZN3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEED1Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::~RemoteEventDesc()")]
pub fn stub_91d8a4() -> ! {
    todo!("0x91d8a4 RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::~RemoteEventDesc()")
}

// 0x91d8c8 — __ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::~BoundFuncDesc()")]
pub fn stub_91d8c8() -> ! {
    todo!("0x91d8c8 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::~BoundFuncDesc()")
}

// 0x91d9bc — __ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEELi2EED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),2>::~BoundFuncDesc()")]
pub fn stub_91d9bc() -> ! {
    todo!("0x91d9bc RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),2>::~BoundFuncDesc()")
}

// 0x91dad4 — __ZN3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")]
pub fn stub_91dad4() -> ! {
    todo!("0x91dad4 RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")
}

// 0x91daf8 — __ZN3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")]
pub fn stub_91daf8() -> ! {
    todo!("0x91daf8 RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")
}

// 0x91db60 — __ZNSt3mapIiN3RBX14RemoteFunction16RemoteInvocationESt4lessIiESaISt4pairIKiS2_EEEixERS6_
#[doc(alias = "std::map<int,RBX::RemoteFunction::RemoteInvocation,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::operator[](int const&)")]
pub fn stub_91db60() -> ! {
    todo!("0x91db60 std::map<int,RBX::RemoteFunction::RemoteInvocation,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::operator[](int const&)")
}

// 0x91dd74 — __ZN5boost4bindIvN3RBX14RemoteFunctionENS_10shared_ptrINS1_8InstanceEEENS3_IKNS1_10Reflection5TupleEEENS_8functionIFvS9_EEENSA_IFvSsEEEPS2_S5_S9_SC_SE_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISI_T0_T1_T2_T3_T4_EENSG_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSL_FSI_SM_SN_SO_SP_ESS_ST_SU_SV_SW_ // was: boost::shared_ptr
// type: int __fastcall(int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>,RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>(void (RBX::RemoteFunction::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_91dd74() -> ! {
    todo!("0x91dd74 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::RemoteFunction*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>,RBX::RemoteFunction*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>(void (RBX::RemoteFunction::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),RBX::RemoteFunction*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x91e318 — __ZN5boost4bindIvN3RBX14RemoteFunctionENS_10shared_ptrIKNS1_10Reflection5TupleEEENS_8functionIFvS7_EEENS8_IFvSsEEEPS2_S7_SA_SC_EENS_3_bi6bind_tIT_NS_4_mfi3mf3ISG_T0_T1_T2_T3_EENSE_9list_av_4IT4_T5_T6_T7_E4typeEEEMSJ_FSG_SK_SL_SM_ESP_SQ_SR_SS_ // was: boost::shared_ptr
// type: int __fastcall(int, int, char, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_4<RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>,RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>(void (RBX::RemoteFunction::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_91e318() -> ! {
    todo!("0x91e318 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_4<RBX::RemoteFunction*,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>,RBX::RemoteFunction*,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>(void (RBX::RemoteFunction::*)(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),RBX::RemoteFunction*,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x91e57c — __ZN3RBX13LatchedSignalIN3rbx13remote_signalEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS_10Reflection5TupleEEEEEclIS6_SA_EEvT_T0_ // was: boost::shared_ptr
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::operator()<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_91e57c() -> ! {
    todo!("0x91e57c void RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::operator()<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x91e690 — __ZN3RBX13LatchedSignalIN3rbx13remote_signalEFvN5boost10shared_ptrIKNS_10Reflection5TupleEEEEEclIS8_EEvT_ // was: boost::shared_ptr
#[doc(alias = "void RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::operator()<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_91e690() -> ! {
    todo!("0x91e690 void RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::operator()<boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x91e75c — __ZN3RBX14RemoteFunctionD1Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
#[doc(alias = "RBX::RemoteFunction::~RemoteFunction()")]
pub fn stub_91e75c() -> ! {
    todo!("0x91e75c RBX::RemoteFunction::~RemoteFunction()")
}

// 0x91e760 — __ZN3RBX14RemoteFunctionD0Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
#[doc(alias = "RBX::RemoteFunction::~RemoteFunction()")]
pub fn stub_91e760() -> ! {
    todo!("0x91e760 RBX::RemoteFunction::~RemoteFunction()")
}

// 0x91e800 — __ZNK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E12getClassNameEv")]
pub fn stub_91e800() -> ! {
    todo!("0x91e800 __ZNK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E12getClassNameEv")
}

// 0x91e810 — __ZThn32_N3RBX14RemoteFunctionD1Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::RemoteFunction::~RemoteFunction()")]
pub fn stub_91e810() -> ! {
    todo!("0x91e810 non-virtual thunk to RBX::RemoteFunction::~RemoteFunction()")
}

// 0x91e818 — __ZThn32_N3RBX14RemoteFunctionD0Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::RemoteFunction::~RemoteFunction()")]
pub fn stub_91e818() -> ! {
    todo!("0x91e818 non-virtual thunk to RBX::RemoteFunction::~RemoteFunction()")
}

// 0x91e8bc — __ZThn32_NK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E12getClassNameEv")]
pub fn stub_91e8bc() -> ! {
    todo!("0x91e8bc __ZThn32_NK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E12getClassNameEv")
}

// 0x91e8cc — __ZThn36_N3RBX14RemoteFunctionD1Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::RemoteFunction::~RemoteFunction()")]
pub fn stub_91e8cc() -> ! {
    todo!("0x91e8cc non-virtual thunk to RBX::RemoteFunction::~RemoteFunction()")
}

// 0x91e8d4 — __ZThn36_N3RBX14RemoteFunctionD0Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::RemoteFunction::~RemoteFunction()")]
pub fn stub_91e8d4() -> ! {
    todo!("0x91e8d4 non-virtual thunk to RBX::RemoteFunction::~RemoteFunction()")
}

// 0x91e978 — __ZN3RBX11RemoteEventD1Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
#[doc(alias = "RBX::RemoteEvent::~RemoteEvent()")]
pub fn stub_91e978() -> ! {
    todo!("0x91e978 RBX::RemoteEvent::~RemoteEvent()")
}

// 0x91e97c — __ZN3RBX11RemoteEventD0Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
#[doc(alias = "RBX::RemoteEvent::~RemoteEvent()")]
pub fn stub_91e97c() -> ! {
    todo!("0x91e97c RBX::RemoteEvent::~RemoteEvent()")
}

// 0x91ea1c — __ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E12getClassNameEv")]
pub fn stub_91ea1c() -> ! {
    todo!("0x91ea1c __ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E12getClassNameEv")
}

// 0x91ea2c — __ZThn32_N3RBX11RemoteEventD1Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::RemoteEvent::~RemoteEvent()")]
pub fn stub_91ea2c() -> ! {
    todo!("0x91ea2c non-virtual thunk to RBX::RemoteEvent::~RemoteEvent()")
}

// 0x91ea34 — __ZThn32_N3RBX11RemoteEventD0Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::RemoteEvent::~RemoteEvent()")]
pub fn stub_91ea34() -> ! {
    todo!("0x91ea34 non-virtual thunk to RBX::RemoteEvent::~RemoteEvent()")
}

// 0x91ead8 — __ZThn32_NK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E12getClassNameEv")]
pub fn stub_91ead8() -> ! {
    todo!("0x91ead8 __ZThn32_NK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E12getClassNameEv")
}

// 0x91eae8 — __ZThn36_N3RBX11RemoteEventD1Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::RemoteEvent::~RemoteEvent()")]
pub fn stub_91eae8() -> ! {
    todo!("0x91eae8 non-virtual thunk to RBX::RemoteEvent::~RemoteEvent()")
}

// 0x91eaf0 — __ZThn36_N3RBX11RemoteEventD0Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::RemoteEvent::~RemoteEvent()")]
pub fn stub_91eaf0() -> ! {
    todo!("0x91eaf0 non-virtual thunk to RBX::RemoteEvent::~RemoteEvent()")
}

// 0x91eb94 — __ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorD1Ev")]
pub fn stub_91eb94() -> ! {
    todo!("0x91eb94 __ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorD1Ev")
}

// 0x91eb98 — __ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorD1Ev")]
pub fn stub_91eb98() -> ! {
    todo!("0x91eb98 __ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorD1Ev")
}

// 0x91eb9c — __ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorD2Ev")]
pub fn stub_91eb9c() -> ! {
    todo!("0x91eb9c __ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorD2Ev")
}

// 0x91ec38 — __ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7Creator12getClassNameEv")]
pub fn stub_91ec38() -> ! {
    todo!("0x91ec38 __ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7Creator12getClassNameEv")
}

// 0x91ecc0 — __ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7Creator6createEv")]
pub fn stub_91ecc0() -> ! {
    todo!("0x91ecc0 __ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7Creator6createEv")
}

// 0x91ee04 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11RemoteEventEEEN5boost10shared_ptrIT_EEv // was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::RemoteEvent> RBX::Creatable<RBX::Instance>::create<RBX::RemoteEvent>(void)")]
pub fn stub_91ee04() -> ! {
    todo!("0x91ee04 boost::shared_ptr<RBX::RemoteEvent> RBX::Creatable<RBX::Instance>::create<RBX::RemoteEvent>(void)")
}

// 0x91eeb4 — __ZN5boost10shared_ptrIN3RBX11RemoteEventEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ // was: boost::shared_ptr
#[doc(alias = "rbx_core::SharedPtr<RBX::RemoteEvent>::shared_ptr<RBX::RemoteEvent,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_91eeb4() -> ! {
    todo!("0x91eeb4 boost::shared_ptr<RBX::RemoteEvent>::shared_ptr<RBX::RemoteEvent,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x91ef7c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11RemoteEventES6_EEvPKNS_10shared_ptrIT_EEPT0_ // was: boost::shared_ptr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RemoteEvent,RBX::RemoteEvent>(rbx_core::SharedPtr<RBX::RemoteEvent> const*,RBX::RemoteEvent *)const")]
pub fn stub_91ef7c() -> ! {
    todo!("0x91ef7c void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RemoteEvent,RBX::RemoteEvent>(boost::shared_ptr<RBX::RemoteEvent> const*,RBX::RemoteEvent *)const")
}

// 0x91f064 — __ZN5boost6detail12shared_countC2IPN3RBX11RemoteEventENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_91f064() -> ! {
    todo!("0x91f064 boost::detail::shared_count::shared_count<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x91f16c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11RemoteEventENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev // was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_91f16c() -> ! {
    todo!("0x91f16c boost::detail::sp_counted_impl_pd<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x91f170 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11RemoteEventENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev // was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_91f170() -> ! {
    todo!("0x91f170 boost::detail::sp_counted_impl_pd<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x91f174 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11RemoteEventENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv // was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_91f174() -> ! {
    todo!("0x91f174 boost::detail::sp_counted_impl_pd<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x91f194 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11RemoteEventENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info // was: boost::shared_ptr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_91f194() -> ! {
    todo!("0x91f194 boost::detail::sp_counted_impl_pd<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

