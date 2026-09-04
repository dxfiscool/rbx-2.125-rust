//! audio fmod dedup — 100 stubs FMOD|Soundscape not in global dedup (0x-prefixed)
//! Source: ida/export.json filtered FMOD|Soundscape (2398 funcs, 124 not in global_eas.txt, batch 100 EA-sorted asc)
//! Range 0x7f964c..0xf30984 | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x7f964c — __ZN3RBX10Soundscape14CollisionSound4PlayEPN4FMOD6SystemEPNS2_12ChannelGroupERKN3G3D7Vector3ESA_f
// type: _DWORD __fastcall(RBX::Soundscape::CollisionSound *__hidden this, FMOD::System *, FMOD::ChannelGroup *, const G3D::Vector3 *, const G3D::Vector3 *, boost::detail::sp_counted_base *)
#[doc(alias = "RBX::Soundscape::CollisionSound::Play(FMOD::System *,FMOD::ChannelGroup *,G3D::Vector3 const&,G3D::Vector3 const&,float)")]
#[doc(alias = "__ZN3RBX10Soundscape14CollisionSound4PlayEPN4FMOD6SystemEPNS2_12ChannelGroupERKN3G3D7Vector3ESA_f")]
pub fn stub_0x7f964c() -> ! {
    todo!("0x7f964c RBX::Soundscape::CollisionSound::Play(FMOD::System *,FMOD::ChannelGroup *,G3D::Vector3 const&,G3D::Vector3 const&,float)")
}

// 0xb29978 — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator6createEv
// type: void __fastcall(RBX::Soundscape::SoundService **, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, pthread_mutex_t *, int, int, void *, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator6createEv")]
pub fn stub_0xb29978() -> ! {
    todo!("0xb29978 __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator6createEv")
}

// 0xf1ffa8 — __ZN4FMOD7Channel7setMuteEb$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN4FMOD7Channel7setMuteEb$shim")]
pub fn stub_0xf1ffa8() -> ! {
    todo!("0xf1ffa8 __ZN4FMOD7Channel7setMuteEb$shim")
}

// 0xf1ffcc — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv$shim")]
pub fn stub_0xf1ffcc() -> ! {
    todo!("0xf1ffcc __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv$shim")
}

// 0xf1ffd8 — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv$shim")]
pub fn stub_0xf1ffd8() -> ! {
    todo!("0xf1ffd8 __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv$shim")
}

