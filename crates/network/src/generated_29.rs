//! network generated_29 — RakNet + RBX::Network + Replicator + replica/remote expansion (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator|replica|remote (5974 total, 5826 prior filtered + 100 = 5926 filtered, 6139 prior unique + 100 = 6239 combined network crate stubs, shard BG29, EA-sorted ascending earliest gap, 148 remaining before batch, 48 after).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Boost types mapped: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> Weak, with // was: original.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

// 0xf5a6c4 — j___ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS3_18MarketplaceService12CurrencyTypeEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::remote_signal(void)")]
pub fn stub_f5a6c4() -> ! {
    todo!("0xf5a6c4 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::remote_signal(void)")
}

// 0xf5a6d4 — j___ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEibNS3_18MarketplaceService12CurrencyTypeEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~remote_signal()")]
pub fn stub_f5a6d4() -> ! {
    todo!("0xf5a6d4 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>::~remote_signal()")
}

// 0xf5a6e4 — j___ZN3rbx13remote_signalIFvSsiiEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string,int,int)>::~remote_signal()")]
pub fn stub_f5a6e4() -> ! {
    todo!("0xf5a6e4 rbx::remote_signal<void ()(std::string,int,int)>::~remote_signal()")
}

// 0xf5a6f4 — j___ZN3rbx13remote_signalIFviibEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(int,int,bool)>::remote_signal(void)")]
pub fn stub_f5a6f4() -> ! {
    todo!("0xf5a6f4 rbx::remote_signal<void ()(int,int,bool)>::remote_signal(void)")
}

// 0xf5a704 — j___ZN3rbx13remote_signalIFviibEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(int,int,bool)>::~remote_signal()")]
pub fn stub_f5a704() -> ! {
    todo!("0xf5a704 rbx::remote_signal<void ()(int,int,bool)>::~remote_signal()")
}

// 0xf5aec4 — j___ZNK3RBX10Reflection13EventDescImplILi3ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_ib
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::fireEvent(RBX::MarketplaceService*,rbx_core::SharedPtr<RBX::Instance>,int,bool)const")]
pub fn stub_f5aec4() -> ! {
    todo!("0xf5aec4 RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool)> RBX::MarketplaceService::*>::fireEvent(RBX::MarketplaceService*,boost::shared_ptr<RBX::Instance>,int,bool)const")
}

// 0xf5aed4 — j___ZNK3RBX10Reflection13EventDescImplILi3ENS_18MarketplaceServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ssii
// type: int __fastcall(int, int, std::string *, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::fireEvent(RBX::MarketplaceService*,std::string,int,int)const")]
pub fn stub_f5aed4() -> ! {
    todo!("0xf5aed4 RBX::Reflection::EventDescImpl<3,RBX::MarketplaceService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::MarketplaceService::*>::fireEvent(RBX::MarketplaceService*,std::string,int,int)const")
}

// 0xf5aee4 — j___ZNK3RBX10Reflection13EventDescImplILi4ENS_18MarketplaceServiceEFvN5boost10shared_ptrINS_8InstanceEEEibNS2_12CurrencyTypeEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPS2_S6_ibS7_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::MarketplaceService*,rbx_core::SharedPtr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)const")]
pub fn stub_f5aee4() -> ! {
    todo!("0xf5aee4 RBX::Reflection::EventDescImpl<4,RBX::MarketplaceService,void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)> RBX::MarketplaceService::*>::fireEvent(RBX::MarketplaceService*,boost::shared_ptr<RBX::Instance>,int,bool,RBX::MarketplaceService::CurrencyType)const")
}

// 0xf5ca44 — j___ZN3RBX10Reflection11Call1HelperINS_11RemoteEventEMS2_FvN5boost10shared_ptrIKNS0_5TupleEEEES7_vE4callEPS2_S9_RNS0_7VariantERKS7_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::RemoteEvent,void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,void>::call(RBX::RemoteEvent*,void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
pub fn stub_f5ca44() -> ! {
    todo!("0xf5ca44 RBX::Reflection::Call1Helper<RBX::RemoteEvent,void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,void>::call(RBX::RemoteEvent*,void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Reflection::Tuple const> const&)")
}

// 0xf5ca54 — j___ZN3RBX10Reflection11Call2HelperINS_11RemoteEventEMS2_FvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEES6_S9_vE4callEPS2_SB_RNS0_7VariantERKS6_RKS9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::RemoteEvent,void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,void>::call(RBX::RemoteEvent*,void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
pub fn stub_f5ca54() -> ! {
    todo!("0xf5ca54 RBX::Reflection::Call2Helper<RBX::RemoteEvent,void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,void>::call(RBX::RemoteEvent*,void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::Tuple const> const&)")
}

// 0xf5ca84 — j___ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_f5ca84() -> ! {
    todo!("0xf5ca84 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf5ca94 — j___ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EEC2EMS2_FvS7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::BoundFuncDesc(void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5ca94() -> ! {
    todo!("0xf5ca94 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::BoundFuncDesc(void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Reflection::Tuple const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5caa4 — j___ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEELi2EE16declareSignatureEPKcNS0_7VariantESD_SE_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_f5caa4() -> ! {
    todo!("0xf5caa4 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0xf5cab4 — j___ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEELi2EEC2EMS2_FvS6_S9_EPKcSF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),2>::BoundFuncDesc(void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5cab4() -> ! {
    todo!("0xf5cab4 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),2>::BoundFuncDesc(void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5cae4 — j___ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEENS3_INS_8InstanceEEES6_EEC2INS_14RemoteFunctionEEEPKcMT_NS2_8functionIS9_EESE_SE_MSF_FvvENS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::RemoteFunction>(char const*,boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,void (RBX::RemoteFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5cae4() -> ! {
    todo!("0xf5cae4 RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::RemoteFunction>(char const*,boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,void (RBX::RemoteFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5caf4 — j___ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EEC2INS_14RemoteFunctionEEEPKcMT_NS2_8functionIS7_EESC_MSD_FvvENS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::RemoteFunction>(char const*,boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,void (RBX::RemoteFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5caf4() -> ! {
    todo!("0xf5caf4 RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::RemoteFunction>(char const*,boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,void (RBX::RemoteFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5cb04 — j___ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEENS4_INS_8InstanceEEES7_ES7_Li2EE16declareSignatureEPKcNS0_7VariantESD_SE_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_f5cb04() -> ! {
    todo!("0xf5cb04 RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0xf5cb14 — j___ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEENS4_INS_8InstanceEEES7_ES7_Li2EEC2EMS2_FvS9_S7_NS3_8functionIFvS7_EEENSC_IFvSsEEEEPKcSK_SK_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,2>::BoundYieldFuncDesc(void (RBX::RemoteFunction::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5cb14() -> ! {
    todo!("0xf5cb14 RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,2>::BoundYieldFuncDesc(void (RBX::RemoteFunction::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5cb24 — j___ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EE16declareSignatureEPKcNS0_7VariantE
// type: int()
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_f5cb24() -> ! {
    todo!("0xf5cb24 RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf5cb34 — j___ZN3RBX10Reflection18BoundYieldFuncDescINS_14RemoteFunctionEFN5boost10shared_ptrIKNS0_5TupleEEES7_ES7_Li1EEC2EMS2_FvS7_NS3_8functionIFvS7_EEENSA_IFvSsEEEEPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::RemoteFunction::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5cb34() -> ! {
    todo!("0xf5cb34 RBX::Reflection::BoundYieldFuncDesc<RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::RemoteFunction::*)(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5cb64 — j___ZN3RBX10Reflection9EventDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEMS2_SC_EC2ESD_PKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::EventDesc(RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5cb64() -> ! {
    todo!("0xf5cb64 RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::EventDesc(RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5cb74 — j___ZN3RBX10Reflection9EventDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEMS2_SE_EC2ESF_PKcSI_SI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::EventDesc(RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5cb74() -> ! {
    todo!("0xf5cb74 RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::EventDesc(RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5cb84 — j___ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5cb84() -> ! {
    todo!("0xf5cb84 RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5cb94 — j___ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEMS2_SD_EC2ESE_PKcSH_SH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5cb94() -> ! {
    todo!("0xf5cb94 RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5cba4 — j___ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5cba4() -> ! {
    todo!("0xf5cba4 RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf5cbb4 — j___ZN3RBX11RemoteEventD2Ev
// type: void __fastcall(RBX::RemoteEvent *__hidden this)
#[doc(alias = "RBX::RemoteEvent::~RemoteEvent()")]
pub fn stub_f5cbb4() -> ! {
    todo!("0xf5cbb4 RBX::RemoteEvent::~RemoteEvent()")
}

// 0xf5cbc4 — j___ZN3RBX13LatchedSignalIN3rbx13remote_signalEFvN5boost10shared_ptrIKNS_10Reflection5TupleEEEEE5fire1IS8_EEvT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::fire1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_f5cbc4() -> ! {
    todo!("0xf5cbc4 void RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::fire1<boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0xf5cbd4 — j___ZN3RBX13LatchedSignalIN3rbx13remote_signalEFvN5boost10shared_ptrIKNS_10Reflection5TupleEEEEE7connectINS3_8functionIS9_EEEENS1_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)")]
pub fn stub_f5cbd4() -> ! {
    todo!("0xf5cbd4 rbx::signals::connection RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>(boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> const&)")
}

// 0xf5cbe4 — j___ZN3RBX13LatchedSignalIN3rbx13remote_signalEFvN5boost10shared_ptrIKNS_10Reflection5TupleEEEEEclIS8_EEvT_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "void RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::operator()<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_f5cbe4() -> ! {
    todo!("0xf5cbe4 void RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::operator()<boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0xf5cbf4 — j___ZN3RBX13LatchedSignalIN3rbx13remote_signalEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS_10Reflection5TupleEEEEE5fire2IS6_SA_EEvT_T0_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::fire2<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_f5cbf4() -> ! {
    todo!("0xf5cbf4 void RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::fire2<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0xf5cc04 — j___ZN3RBX13LatchedSignalIN3rbx13remote_signalEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS_10Reflection5TupleEEEEE7connectINS3_8functionISB_EEEENS1_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)")]
pub fn stub_f5cc04() -> ! {
    todo!("0xf5cc04 rbx::signals::connection RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> const&)")
}

// 0xf5cc14 — j___ZN3RBX13LatchedSignalIN3rbx13remote_signalEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS_10Reflection5TupleEEEEEclIS6_SA_EEvT_T0_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::operator()<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_f5cc14() -> ! {
    todo!("0xf5cc14 void RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::operator()<boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0xf5cc24 — j___ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E17static_getCreatorEv
// type: int()
#[doc(alias = "j___ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E17static_getCreatorEv")]
pub fn stub_f5cc24() -> ! {
    todo!("0xf5cc24 j___ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E17static_getCreatorEv")
}

// 0xf5cc34 — j___ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorC2Ev")]
pub fn stub_f5cc34() -> ! {
    todo!("0xf5cc34 j___ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorC2Ev")
}

// 0xf5cc44 — j___ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorD2Ev
// type: int()
#[doc(alias = "j___ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorD2Ev")]
pub fn stub_f5cc44() -> ! {
    todo!("0xf5cc44 j___ZN3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7CreatorD2Ev")
}

// 0xf5cc54 — j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E17static_getCreatorEv
// type: int()
#[doc(alias = "j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E17static_getCreatorEv")]
pub fn stub_f5cc54() -> ! {
    todo!("0xf5cc54 j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E17static_getCreatorEv")
}

// 0xf5cc64 — j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorC2Ev")]
pub fn stub_f5cc64() -> ! {
    todo!("0xf5cc64 j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorC2Ev")
}

// 0xf5cc74 — j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorD2Ev
// type: int()
#[doc(alias = "j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorD2Ev")]
pub fn stub_f5cc74() -> ! {
    todo!("0xf5cc74 j___ZN3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7CreatorD2Ev")
}

// 0xf5cc84 — j___ZN3RBX14RemoteFunctionD2Ev
// type: void __fastcall(RBX::RemoteFunction *__hidden this)
#[doc(alias = "RBX::RemoteFunction::~RemoteFunction()")]
pub fn stub_f5cc84() -> ! {
    todo!("0xf5cc84 RBX::RemoteFunction::~RemoteFunction()")
}

// 0xf5cc94 — j___ZN3RBX4Name9doDeclareILZNS_12sRemoteEventEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sRemoteEventEEEERKS0_v")]
pub fn stub_f5cc94() -> ! {
    todo!("0xf5cc94 j___ZN3RBX4Name9doDeclareILZNS_12sRemoteEventEEEERKS0_v")
}

// 0xf5cca4 — j___ZN3RBX4Name9doDeclareILZNS_15sRemoteFunctionEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_15sRemoteFunctionEEEERKS0_v")]
pub fn stub_f5cca4() -> ! {
    todo!("0xf5cca4 j___ZN3RBX4Name9doDeclareILZNS_15sRemoteFunctionEEEERKS0_v")
}

// 0xf5ccb4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_11RemoteEventEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::RemoteEvent> RBX::Creatable<RBX::Instance>::create<RBX::RemoteEvent>(void)")]
pub fn stub_f5ccb4() -> ! {
    todo!("0xf5ccb4 boost::shared_ptr<RBX::RemoteEvent> RBX::Creatable<RBX::Instance>::create<RBX::RemoteEvent>(void)")
}

// 0xf5ccc4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_14RemoteFunctionEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::RemoteFunction> RBX::Creatable<RBX::Instance>::create<RBX::RemoteFunction>(void)")]
pub fn stub_f5ccc4() -> ! {
    todo!("0xf5ccc4 boost::shared_ptr<RBX::RemoteFunction> RBX::Creatable<RBX::Instance>::create<RBX::RemoteFunction>(void)")
}

// 0xf5cce4 — j___ZN3rbx13remote_signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::remote_signal(void)")]
pub fn stub_f5cce4() -> ! {
    todo!("0xf5cce4 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::remote_signal(void)")
}

// 0xf5ccf4 — j___ZN3rbx13remote_signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::~remote_signal()")]
pub fn stub_f5ccf4() -> ! {
    todo!("0xf5ccf4 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::~remote_signal()")
}

// 0xf5cd04 — j___ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS2_IKNS3_10Reflection5TupleEEEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::remote_signal(void)")]
pub fn stub_f5cd04() -> ! {
    todo!("0xf5cd04 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::remote_signal(void)")
}

// 0xf5cd14 — j___ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS2_IKNS3_10Reflection5TupleEEEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::~remote_signal()")]
pub fn stub_f5cd14() -> ! {
    todo!("0xf5cd14 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::~remote_signal()")
}

// 0xf5cd24 — j___ZN3rbx13remote_signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::remote_signal(void)")]
pub fn stub_f5cd24() -> ! {
    todo!("0xf5cd24 rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::remote_signal(void)")
}

// 0xf5cd34 — j___ZN3rbx13remote_signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::~remote_signal()")]
pub fn stub_f5cd34() -> ! {
    todo!("0xf5cd34 rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::~remote_signal()")
}

// 0xf5cd44 — j___ZN3rbx13remote_signalIFviN5boost10shared_ptrIN3RBX8InstanceEEENS2_IKNS3_10Reflection5TupleEEEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::remote_signal(void)")]
pub fn stub_f5cd44() -> ! {
    todo!("0xf5cd44 rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::remote_signal(void)")
}

// 0xf5cd54 — j___ZN3rbx13remote_signalIFviN5boost10shared_ptrIN3RBX8InstanceEEENS2_IKNS3_10Reflection5TupleEEEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::~remote_signal()")]
pub fn stub_f5cd54() -> ! {
    todo!("0xf5cd54 rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::~remote_signal()")
}

// 0xf5cd64 — j___ZN3rbx13remote_signalIFviSsEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(int,std::string)>::remote_signal(void)")]
pub fn stub_f5cd64() -> ! {
    todo!("0xf5cd64 rbx::remote_signal<void ()(int,std::string)>::remote_signal(void)")
}

// 0xf5cd74 — j___ZN3rbx13remote_signalIFviSsEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(int,std::string)>::~remote_signal()")]
pub fn stub_f5cd74() -> ! {
    todo!("0xf5cd74 rbx::remote_signal<void ()(int,std::string)>::~remote_signal()")
}

// 0xf5cf24 — j___ZN3rbx7signals6signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_14RemoteFunctionEiS8_EENSC_5list3INSC_5valueIPSG_EENS2_3argILi1EEENSM_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int()
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_f5cf24() -> ! {
    todo!("0xf5cf24 rbx::signals::connection rbx::signals::signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0xf5d034 — j___ZN3rbx7signals6signalIFviSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int()
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_f5d034() -> ! {
    todo!("0xf5d034 rbx::signals::connection rbx::signals::signal<void ()(int,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0xf5d114 — j___ZN5boost10shared_ptrIN3RBX11RemoteEventEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::RemoteEvent>::shared_ptr<RBX::RemoteEvent,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f5d114() -> ! {
    todo!("0xf5d114 boost::shared_ptr<RBX::RemoteEvent>::shared_ptr<RBX::RemoteEvent,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf5d124 — j___ZN5boost10shared_ptrIN3RBX14RemoteFunctionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::RemoteFunction>::shared_ptr<RBX::RemoteFunction,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteFunction *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f5d124() -> ! {
    todo!("0xf5d124 boost::shared_ptr<RBX::RemoteFunction>::shared_ptr<RBX::RemoteFunction,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteFunction *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf5d1e4 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS3_10Reflection5TupleEEEEEEEENS2_ISB_EEEC2ESF_SG_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>::list2(boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>)")]
pub fn stub_f5d1e4() -> ! {
    todo!("0xf5d1e4 boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>::list2(boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>)")
}

// 0xf5d1f4 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS3_10Reflection5TupleEEEEEEEENS2_ISB_EEEclINS_4_mfi3mf1IvSD_SB_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>::operator()<boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>> &,boost::_bi::list0 &,int)")]
pub fn stub_f5d1f4() -> ! {
    todo!("0xf5d1f4 void boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>::operator()<boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>> &,boost::_bi::list0 &,int)")
}

// 0xf5d224 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS3_8InstanceEEENS7_IKNS3_10Reflection5TupleEEEEEEEENS2_IS9_EENS2_ISD_EEEC2ESH_SI_SJ_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>::list3(boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>)")]
pub fn stub_f5d224() -> ! {
    todo!("0xf5d224 boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>::list3(boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>)")
}

// 0xf5d234 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS3_8InstanceEEENS7_IKNS3_10Reflection5TupleEEEEEEEENS2_IS9_EENS2_ISD_EEEclINS_4_mfi3mf2IvSF_S9_SD_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>::operator()<boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>> &,boost::_bi::list0 &,int)")]
pub fn stub_f5d234() -> ! {
    todo!("0xf5d234 void boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>::operator()<boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>> &,boost::_bi::list0 &,int)")
}

// 0xf5d244 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX14RemoteFunctionEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_iNS_10shared_ptrIKNS3_10Reflection5TupleEEEEENS0_5list2IRiRSI_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<int &,rbx_core::SharedPtr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>> &,boost::_bi::list2<int &,rbx_core::SharedPtr<RBX::Reflection::Tuple const>&> &,int)")]
pub fn stub_f5d244() -> ! {
    todo!("0xf5d244 void boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<int &,boost::shared_ptr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>> &,boost::_bi::list2<int &,boost::shared_ptr<RBX::Reflection::Tuple const>&> &,int)")
}

// 0xf5d254 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX14RemoteFunctionEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_iSsEENS0_5list2IRiRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: int()
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list2<int &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string> &,boost::_bi::list2<int &,std::string &> &,int)")]
pub fn stub_f5d254() -> ! {
    todo!("0xf5d254 void boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list2<int &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string> &,boost::_bi::list2<int &,std::string &> &,int)")
}

// 0xf5d274 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS3_13SystemAddressEEENS2_IiEENS_3argILi1EEEEclINS_4_mfi3mf3IvS4_S7_iNS_10shared_ptrIKNS3_10Reflection5TupleEEEEENS0_5list1IRSK_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<RBX::SystemAddress>,boost::_bi::value<int>,boost::arg<1>>::operator()<boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>&> &,int)")]
pub fn stub_f5d274() -> ! {
    todo!("0xf5d274 void boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<RBX::SystemAddress>,boost::_bi::value<int>,boost::arg<1>>::operator()<boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list1<boost::shared_ptr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,boost::shared_ptr<RBX::Reflection::Tuple const>> &,boost::_bi::list1<boost::shared_ptr<RBX::Reflection::Tuple const>&> &,int)")
}

// 0xf5d284 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS3_13SystemAddressEEENS2_IiEENS_3argILi1EEEEclINS_4_mfi3mf3IvS4_S7_iSsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: int()
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<RBX::SystemAddress>,boost::_bi::value<int>,boost::arg<1>>::operator()<boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string> &,boost::_bi::list1<std::string &> &,int)")]
pub fn stub_f5d284() -> ! {
    todo!("0xf5d284 void boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<RBX::SystemAddress>,boost::_bi::value<int>,boost::arg<1>>::operator()<boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string> &,boost::_bi::list1<std::string &> &,int)")
}

// 0xf5d294 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrIKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSB_EEEEENS2_INSD_IFvSsEEEEEEC2ES6_SC_SG_SJ_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list4(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_f5d294() -> ! {
    todo!("0xf5d294 boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list4(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")
}

// 0xf5d2a4 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrIKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSB_EEEEENS2_INSD_IFvSsEEEEEEclINS_4_mfi3mf3IvS4_SB_SF_SI_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, char, int, int, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>> &,boost::_bi::list0 &,int)")]
pub fn stub_f5d2a4() -> ! {
    todo!("0xf5d2a4 void boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>> &,boost::_bi::list0 &,int)")
}

// 0xf5d2b4 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrINS3_8InstanceEEEEENS2_INS7_IKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSE_EEEEENS2_INSG_IFvSsEEEEEEC2ES6_SA_SF_SJ_SM_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_f5d2b4() -> ! {
    todo!("0xf5d2b4 boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")
}

// 0xf5d2c4 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrINS3_8InstanceEEEEENS2_INS7_IKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSE_EEEEENS2_INSG_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_S9_SE_SI_SL_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(char, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>> &,boost::_bi::list0 &,int)")]
pub fn stub_f5d2c4() -> ! {
    todo!("0xf5d2c4 void boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>> &,boost::_bi::list0 &,int)")
}

// 0xf5d314 — j___ZN5boost3_bi8storage3INS0_5valueIPN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS3_8InstanceEEENS7_IKNS3_10Reflection5TupleEEEEEEEENS2_IS9_EENS2_ISD_EEEC2ESH_SI_SJ_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>::storage3(boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>)")]
pub fn stub_f5d314() -> ! {
    todo!("0xf5d314 boost::_bi::storage3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>::storage3(boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>)")
}

// 0xf5d324 — j___ZN5boost3_bi8storage3INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrIKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSB_EEEEEEC2ES6_SC_SG_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::storage3(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
pub fn stub_f5d324() -> ! {
    todo!("0xf5d324 boost::_bi::storage3<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::storage3(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)")
}

// 0xf5d334 — j___ZN5boost3_bi8storage3INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrINS3_8InstanceEEEEENS2_INS7_IKNS3_10Reflection5TupleEEEEEEC2ES6_SA_SF_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>::storage3(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>)")]
pub fn stub_f5d334() -> ! {
    todo!("0xf5d334 boost::_bi::storage3<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>::storage3(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>)")
}

// 0xf5d344 — j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrIKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSB_EEEEENS2_INSD_IFvSsEEEEEEC2ERKSK_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_f5d344() -> ! {
    todo!("0xf5d344 boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")
}

// 0xf5d354 — j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrIKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSB_EEEEENS2_INSD_IFvSsEEEEEEC2ES6_SC_SG_SJ_
// type: int __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_f5d354() -> ! {
    todo!("0xf5d354 boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")
}

// 0xf5d364 — j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrINS3_8InstanceEEEEENS2_INS7_IKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSE_EEEEEEC2ERKSK_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::storage4(boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>> const&)")]
pub fn stub_f5d364() -> ! {
    todo!("0xf5d364 boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::storage4(boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>> const&)")
}

// 0xf5d374 — j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrINS3_8InstanceEEEEENS2_INS7_IKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSE_EEEEEEC2ES6_SA_SF_SJ_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::storage4(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
pub fn stub_f5d374() -> ! {
    todo!("0xf5d374 boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::storage4(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)")
}

// 0xf5d384 — j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrINS3_8InstanceEEEEENS2_INS7_IKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSE_EEEEEED2Ev
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::~storage4()")]
pub fn stub_f5d384() -> ! {
    todo!("0xf5d384 boost::_bi::storage4<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::~storage4()")
}

// 0xf5d394 — j___ZN5boost3_bi8storage5INS0_5valueIPN3RBX14RemoteFunctionEEENS2_INS_10shared_ptrINS3_8InstanceEEEEENS2_INS7_IKNS3_10Reflection5TupleEEEEENS2_INS_8functionIFvSE_EEEEENS2_INSG_IFvSsEEEEEEC2ES6_SA_SF_SJ_SM_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_f5d394() -> ! {
    todo!("0xf5d394 boost::_bi::storage5<boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<RBX::RemoteFunction *>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")
}

// 0xf5d414 — j___ZN5boost4bindIvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS1_10Reflection5TupleEEEEEES9_PSB_S9_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISF_T0_T1_EENSD_9list_av_2IT2_T3_E4typeEEEMSI_FSF_SJ_ESM_SN_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list_av_2<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::type> boost::bind<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(void (RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_f5d414() -> ! {
    todo!("0xf5d414 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list_av_2<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*,boost::shared_ptr<RBX::Reflection::Tuple const>>::type> boost::bind<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*,boost::shared_ptr<RBX::Reflection::Tuple const>>(void (RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::*)(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0xf5d424 — j___ZN5boost4bindIvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS1_8InstanceEEENS5_IKNS1_10Reflection5TupleEEEEEES7_SB_PSD_S7_SB_EENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list_av_3<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::type> boost::bind<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(void (RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_f5d424() -> ! {
    todo!("0xf5d424 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list_av_3<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>::type> boost::bind<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>(void (RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0xf5d434 — j___ZN5boost4bindIvN3RBX14RemoteFunctionENS_10shared_ptrIKNS1_10Reflection5TupleEEENS_8functionIFvS7_EEENS8_IFvSsEEEPS2_S7_SA_SC_EENS_3_bi6bind_tIT_NS_4_mfi3mf3ISG_T0_T1_T2_T3_EENSE_9list_av_4IT4_T5_T6_T7_E4typeEEEMSJ_FSG_SK_SL_SM_ESP_SQ_SR_SS_
// type: int __fastcall(int, int, char, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_4<RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>,RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>(void (RBX::RemoteFunction::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_f5d434() -> ! {
    todo!("0xf5d434 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_4<RBX::RemoteFunction*,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>,RBX::RemoteFunction*,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>(void (RBX::RemoteFunction::*)(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),RBX::RemoteFunction*,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0xf5d444 — j___ZN5boost4bindIvN3RBX14RemoteFunctionENS_10shared_ptrINS1_8InstanceEEENS3_IKNS1_10Reflection5TupleEEENS_8functionIFvS9_EEENSA_IFvSsEEEPS2_S5_S9_SC_SE_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISI_T0_T1_T2_T3_T4_EENSG_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSL_FSI_SM_SN_SO_SP_ESS_ST_SU_SV_SW_
// type: int __fastcall(int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>,RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>(void (RBX::RemoteFunction::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_f5d444() -> ! {
    todo!("0xf5d444 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::RemoteFunction*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>,RBX::RemoteFunction*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>(void (RBX::RemoteFunction::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),RBX::RemoteFunction*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0xf5d454 — j___ZN5boost6detail12shared_countC2IPN3RBX11RemoteEventENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f5d454() -> ! {
    todo!("0xf5d454 boost::detail::shared_count::shared_count<RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteEvent *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf5d464 — j___ZN5boost6detail12shared_countC2IPN3RBX14RemoteFunctionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RemoteFunction *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteFunction *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f5d464() -> ! {
    todo!("0xf5d464 boost::detail::shared_count::shared_count<RBX::RemoteFunction *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RemoteFunction *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf5d484 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS7_10Reflection5TupleEEEEEESF_EENS3_5list2INS3_5valueIPSH_EENSK_ISF_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_f5d484() -> ! {
    todo!("0xf5d484 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf5d4c4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS7_8InstanceEEENSB_IKNS7_10Reflection5TupleEEEEEESD_SH_EENS3_5list3INS3_5valueIPSJ_EENSM_ISD_EENSM_ISH_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_f5d4c4() -> ! {
    todo!("0xf5d4c4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf5d4e4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS7_10Reflection5TupleEEENS_8functionIFvSD_EEENSE_IFvSsEEEEENS3_5list4INS3_5valueIPS8_EENSL_ISD_EENSL_ISG_EENSL_ISI_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_f5d4e4() -> ! {
    todo!("0xf5d4e4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf5d4f4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS7_8InstanceEEENS9_IKNS7_10Reflection5TupleEEENS_8functionIFvSF_EEENSG_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENSN_ISB_EENSN_ISF_EENSN_ISI_EENSN_ISK_EEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_f5d4f4() -> ! {
    todo!("0xf5d4f4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf5d5b4 — j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS8_10Reflection5TupleEEEEEESG_EENS4_5list2INS4_5valueIPSI_EENSL_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS8_10Reflection5TupleEEEEEESG_EENS4_5list2INS4_5valueIPSI_EENSL_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_f5d5b4() -> ! {
    todo!("0xf5d5b4 j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS8_10Reflection5TupleEEEEEESG_EENS4_5list2INS4_5valueIPSI_EENSL_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

// 0xf5d5c4 — j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS8_8InstanceEEENSC_IKNS8_10Reflection5TupleEEEEEESE_SI_EENS4_5list3INS4_5valueIPSK_EENSN_ISE_EENSN_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS8_8InstanceEEENSC_IKNS8_10Reflection5TupleEEEEEESE_SI_EENS4_5list3INS4_5valueIPSK_EENSN_ISE_EENSN_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_f5d5c4() -> ! {
    todo!("0xf5d5c4 j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS8_8InstanceEEENSC_IKNS8_10Reflection5TupleEEEEEESE_SI_EENS4_5list3INS4_5valueIPSK_EENSN_ISE_EENSN_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")
}

// 0xf5d5d4 — j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS8_10Reflection5TupleEEENS0_IFvSE_EEENS0_IFvSsEEEEENS4_5list4INS4_5valueIPS9_EENSL_ISE_EENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS8_10Reflection5TupleEEENS0_IFvSE_EEENS0_IFvSsEEEEENS4_5list4INS4_5valueIPS9_EENSL_ISE_EENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_f5d5d4() -> ! {
    todo!("0xf5d5d4 j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS8_10Reflection5TupleEEENS0_IFvSE_EEENS0_IFvSsEEEEENS4_5list4INS4_5valueIPS9_EENSL_ISE_EENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

// 0xf5d5e4 — j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS8_8InstanceEEENSA_IKNS8_10Reflection5TupleEEENS0_IFvSG_EEENS0_IFvSsEEEEENS4_5list5INS4_5valueIPS9_EENSN_ISC_EENSN_ISG_EENSN_ISI_EENSN_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS8_8InstanceEEENSA_IKNS8_10Reflection5TupleEEENS0_IFvSG_EEENS0_IFvSsEEEEENS4_5list5INS4_5valueIPS9_EENSN_ISC_EENSN_ISG_EENSN_ISI_EENSN_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_f5d5e4() -> ! {
    todo!("0xf5d5e4 j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS8_8InstanceEEENSA_IKNS8_10Reflection5TupleEEENS0_IFvSG_EEENS0_IFvSsEEEEENS4_5list5INS4_5valueIPS9_EENSN_ISC_EENSN_ISG_EENSN_ISI_EENSN_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")
}

// 0xf5d5f4 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS7_10Reflection5TupleEEEEEESF_EENS3_5list2INS3_5valueIPSH_EENSK_ISF_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>)")]
pub fn stub_f5d5f4() -> ! {
    todo!("0xf5d5f4 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>)")
}

// 0xf5d604 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS7_8InstanceEEENSB_IKNS7_10Reflection5TupleEEEEEESD_SH_EENS3_5list3INS3_5valueIPSJ_EENSM_ISD_EENSM_ISH_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>)")]
pub fn stub_f5d604() -> ! {
    todo!("0xf5d604 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>)")
}

// 0xf5d614 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS7_10Reflection5TupleEEENS_8functionIFvSD_EEENSE_IFvSsEEEEENS3_5list4INS3_5valueIPS8_EENSL_ISD_EENSL_ISG_EENSL_ISI_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_f5d614() -> ! {
    todo!("0xf5d614 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")
}

// 0xf5d624 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS7_8InstanceEEENS9_IKNS7_10Reflection5TupleEEENS_8functionIFvSF_EEENSG_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENSN_ISB_EENSN_ISF_EENSN_ISI_EENSN_ISK_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_f5d624() -> ! {
    todo!("0xf5d624 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")
}

// 0xf5d634 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS7_10Reflection5TupleEEEEEESF_EENS3_5list2INS3_5valueIPSH_EENSK_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS7_10Reflection5TupleEEEEEESF_EENS3_5list2INS3_5valueIPSH_EENSK_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_f5d634() -> ! {
    todo!("0xf5d634 j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS7_10Reflection5TupleEEEEEESF_EENS3_5list2INS3_5valueIPSH_EENSK_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0xf5d644 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS7_8InstanceEEENSB_IKNS7_10Reflection5TupleEEEEEESD_SH_EENS3_5list3INS3_5valueIPSJ_EENSM_ISD_EENSM_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS7_8InstanceEEENSB_IKNS7_10Reflection5TupleEEEEEESD_SH_EENS3_5list3INS3_5valueIPSJ_EENSM_ISD_EENSM_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_f5d644() -> ! {
    todo!("0xf5d644 j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS7_8InstanceEEENSB_IKNS7_10Reflection5TupleEEEEEESD_SH_EENS3_5list3INS3_5valueIPSJ_EENSM_ISD_EENSM_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

// 0xf5d654 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS7_10Reflection5TupleEEENS_8functionIFvSD_EEENSE_IFvSsEEEEENS3_5list4INS3_5valueIPS8_EENSL_ISD_EENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS7_10Reflection5TupleEEENS_8functionIFvSD_EEENSE_IFvSsEEEEENS3_5list4INS3_5valueIPS8_EENSL_ISD_EENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_f5d654() -> ! {
    todo!("0xf5d654 j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS7_10Reflection5TupleEEENS_8functionIFvSD_EEENSE_IFvSsEEEEENS3_5list4INS3_5valueIPS8_EENSL_ISD_EENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

// 0xf5d664 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS7_8InstanceEEENS9_IKNS7_10Reflection5TupleEEENS_8functionIFvSF_EEENSG_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENSN_ISB_EENSN_ISF_EENSN_ISI_EENSN_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS7_8InstanceEEENS9_IKNS7_10Reflection5TupleEEENS_8functionIFvSF_EEENSG_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENSN_ISB_EENSN_ISF_EENSN_ISI_EENSN_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_f5d664() -> ! {
    todo!("0xf5d664 j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS7_8InstanceEEENS9_IKNS7_10Reflection5TupleEEENS_8functionIFvSF_EEENSG_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENSN_ISB_EENSN_ISF_EENSN_ISI_EENSN_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")
}