// 0xf1ffe4 — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev$shim
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev$shim")]
pub fn stub_0xf1ffe4() {
    // IDA 0xf1ffe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf1fff0 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev$shim
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev$shim")]
pub fn stub_0xf1fff0() {
    // IDA 0xf1fff0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf20008 — __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v$shim")]
pub fn stub_0xf20008() -> ! {
    todo!("0xf20008 __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v$shim")
}

// 0xf20020 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_$shim")]
pub fn stub_0xf20020() -> ! {
    todo!("0xf20020 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_$shim")
}

// 0xf20044 — __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv$shim")]
pub fn stub_0xf20044() -> ! {
    todo!("0xf20044 __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv$shim")
}

// 0xf20050 — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
pub fn stub_0xf20050() -> ! {
    todo!("0xf20050 __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")
}

// 0xf20bcc — __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD2Ev$shim")]
pub fn stub_0xf20bcc() {
    // IDA 0xf20bcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf20dd0 — __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v$shim
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v$shim")]
pub fn stub_0xf20dd0() -> ! {
    todo!("0xf20dd0 __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v$shim")
}

// 0xf20ddc — __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v$shim")]
pub fn stub_0xf20ddc() -> ! {
    todo!("0xf20ddc __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v$shim")
}

// 0xf245a4 — __ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v$shim")]
pub fn stub_0xf245a4() -> ! {
    todo!("0xf245a4 __ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v$shim")
}

// 0xf30314 — j___ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE
// type: _DWORD __fastcall(SoundServiceStatsItem *__hidden this, const RBX::Soundscape::SoundService *)
#[doc(alias = "SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")]
#[doc(alias = "j___ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE")]
pub fn stub_0xf30314() -> ! {
    todo!("0xf30314 SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")
}

// 0xf30324 — j___ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE
// type: SoundServiceStatsItem *__fastcall(SoundServiceStatsItem *__hidden this, const RBX::Soundscape::SoundService *)
#[doc(alias = "SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")]
#[doc(alias = "j___ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE")]
pub fn stub_0xf30324() -> ! {
    todo!("0xf30324 SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")
}

// 0xf30334 — j___ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::BoundFuncDesc(void (RBX::Soundscape::SoundChannel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xf30334() -> ! {
    todo!("0xf30334 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::BoundFuncDesc(void (RBX::Soundscape::SoundChannel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf30344 — j___ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0xf30344() -> ! {
    todo!("0xf30344 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf30354 — j___ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EEC2EMS3_FvS4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::BoundFuncDesc(void (RBX::Soundscape::SoundService::*)(RBX::SoundType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EEC2EMS3_FvS4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xf30354() -> ! {
    todo!("0xf30354 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::BoundFuncDesc(void (RBX::Soundscape::SoundService::*)(RBX::SoundType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf30364 — j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEEC2IMS3_KFS4_vEMS3_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::PropDescriptor<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>(char const*,char const*,RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEEC2IMS3_KFS4_vEMS3_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf30364() -> ! {
    todo!("0xf30364 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::PropDescriptor<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>(char const*,char const*,RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf30374 — j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEMS3_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEMS3_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf30374() -> ! {
    todo!("0xf30374 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf30384 — j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,int>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf30384() -> ! {
    todo!("0xf30384 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,int>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf30394 — j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfEC2IMS3_KFfvEMS3_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::PropDescriptor<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>(char const*,char const*,float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfEC2IMS3_KFfvEMS3_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf30394() -> ! {
    todo!("0xf30394 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::PropDescriptor<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>(char const*,char const*,float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf303a4 — j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiEC2IMS3_KFivEMS3_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::PropDescriptor<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>(char const*,char const*,int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiEC2IMS3_KFivEMS3_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf303a4() -> ! {
    todo!("0xf303a4 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::PropDescriptor<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>(char const*,char const*,int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf303b4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEEC2IMS3_KFS4_vEMS3_FvRKS4_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::EnumPropDescriptor<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>(char const*,char const*,RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEEC2IMS3_KFS4_vEMS3_FvRKS4_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf303b4() -> ! {
    todo!("0xf303b4 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::EnumPropDescriptor<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>(char const*,char const*,RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf303c4 — j___ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf303c4() -> ! {
    todo!("0xf303c4 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf303d4 — j___ZN3RBX10Reflection7Variant14genericConvertINS_10Soundscape7SoundIdEEERT_v
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")]
#[doc(alias = "j___ZN3RBX10Reflection7Variant14genericConvertINS_10Soundscape7SoundIdEEERT_v")]
pub fn stub_0xf303d4() -> ! {
    todo!("0xf303d4 RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")
}

// 0xf303e4 — j___ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE7addPairES3_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbType,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE7addPairES3_PKc")]
pub fn stub_0xf303e4() -> ! {
    todo!("0xf303e4 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbType,char const*)")
}

// 0xf303f4 — j___ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED2Ev")]
pub fn stub_0xf303f4() {
    // IDA 0xf303f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf30424 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundChannelEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundChannel>(char const*,char const*,bool RBX::Soundscape::SoundChannel::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundChannelEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf30424() -> ! {
    todo!("0xf30424 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundChannel>(char const*,char const*,bool RBX::Soundscape::SoundChannel::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf30434 — j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundServiceEEEPKcS8_MT_fMS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundService>(char const*,char const*,float RBX::Soundscape::SoundService::*,void (RBX::Soundscape::SoundService::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundServiceEEEPKcS8_MT_fMS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf30434() -> ! {
    todo!("0xf30434 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundService>(char const*,char const*,float RBX::Soundscape::SoundService::*,void (RBX::Soundscape::SoundService::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf30444 — j___ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf30444() -> ! {
    todo!("0xf30444 j___ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0xf30454 — j___ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf30454() -> ! {
    todo!("0xf30454 j___ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0xf30464 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Soundscape10ReverbTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Soundscape10ReverbTypeEEEE14doGetSingletonEv")]
pub fn stub_0xf30464() -> ! {
    todo!("0xf30464 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType> const>::doGetSingleton(void)")
}

// 0xf30474 — j___ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_
// type: _DWORD __fastcall(RBX::Soundscape::SoundService::SoundJob *__hidden this, RBX::Soundscape::SoundService *)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")]
#[doc(alias = "j___ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_")]
pub fn stub_0xf30474() -> ! {
    todo!("0xf30474 RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")
}

// 0xf30484 — j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv")]
pub fn stub_0xf30484() -> ! {
    todo!("0xf30484 j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv")
}

// 0xf30494 — j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev")]
pub fn stub_0xf30494() -> ! {
    todo!("0xf30494 j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev")
}

// 0xf304a4 — j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev")]
pub fn stub_0xf304a4() {
    // IDA 0xf304a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf304b4 — j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0xf304b4() -> ! {
    todo!("0xf304b4 j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv")
}

// 0xf304c4 — j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0xf304c4() -> ! {
    todo!("0xf304c4 j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev")
}

// 0xf304d4 — j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0xf304d4() {
    // IDA 0xf304d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf304f4 — j___ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v")]
pub fn stub_0xf304f4() -> ! {
    todo!("0xf304f4 j___ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v")
}

// 0xf30564 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape10ReverbTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::ReverbType>(RBX::Soundscape::ReverbType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape10ReverbTypeEEERS3_RKT_")]
pub fn stub_0xf30564() -> ! {
    todo!("0xf30564 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::ReverbType>(RBX::Soundscape::ReverbType const&)")
}

// 0xf30574 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_")]
pub fn stub_0xf30574() -> ! {
    todo!("0xf30574 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")
}

// 0xf30584 — j___ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv
// type: int(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv")]
pub fn stub_0xf30584() -> ! {
    todo!("0xf30584 rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")
}

// 0xf30594 — j___ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE9singletonEv
// type: int(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE9singletonEv")]
pub fn stub_0xf30594() -> ! {
    todo!("0xf30594 rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::singleton(void)")
}

// 0xf305a4 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf305a4() -> ! {
    todo!("0xf305a4 rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>> const&)")
}

// 0xf305b4 — j___ZN3rbx8any_castIN3RBX10Soundscape7SoundIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Soundscape::SoundId * rbx::any_cast<RBX::Soundscape::SoundId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX10Soundscape7SoundIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_0xf305b4() -> ! {
    todo!("0xf305b4 RBX::Soundscape::SoundId * rbx::any_cast<RBX::Soundscape::SoundId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0xf305c4 — j___ZN3rbx8any_castIRKN3RBX10Soundscape10ReverbTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Soundscape::ReverbType const& rbx::any_cast<RBX::Soundscape::ReverbType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX10Soundscape10ReverbTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf305c4() -> ! {
    todo!("0xf305c4 RBX::Soundscape::ReverbType const& rbx::any_cast<RBX::Soundscape::ReverbType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf305d4 — j___ZN3rbx8any_castIRKN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Soundscape::SoundId const& rbx::any_cast<RBX::Soundscape::SoundId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf305d4() -> ! {
    todo!("0xf305d4 RBX::Soundscape::SoundId const& rbx::any_cast<RBX::Soundscape::SoundId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf305e4 — j___ZN3rbx8any_castIRN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Soundscape::SoundId & rbx::any_cast<RBX::Soundscape::SoundId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf305e4() -> ! {
    todo!("0xf305e4 RBX::Soundscape::SoundId & rbx::any_cast<RBX::Soundscape::SoundId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf305f4 — j___ZN5boost10scoped_ptrIN3RBX10Soundscape21CollisionSoundManagerEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::scoped_ptr<RBX::Soundscape::CollisionSoundManager>::~scoped_ptr()")]
#[doc(alias = "j___ZN5boost10scoped_ptrIN3RBX10Soundscape21CollisionSoundManagerEED2Ev")]
pub fn stub_0xf305f4() {
    // IDA 0xf305f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf30624 — j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>& rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE")]
pub fn stub_0xf30624() -> ! {
    todo!("0xf30624 boost::shared_ptr<RBX::Soundscape::SoundChannel>& boost::shared_ptr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(boost::shared_ptr<RBX::StockSound> const&)")
}

// 0xf30634 — j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEC2IS4_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::shared_ptr<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEC2IS4_EEPT_")]
pub fn stub_0xf30634() -> ! {
    todo!("0xf30634 boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob>::shared_ptr<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")
}

// 0xf30644 — j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::operator=(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_")]
pub fn stub_0xf30644() -> ! {
    todo!("0xf30644 boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob>::operator=(boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob> const&)")
}

// 0xf30654 — j___ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEC2IS3_EEPT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::Sound>::shared_ptr<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEC2IS3_EEPT_")]
pub fn stub_0xf30654() -> ! {
    todo!("0xf30654 boost::shared_ptr<RBX::Soundscape::Sound>::shared_ptr<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")
}

// 0xf30664 — j___ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEaSERKS4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::Sound>::operator=(rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEaSERKS4_")]
pub fn stub_0xf30664() -> ! {
    todo!("0xf30664 boost::shared_ptr<RBX::Soundscape::Sound>::operator=(boost::shared_ptr<RBX::Soundscape::Sound> const&)")
}

// 0xf30694 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_")]
pub fn stub_0xf30694() -> ! {
    todo!("0xf30694 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")
}

// 0xf306a4 — j___ZN5boost6detail12shared_countC2IN3RBX10Soundscape12SoundService8SoundJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX10Soundscape12SoundService8SoundJobEEEPT_")]
pub fn stub_0xf306a4() {
    // IDA 0xf306a4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf306b4 — j___ZN5boost6detail12shared_countC2IN3RBX10Soundscape5SoundEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX10Soundscape5SoundEEEPT_")]
pub fn stub_0xf306b4() {
    // IDA 0xf306b4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf30704 — j___ZN9__gnu_cxx13new_allocatorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEE7destroyEPSA_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>::destroy(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>*)")]
#[doc(alias = "j___ZN9__gnu_cxx13new_allocatorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEE7destroyEPSA_")]
pub fn stub_0xf30704() -> ! {
    todo!("0xf30704 __gnu_cxx::new_allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>::destroy(std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>*)")
}

// 0xf30714 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf30714() -> ! {
    todo!("0xf30714 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf30724 — j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE13convertToItemERKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToItem(RBX::Soundscape::ReverbType const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE13convertToItemERKS3_")]
pub fn stub_0xf30724() -> ! {
    todo!("0xf30724 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToItem(RBX::Soundscape::ReverbType const&)const")
}

// 0xf30734 — j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToIndex(RBX::Soundscape::ReverbType)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToIndexES3_")]
pub fn stub_0xf30734() -> ! {
    todo!("0xf30734 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToIndex(RBX::Soundscape::ReverbType)const")
}

// 0xf30744 — j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToValue(RBX::Name const&,RBX::Soundscape::ReverbType&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf30744() -> ! {
    todo!("0xf30744 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToValue(RBX::Name const&,RBX::Soundscape::ReverbType&)const")
}

// 0xf30754 — j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToString(RBX::Soundscape::ReverbType const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE15convertToStringERKS3_")]
pub fn stub_0xf30754() -> ! {
    todo!("0xf30754 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToString(RBX::Soundscape::ReverbType const&)const")
}

// 0xf30764 — j___ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv
// type: int __fastcall(_DWORD)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv")]
pub fn stub_0xf30764() -> ! {
    todo!("0xf30764 j___ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv")
}

// 0xf30774 — j___ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv
// type: int __fastcall(_DWORD)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0xf30774() -> ! {
    todo!("0xf30774 j___ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0xf30794 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundChannelES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundChannel>(rbx_core::SharedPtr<RBX::Soundscape::SoundChannel> const*,RBX::Soundscape::SoundChannel *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundChannelES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf30794() {
    // IDA 0xf30794: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf307b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10Soundscape12SoundService8SoundJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Soundscape::SoundService::SoundJob,RBX::Soundscape::SoundService::SoundJob>(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const*,RBX::Soundscape::SoundService::SoundJob *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10Soundscape12SoundService8SoundJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf307b4() {
    // IDA 0xf307b4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf307c4 — j___ZNSt12_Vector_baseIN3RBX10Soundscape10ReverbTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Vector_base<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX10Soundscape10ReverbTypeESaIS2_EE11_M_allocateEm")]
pub fn stub_0xf307c4() -> ! {
    todo!("0xf307c4 std::_Vector_base<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_allocate(unsigned long)")
}

// 0xf307d4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Soundscape10ReverbTypeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Soundscape::ReverbType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *>(RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Soundscape10ReverbTypeES6_EET0_T_S8_S7_")]
pub fn stub_0xf307d4() -> ! {
    todo!("0xf307d4 RBX::Soundscape::ReverbType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *>(RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *)")
}

// 0xf307e4 — j___ZNSt3mapIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::operator[](RBX::Soundscape::SoundId const&)")]
#[doc(alias = "j___ZNSt3mapIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_")]
pub fn stub_0xf307e4() -> ! {
    todo!("0xf307e4 std::map<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::operator[](RBX::Soundscape::SoundId const&)")
}

// 0xf307f4 — j___ZNSt3mapIN3RBX9SoundTypeEN5boost10shared_ptrINS0_10Soundscape12SoundChannelEEESt4lessIS1_ESaISt4pairIKS1_S6_EEEixERSA_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::SoundType,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::operator[](RBX::SoundType const&)")]
#[doc(alias = "j___ZNSt3mapIN3RBX9SoundTypeEN5boost10shared_ptrINS0_10Soundscape12SoundChannelEEESt4lessIS1_ESaISt4pairIKS1_S6_EEEixERSA_")]
pub fn stub_0xf307f4() -> ! {
    todo!("0xf307f4 std::map<RBX::SoundType,boost::shared_ptr<RBX::Soundscape::SoundChannel>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::operator[](RBX::SoundType const&)")
}

// 0xf30804 — j___ZNSt3mapIPKN3RBX4NameENS0_10Soundscape10ReverbTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::Soundscape::ReverbType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_10Soundscape10ReverbTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0xf30804() -> ! {
    todo!("0xf30804 std::map<RBX::Name const*,RBX::Soundscape::ReverbType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::operator[](RBX::Name const* const&)")
}

// 0xf30814 — j___ZNSt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2ERS3_RKS7_
#[doc(alias = "std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>::pair(RBX::Soundscape::SoundId const&,rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")]
#[doc(alias = "j___ZNSt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2ERS3_RKS7_")]
pub fn stub_0xf30814() -> ! {
    todo!("0xf30814 std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>::pair(RBX::Soundscape::SoundId const&,boost::shared_ptr<RBX::Soundscape::Sound> const&)")
}

// 0xf30824 — j___ZNSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2IKS2_S6_EERKS_IT_T0_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::pair<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>(std::pair const&<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)")]
#[doc(alias = "j___ZNSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2IKS2_S6_EERKS_IT_T0_E")]
pub fn stub_0xf30824() -> ! {
    todo!("0xf30824 std::pair<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>(std::pair const&<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>)")
}

// 0xf30834 — j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,RBX::Soundscape::ReverbType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0xf30834() -> ! {
    todo!("0xf30834 std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,RBX::Soundscape::ReverbType const&)")
}

// 0xf30844 — j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(_DWORD)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,unsigned long,RBX::Soundscape::ReverbType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0xf30844() -> ! {
    todo!("0xf30844 std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,unsigned long,RBX::Soundscape::ReverbType const&)")
}

// 0xf30854 — j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::resize(unsigned long,RBX::Soundscape::ReverbType)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE6resizeEmS2_")]
pub fn stub_0xf30854() -> ! {
    todo!("0xf30854 std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::resize(unsigned long,RBX::Soundscape::ReverbType)")
}

// 0xf30864 — j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::push_back(RBX::Soundscape::ReverbType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE9push_backERKS2_")]
pub fn stub_0xf30864() -> ! {
    todo!("0xf30864 std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::push_back(RBX::Soundscape::ReverbType const&)")
}

// 0xf30874 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_create_node(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_")]
pub fn stub_0xf30874() {
    // IDA 0xf30874: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0xf30884 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_")]
pub fn stub_0xf30884() -> ! {
    todo!("0xf30884 std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")
}

// 0xf30894 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
pub fn stub_0xf30894() -> ! {
    todo!("0xf30894 std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")
}

// 0xf308a4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE4findERS4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::find(RBX::Soundscape::SoundId const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE4findERS4_")]
pub fn stub_0xf308a4() -> ! {
    todo!("0xf308a4 std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::find(RBX::Soundscape::SoundId const&)")
}

// 0xf308b4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_E")]
pub fn stub_0xf308b4() -> ! {
    todo!("0xf308b4 std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>)")
}

// 0xf308c4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")]
pub fn stub_0xf308c4() {
    // IDA 0xf308c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf308d4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")]
pub fn stub_0xf308d4() -> ! {
    todo!("0xf308d4 std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")
}

// 0xf308e4 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE14_M_create_nodeERKS9_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_create_node(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE14_M_create_nodeERKS9_")]
pub fn stub_0xf308e4() {
    // IDA 0xf308e4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0xf308f4 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E")]
pub fn stub_0xf308f4() {
    // IDA 0xf308f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30904 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueERKS9_")]
pub fn stub_0xf30904() -> ! {
    todo!("0xf30904 std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")
}

// 0xf30914 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
pub fn stub_0xf30914() -> ! {
    todo!("0xf30914 std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")
}

// 0xf30924 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")]
pub fn stub_0xf30924() {
    // IDA 0xf30924: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30934 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")]
pub fn stub_0xf30934() -> ! {
    todo!("0xf30934 std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")
}

// 0xf30944 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0xf30944() -> ! {
    todo!("0xf30944 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")
}

// 0xf30954 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0xf30954() -> ! {
    todo!("0xf30954 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")
}

// 0xf30964 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0xf30964() {
    // IDA 0xf30964: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30974 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0xf30974() -> ! {
    todo!("0xf30974 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")
}

// 0xf30984 — j___ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEEPFvRKS1_IS4_S9_EEET0_T_SI_SH_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int)
#[doc(alias = "void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>) std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)>(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>))")]
#[doc(alias = "j___ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEEPFvRKS1_IS4_S9_EEET0_T_SI_SH_")]
pub fn stub_0xf30984() -> ! {
    todo!("0xf30984 void (*)(std::pair const&<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>) std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>)>(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>))")
}
