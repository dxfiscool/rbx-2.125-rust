//! audio generated — 400 stubs sorted by EA, from ida/export.json
//! Filter: FMOD/Sound/Audio (2541 total, 2541 distinct EA) — 2011 distinct EA (2157 stubs) prior in lib.rs + 400 this file = 2411 distinct (2557 stubs) total, 130 distinct remaining
//! Batch: 0x376198..0xf304a4 | SharedPtr = rbx_core::SharedPtr (Arc) not boost::shared_ptr | strip '

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — type alias mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x376198 — __ZN3RBX13registerSoundEv
// type: int __fastcall(RBX *this)
#[doc(alias = "RBX::registerSound(void)")]
pub fn stub_376198() -> ! {
    todo!("0x376198 RBX::registerSound(void)")
}

// 0x37677c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10StockSoundEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(__guard *)
#[doc(alias = "boost::shared_ptr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")]
pub fn stub_37677c() -> ! {
    todo!("0x37677c boost::shared_ptr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")
}

// 0x376a24 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::SoundChannel>& boost::shared_ptr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(boost::shared_ptr<RBX::StockSound> const&)")]
pub fn stub_376a24() -> ! {
    todo!("0x376a24 boost::shared_ptr<RBX::Soundscape::SoundChannel>& boost::shared_ptr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(boost::shared_ptr<RBX::StockSound> const&)")
}

// 0x376a58 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob>::operator=(boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob> const&)")]
pub fn stub_376a58() -> ! {
    todo!("0x376a58 boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob>::operator=(boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob> const&)")
}

// 0x376a90 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI21SoundServiceStatsItemEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<SoundServiceStatsItem>(boost::shared_ptr<SoundServiceStatsItem> const&)")]
pub fn stub_376a90() -> ! {
    todo!("0x376a90 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<SoundServiceStatsItem>(boost::shared_ptr<SoundServiceStatsItem> const&)")
}

// 0x376ac4 — __ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE
// type: void __fastcall(RBX::Stats::Item **this, const RBX::Soundscape::SoundService *)
#[doc(alias = "SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")]
pub fn stub_376ac4() -> ! {
    todo!("0x376ac4 SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")
}

// 0x376c84 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), const std::string *))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")]
pub fn stub_376c84() -> ! {
    todo!("0x376c84 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")
}

// 0x376ce4 — __ZN3RBX10Reflection7Variant14genericConvertINS_10Soundscape7SoundIdEEERT_v
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")]
pub fn stub_376ce4() -> ! {
    todo!("0x376ce4 RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")
}

// 0x376f90 — __ZN3RBX10Soundscape12SoundService18on3DSettingChangedERKNS_10Reflection18PropertyDescriptorE
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Soundscape::SoundService::on3DSettingChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_376f90() -> ! {
    todo!("0x376f90 RBX::Soundscape::SoundService::on3DSettingChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x376fb8 — __ZNK3RBX10Soundscape12SoundService16getAmbientReverbEv
// type: int __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::getAmbientReverb(void)const")]
pub fn stub_376fb8() -> ! {
    todo!("0x376fb8 RBX::Soundscape::SoundService::getAmbientReverb(void)const")
}

// 0x376fc0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")]
pub fn stub_376fc0() -> ! {
    todo!("0x376fc0 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")
}

// 0x376fe4 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")]
pub fn stub_376fe4() -> ! {
    todo!("0x376fe4 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")
}

// 0x377024 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::~PropDescriptor()")]
pub fn stub_377024() -> ! {
    todo!("0x377024 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::~PropDescriptor()")
}

// 0x377048 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::~PropDescriptor()")]
pub fn stub_377048() -> ! {
    todo!("0x377048 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::~PropDescriptor()")
}

// 0x37706c — __ZNK3RBX10Soundscape12SoundChannel12getPlayCountEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getPlayCount(void)const")]
pub fn stub_37706c() -> ! {
    todo!("0x37706c RBX::Soundscape::SoundChannel::getPlayCount(void)const")
}

// 0x377074 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::~PropDescriptor()")]
pub fn stub_377074() -> ! {
    todo!("0x377074 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::~PropDescriptor()")
}

// 0x377098 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::~PropDescriptor()")]
pub fn stub_377098() -> ! {
    todo!("0x377098 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::~PropDescriptor()")
}

// 0x3770bc — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_3770bc() -> ! {
    todo!("0x3770bc RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::~BoundFuncDesc()")
}

// 0x3770e0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>> const&)")]
pub fn stub_3770e0() -> ! {
    todo!("0x3770e0 rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>> const&)")
}

// 0x377154 — __ZN3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")]
pub fn stub_377154() -> ! {
    todo!("0x377154 RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")
}

// 0x37716c — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEaSERKS4_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::Sound>::operator=(boost::shared_ptr<RBX::Soundscape::Sound> const&)")]
pub fn stub_37716c() -> ! {
    todo!("0x37716c boost::shared_ptr<RBX::Soundscape::Sound>::operator=(boost::shared_ptr<RBX::Soundscape::Sound> const&)")
}

// 0x3771a4 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_3771a4() -> ! {
    todo!("0x3771a4 __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x3772c0 — __ZNSt3mapIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(int, const std::string *)
#[doc(alias = "std::map<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::operator[](RBX::Soundscape::SoundId const&)")]
pub fn stub_3772c0() -> ! {
    todo!("0x3772c0 std::map<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::operator[](RBX::Soundscape::SoundId const&)")
}

// 0x37750c — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")]
pub fn stub_37750c() -> ! {
    todo!("0x37750c __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")
}

// 0x37751c — __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")]
pub fn stub_37751c() -> ! {
    todo!("0x37751c __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")
}

// 0x37752c — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")]
pub fn stub_37752c() -> ! {
    todo!("0x37752c __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")
}

// 0x37753c — __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")]
pub fn stub_37753c() -> ! {
    todo!("0x37753c __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")
}

// 0x37754c — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_37754c() -> ! {
    todo!("0x37754c __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev")
}

// 0x377550 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev")]
pub fn stub_377550() -> ! {
    todo!("0x377550 __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev")
}

// 0x377554 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
pub fn stub_377554() -> ! {
    todo!("0x377554 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")
}

// 0x377558 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
pub fn stub_377558() -> ! {
    todo!("0x377558 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")
}

// 0x3775f8 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(char const*)const")]
pub fn stub_3775f8() -> ! {
    todo!("0x3775f8 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(char const*)const")
}

// 0x377628 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_377628() -> ! {
    todo!("0x377628 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x377648 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_377648() -> ! {
    todo!("0x377648 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x3776a4 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToString(unsigned long,std::string &)const")]
pub fn stub_3776a4() -> ! {
    todo!("0x3776a4 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToString(unsigned long,std::string &)const")
}

// 0x3777e8 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToString(RBX::Soundscape::ReverbType const&)const")]
pub fn stub_3777e8() -> ! {
    todo!("0x3777e8 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToString(RBX::Soundscape::ReverbType const&)const")
}

// 0x377988 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape10ReverbTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::ReverbType>(RBX::Soundscape::ReverbType const&)")]
pub fn stub_377988() -> ! {
    todo!("0x377988 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::ReverbType>(RBX::Soundscape::ReverbType const&)")
}

// 0x3779d8 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")]
pub fn stub_3779d8() -> ! {
    todo!("0x3779d8 rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")
}

// 0x377a44 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::construct_func(char const*,char *)")]
pub fn stub_377a44() -> ! {
    todo!("0x377a44 rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::construct_func(char const*,char *)")
}

// 0x377a50 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::destruct_func(char *)")]
pub fn stub_377a50() -> ! {
    todo!("0x377a50 rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::destruct_func(char *)")
}

// 0x377a54 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToItem(RBX::Soundscape::ReverbType const&)const")]
pub fn stub_377a54() -> ! {
    todo!("0x377a54 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToItem(RBX::Soundscape::ReverbType const&)const")
}

// 0x377b20 — __ZN3rbx8any_castIRKN3RBX10Soundscape10ReverbTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Soundscape::ReverbType const& rbx::any_cast<RBX::Soundscape::ReverbType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_377b20() -> ! {
    todo!("0x377b20 RBX::Soundscape::ReverbType const& rbx::any_cast<RBX::Soundscape::ReverbType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x377c10 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToValue(RBX::Name const&,RBX::Soundscape::ReverbType&)const")]
pub fn stub_377c10() -> ! {
    todo!("0x377c10 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToValue(RBX::Name const&,RBX::Soundscape::ReverbType&)const")
}

// 0x377c8c — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
pub fn stub_377c8c() -> ! {
    todo!("0x377c8c RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")
}

// 0x377e60 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev")]
pub fn stub_377e60() -> ! {
    todo!("0x377e60 __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev")
}

// 0x377efc — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv")]
pub fn stub_377efc() -> ! {
    todo!("0x377efc __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv")
}

// 0x377f84 — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator6createEv")]
pub fn stub_377f84() -> ! {
    todo!("0x377f84 __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator6createEv")
}

// 0x3780c8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10Soundscape12SoundChannelEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::SoundChannel> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundChannel>(void)")]
pub fn stub_3780c8() -> ! {
    todo!("0x3780c8 boost::shared_ptr<RBX::Soundscape::SoundChannel> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundChannel>(void)")
}

// 0x378178 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::SoundChannel>::shared_ptr<RBX::Soundscape::SoundChannel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_378178() -> ! {
    todo!("0x378178 boost::shared_ptr<RBX::Soundscape::SoundChannel>::shared_ptr<RBX::Soundscape::SoundChannel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x378240 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundChannelES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundChannel>(boost::shared_ptr<RBX::Soundscape::SoundChannel> const*,RBX::Soundscape::SoundChannel *)const")]
pub fn stub_378240() -> ! {
    todo!("0x378240 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundChannel>(boost::shared_ptr<RBX::Soundscape::SoundChannel> const*,RBX::Soundscape::SoundChannel *)const")
}

// 0x37832c — __ZN5boost6detail12shared_countC2IPN3RBX10Soundscape12SoundChannelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37832c() -> ! {
    todo!("0x37832c boost::detail::shared_count::shared_count<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x378434 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_378434() -> ! {
    todo!("0x378434 boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x378438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_378438() -> ! {
    todo!("0x378438 boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37843c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_37843c() -> ! {
    todo!("0x37843c boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x37845c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_37845c() -> ! {
    todo!("0x37845c boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x378474 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_378474() -> ! {
    todo!("0x378474 boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x378478 — __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv")]
pub fn stub_378478() -> ! {
    todo!("0x378478 __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv")
}

// 0x37847c — __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v")]
pub fn stub_37847c() -> ! {
    todo!("0x37847c __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v")
}

// 0x37855c — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev")]
pub fn stub_37855c() -> ! {
    todo!("0x37855c __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev")
}

// 0x3787a0 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv")]
pub fn stub_3787a0() -> ! {
    todo!("0x3787a0 __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv")
}

// 0x378814 — __ZNSt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2ERS3_RKS7_
// type: _DWORD *__fastcall(_DWORD *, const std::string *, const shared_count *)
#[doc(alias = "std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>::pair(RBX::Soundscape::SoundId const&,boost::shared_ptr<RBX::Soundscape::Sound> const&)")]
pub fn stub_378814() -> ! {
    todo!("0x378814 std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>::pair(RBX::Soundscape::SoundId const&,boost::shared_ptr<RBX::Soundscape::Sound> const&)")
}

// 0x3788dc — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_3788dc() -> ! {
    todo!("0x3788dc std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")
}

// 0x3789c4 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_3789c4() -> ! {
    todo!("0x3789c4 std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")
}

// 0x378a14 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_378a14() -> ! {
    todo!("0x378a14 std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")
}

// 0x378a94 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_create_node(std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_378a94() -> ! {
    todo!("0x378a94 std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_create_node(std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")
}

// 0x378ba0 — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::Sound>::shared_ptr<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
pub fn stub_378ba0() -> ! {
    todo!("0x378ba0 boost::shared_ptr<RBX::Soundscape::Sound>::shared_ptr<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")
}

// 0x378c74 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape5SoundEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
pub fn stub_378c74() -> ! {
    todo!("0x378c74 boost::detail::shared_count::shared_count<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")
}

// 0x378d80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::~sp_counted_impl_p()")]
pub fn stub_378d80() -> ! {
    todo!("0x378d80 boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::~sp_counted_impl_p()")
}

// 0x378d84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::~sp_counted_impl_p()")]
pub fn stub_378d84() -> ! {
    todo!("0x378d84 boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::~sp_counted_impl_p()")
}

// 0x378d88 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::dispose(void)")]
pub fn stub_378d88() -> ! {
    todo!("0x378d88 boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::dispose(void)")
}

// 0x378e2c — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::get_deleter(std::type_info const&)")]
pub fn stub_378e2c() -> ! {
    todo!("0x378e2c boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::get_deleter(std::type_info const&)")
}

// 0x378e30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::get_untyped_deleter(void)")]
pub fn stub_378e30() -> ! {
    todo!("0x378e30 boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::get_untyped_deleter(void)")
}

// 0x378e34 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE4findERS4_
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::find(RBX::Soundscape::SoundId const&)")]
pub fn stub_378e34() -> ! {
    todo!("0x378e34 std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::find(RBX::Soundscape::SoundId const&)")
}

// 0x378e84 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Soundscape10ReverbTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType> const>::initSingleton(void)")]
pub fn stub_378e84() -> ! {
    todo!("0x378e84 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType> const>::initSingleton(void)")
}

// 0x378e88 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Soundscape10ReverbTypeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType> const>::doGetSingleton(void)")]
pub fn stub_378e88() -> ! {
    todo!("0x378e88 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType> const>::doGetSingleton(void)")
}

// 0x378f78 — __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_378f78() -> ! {
    todo!("0x378f78 __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x379094 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_379094() -> ! {
    todo!("0x379094 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3790c0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_3790c0() -> ! {
    todo!("0x3790c0 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x379194 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub fn stub_379194() -> ! {
    todo!("0x379194 rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")
}

// 0x37919c — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub fn stub_37919c() -> ! {
    todo!("0x37919c `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")
}

// 0x3791a4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_
// type: int __fastcall(int)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
pub fn stub_3791a4() -> ! {
    todo!("0x3791a4 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")
}

// 0x3791bc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub fn stub_3791bc() -> ! {
    todo!("0x3791bc rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")
}

// 0x3791e8 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub fn stub_3791e8() -> ! {
    todo!("0x3791e8 rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")
}

// 0x3792bc — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>)")]
pub fn stub_3792bc() -> ! {
    todo!("0x3792bc std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>)")
}

// 0x3792e4 — __ZN9__gnu_cxx13new_allocatorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEE7destroyEPSA_
// type: void __fastcall(int, std::string *)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>::destroy(std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>*)")]
pub fn stub_3792e4() -> ! {
    todo!("0x3792e4 __gnu_cxx::new_allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>::destroy(std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>*)")
}

// 0x379388 — __ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_379388() -> ! {
    todo!("0x379388 __ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x37938c — __ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37938c() -> ! {
    todo!("0x37938c __ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x37942c — __ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37942c() -> ! {
    todo!("0x37942c __ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x379434 — __ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_379434() -> ! {
    todo!("0x379434 __ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3794d8 — __ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3794d8() -> ! {
    todo!("0x3794d8 __ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3794e0 — __ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3794e0() -> ! {
    todo!("0x3794e0 __ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x379584 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_379584() -> ! {
    todo!("0x379584 __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x379588 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_379588() -> ! {
    todo!("0x379588 __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x379628 — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_379628() -> ! {
    todo!("0x379628 __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x379630 — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_379630() -> ! {
    todo!("0x379630 __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3796d4 — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3796d4() -> ! {
    todo!("0x3796d4 __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3796dc — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3796dc() -> ! {
    todo!("0x3796dc __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x379780 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::BoundFuncDesc(void (RBX::Soundscape::SoundChannel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_379780() -> ! {
    todo!("0x379780 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::BoundFuncDesc(void (RBX::Soundscape::SoundChannel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x379884 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_379884() -> ! {
    todo!("0x379884 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::~BoundFuncDesc()")
}

// 0x379938 — __ZNK3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_379938() -> ! {
    todo!("0x379938 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x379958 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundChannelEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundChannel>(char const*,char const*,bool RBX::Soundscape::SoundChannel::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_379958() -> ! {
    todo!("0x379958 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundChannel>(char const*,char const*,bool RBX::Soundscape::SoundChannel::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x379ae8 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::isReadOnly(void)const")]
pub fn stub_379ae8() -> ! {
    todo!("0x379ae8 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::isReadOnly(void)const")
}

// 0x379aec — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::isWriteOnly(void)const")]
pub fn stub_379aec() -> ! {
    todo!("0x379aec RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::isWriteOnly(void)const")
}

// 0x379af0 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_379af0() -> ! {
    todo!("0x379af0 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x379afc — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_379afc() -> ! {
    todo!("0x379afc RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x379b4c — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,int>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_379b4c() -> ! {
    todo!("0x379b4c RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,int>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x379c58 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::~PropDescriptor()")]
pub fn stub_379c58() -> ! {
    todo!("0x379c58 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::~PropDescriptor()")
}

// 0x379c84 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::isReadOnly(void)const")]
pub fn stub_379c84() -> ! {
    todo!("0x379c84 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::isReadOnly(void)const")
}

// 0x379c88 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_379c88() -> ! {
    todo!("0x379c88 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::isWriteOnly(void)const")
}

// 0x379c8c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_379c8c() -> ! {
    todo!("0x379c8c RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x379cb0 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_379cb0() -> ! {
    todo!("0x379cb0 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x379dd0 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEMS3_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_379dd0() -> ! {
    todo!("0x379dd0 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x379ee4 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::isReadOnly(void)const")]
pub fn stub_379ee4() -> ! {
    todo!("0x379ee4 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::isReadOnly(void)const")
}

// 0x379ee8 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_379ee8() -> ! {
    todo!("0x379ee8 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::isWriteOnly(void)const")
}

// 0x379eec — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_379eec() -> ! {
    todo!("0x379eec RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x379f10 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_379f10() -> ! {
    todo!("0x379f10 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x379f34 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiEC2IMS3_KFivEMS3_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::PropDescriptor<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>(char const*,char const*,int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_379f34() -> ! {
    todo!("0x379f34 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::PropDescriptor<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>(char const*,char const*,int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x37a048 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::~PropDescriptor()")]
pub fn stub_37a048() -> ! {
    todo!("0x37a048 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::~PropDescriptor()")
}

// 0x37a074 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::isReadOnly(void)const")]
pub fn stub_37a074() -> ! {
    todo!("0x37a074 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::isReadOnly(void)const")
}

// 0x37a078 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::isWriteOnly(void)const")]
pub fn stub_37a078() -> ! {
    todo!("0x37a078 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::isWriteOnly(void)const")
}

// 0x37a07c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37a07c() -> ! {
    todo!("0x37a07c RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x37a09c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_37a09c() -> ! {
    todo!("0x37a09c RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x37a0c0 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfEC2IMS3_KFfvEMS3_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::PropDescriptor<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>(char const*,char const*,float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_37a0c0() -> ! {
    todo!("0x37a0c0 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::PropDescriptor<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>(char const*,char const*,float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x37a1d4 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::~PropDescriptor()")]
pub fn stub_37a1d4() -> ! {
    todo!("0x37a1d4 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::~PropDescriptor()")
}

// 0x37a200 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::isReadOnly(void)const")]
pub fn stub_37a200() -> ! {
    todo!("0x37a200 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::isReadOnly(void)const")
}

// 0x37a204 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::isWriteOnly(void)const")]
pub fn stub_37a204() -> ! {
    todo!("0x37a204 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::isWriteOnly(void)const")
}

// 0x37a208 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37a208() -> ! {
    todo!("0x37a208 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x37a228 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE8setValueEPNS0_13DescribedBaseERKf
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_37a228() -> ! {
    todo!("0x37a228 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x37a24c — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEEC2IMS3_KFS4_vEMS3_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::PropDescriptor<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>(char const*,char const*,RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_37a24c() -> ! {
    todo!("0x37a24c RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::PropDescriptor<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>(char const*,char const*,RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x37a360 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int *, int, int, char, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_37a360() -> ! {
    todo!("0x37a360 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x37a484 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::~PropDescriptor()")]
pub fn stub_37a484() -> ! {
    todo!("0x37a484 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::~PropDescriptor()")
}

// 0x37a4b0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::isReadOnly(void)const")]
pub fn stub_37a4b0() -> ! {
    todo!("0x37a4b0 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::isReadOnly(void)const")
}

// 0x37a4c0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::isWriteOnly(void)const")]
pub fn stub_37a4c0() -> ! {
    todo!("0x37a4c0 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::isWriteOnly(void)const")
}

// 0x37a4d0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37a4d0() -> ! {
    todo!("0x37a4d0 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x37a67c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_37a67c() -> ! {
    todo!("0x37a67c RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x37a7a8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_37a7a8() -> ! {
    todo!("0x37a7a8 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x37a9a4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_37a9a4() -> ! {
    todo!("0x37a9a4 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x37aacc — __ZN3rbx8any_castIRKN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Soundscape::SoundId const& rbx::any_cast<RBX::Soundscape::SoundId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_37aacc() -> ! {
    todo!("0x37aacc RBX::Soundscape::SoundId const& rbx::any_cast<RBX::Soundscape::SoundId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x37abbc — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::~TypedPropertyDescriptor()")]
pub fn stub_37abbc() -> ! {
    todo!("0x37abbc RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::~TypedPropertyDescriptor()")
}

// 0x37abe0 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::~TypedPropertyDescriptor()")]
pub fn stub_37abe0() -> ! {
    todo!("0x37abe0 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::~TypedPropertyDescriptor()")
}

// 0x37ac0c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::isReadOnly(void)const")]
pub fn stub_37ac0c() -> ! {
    todo!("0x37ac0c RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::isReadOnly(void)const")
}

// 0x37ac10 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::isWriteOnly(void)const")]
pub fn stub_37ac10() -> ! {
    todo!("0x37ac10 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::isWriteOnly(void)const")
}

// 0x37ac14 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37ac14() -> ! {
    todo!("0x37ac14 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x37ac3c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::setValue(RBX::Reflection::DescribedBase *,RBX::Soundscape::SoundId const&)const")]
pub fn stub_37ac3c() -> ! {
    todo!("0x37ac3c RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::setValue(RBX::Reflection::DescribedBase *,RBX::Soundscape::SoundId const&)const")
}

// 0x37ad84 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EEC2EMS3_FvS4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::BoundFuncDesc(void (RBX::Soundscape::SoundService::*)(RBX::SoundType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_37ad84() -> ! {
    todo!("0x37ad84 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::BoundFuncDesc(void (RBX::Soundscape::SoundService::*)(RBX::SoundType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x37aefc — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_37aefc() -> ! {
    todo!("0x37aefc RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x37af2c — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")]
pub fn stub_37af2c() -> ! {
    todo!("0x37af2c RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")
}

// 0x37b000 — __ZNK3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_37b000() -> ! {
    todo!("0x37b000 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x37b034 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9SoundTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int (__fastcall ***__fastcall(int (__fastcall ***)(_DWORD), int))(_DWORD)
#[doc(alias = "RBX::SoundType RBX::Reflection::ArgHelper::getArg<RBX::SoundType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::SoundType> const&,boost::disable_if<boost::is_same<RBX::SoundType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_37b034() -> ! {
    todo!("0x37b034 RBX::SoundType RBX::Reflection::ArgHelper::getArg<RBX::SoundType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::SoundType> const&,boost::disable_if<boost::is_same<RBX::SoundType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x37b1c4 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9SoundTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINS9_7is_enumIS7_EEvE4typeE
// type: int __fastcall(int, _DWORD *, int, int)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::SoundType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::SoundType &,boost::enable_if<boost::is_enum<RBX::SoundType>,void>::type *)")]
pub fn stub_37b1c4() -> ! {
    todo!("0x37b1c4 bool RBX::Reflection::ArgHelper::try_enum<1,RBX::SoundType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::SoundType &,boost::enable_if<boost::is_enum<RBX::SoundType>,void>::type *)")
}

// 0x37b218 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEEC2IMS3_KFS4_vEMS3_FvRKS4_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::EnumPropDescriptor<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>(char const*,char const*,RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_37b218() -> ! {
    todo!("0x37b218 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::EnumPropDescriptor<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>(char const*,char const*,RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x37b3cc — __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")]
pub fn stub_37b3cc() -> ! {
    todo!("0x37b3cc RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")
}

// 0x37b3f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::isReadOnly(void)const")]
pub fn stub_37b3f8() -> ! {
    todo!("0x37b3f8 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::isReadOnly(void)const")
}

// 0x37b408 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::isWriteOnly(void)const")]
pub fn stub_37b408() -> ! {
    todo!("0x37b408 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::isWriteOnly(void)const")
}

// 0x37b418 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37b418() -> ! {
    todo!("0x37b418 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x37b440 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_37b440() -> ! {
    todo!("0x37b440 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x37b464 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_37b464() -> ! {
    todo!("0x37b464 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x37b5b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_37b5b0() -> ! {
    todo!("0x37b5b0 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x37b5d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::hasStringValue(void)const")]
pub fn stub_37b5d4() -> ! {
    todo!("0x37b5d4 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::hasStringValue(void)const")
}

// 0x37b5d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37b5d8() -> ! {
    todo!("0x37b5d8 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x37b5fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_37b5fc() -> ! {
    todo!("0x37b5fc RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x37b63c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_37b63c() -> ! {
    todo!("0x37b63c RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x37b65c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_37b65c() -> ! {
    todo!("0x37b65c RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x37b89c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37b89c() -> ! {
    todo!("0x37b89c RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x37b8b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_37b8b8() -> ! {
    todo!("0x37b8b8 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x37b8ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37b8ec() -> ! {
    todo!("0x37b8ec RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x37b8f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_37b8f4() -> ! {
    todo!("0x37b8f4 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x37b940 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37b940() -> ! {
    todo!("0x37b940 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x37b960 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_37b960() -> ! {
    todo!("0x37b960 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x37b994 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToIndexES3_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToIndex(RBX::Soundscape::ReverbType)const")]
pub fn stub_37b994() -> ! {
    todo!("0x37b994 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToIndex(RBX::Soundscape::ReverbType)const")
}

// 0x37ba04 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_37ba04() -> ! {
    todo!("0x37ba04 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x37ba44 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::isReadOnly(void)const")]
pub fn stub_37ba44() -> ! {
    todo!("0x37ba44 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::isReadOnly(void)const")
}

// 0x37ba48 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::isWriteOnly(void)const")]
pub fn stub_37ba48() -> ! {
    todo!("0x37ba48 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::isWriteOnly(void)const")
}

// 0x37ba4c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37ba4c() -> ! {
    todo!("0x37ba4c RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x37ba6c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE8setValueEPNS0_13DescribedBaseESA_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::Soundscape::ReverbType const&)const")]
pub fn stub_37ba6c() -> ! {
    todo!("0x37ba6c RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::Soundscape::ReverbType const&)const")
}

// 0x37ba90 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundServiceEEEPKcS8_MT_fMS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundService>(char const*,char const*,float RBX::Soundscape::SoundService::*,void (RBX::Soundscape::SoundService::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_37ba90() -> ! {
    todo!("0x37ba90 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundService>(char const*,char const*,float RBX::Soundscape::SoundService::*,void (RBX::Soundscape::SoundService::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x37bc24 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::isReadOnly(void)const")]
pub fn stub_37bc24() -> ! {
    todo!("0x37bc24 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::isReadOnly(void)const")
}

// 0x37bc28 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::isWriteOnly(void)const")]
pub fn stub_37bc28() -> ! {
    todo!("0x37bc28 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::isWriteOnly(void)const")
}

// 0x37bc2c — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_37bc2c() -> ! {
    todo!("0x37bc2c RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x37bc38 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE8setValueEPNS0_13DescribedBaseERKf
// type: float *__fastcall(int, int, float *)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_37bc38() -> ! {
    todo!("0x37bc38 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x37bc94 — __ZN3rbx8any_castIN3RBX10Soundscape7SoundIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::Soundscape::SoundId * rbx::any_cast<RBX::Soundscape::SoundId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_37bc94() -> ! {
    todo!("0x37bc94 RBX::Soundscape::SoundId * rbx::any_cast<RBX::Soundscape::SoundId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x37bcec — __ZN3rbx8any_castIRN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Soundscape::SoundId & rbx::any_cast<RBX::Soundscape::SoundId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_37bcec() -> ! {
    todo!("0x37bcec RBX::Soundscape::SoundId & rbx::any_cast<RBX::Soundscape::SoundId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x37bddc — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::singleton(void)")]
pub fn stub_37bddc() -> ! {
    todo!("0x37bddc rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::singleton(void)")
}

// 0x37be48 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE14construct_funcEPKcPc
// type: const std::string *__fastcall(const std::string *result, std::string *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::construct_func(char const*,char *)")]
pub fn stub_37be48() -> ! {
    todo!("0x37be48 rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::construct_func(char const*,char *)")
}

// 0x37be64 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE13destruct_funcEPc
// type: int __fastcall(int)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::destruct_func(char *)")]
pub fn stub_37be64() -> ! {
    todo!("0x37be64 rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::destruct_func(char *)")
}

// 0x37be68 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEC2IS4_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob>::shared_ptr<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
pub fn stub_37be68() -> ! {
    todo!("0x37be68 boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob>::shared_ptr<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")
}

// 0x37bf50 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10Soundscape12SoundService8SoundJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Soundscape::SoundService::SoundJob,RBX::Soundscape::SoundService::SoundJob>(boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob> const*,RBX::Soundscape::SoundService::SoundJob *)const")]
pub fn stub_37bf50() -> ! {
    todo!("0x37bf50 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Soundscape::SoundService::SoundJob,RBX::Soundscape::SoundService::SoundJob>(boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob> const*,RBX::Soundscape::SoundService::SoundJob *)const")
}

// 0x37c034 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape12SoundService8SoundJobEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
pub fn stub_37c034() -> ! {
    todo!("0x37c034 boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")
}

// 0x37c12c — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")]
pub fn stub_37c12c() -> ! {
    todo!("0x37c12c boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")
}

// 0x37c130 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")]
pub fn stub_37c130() -> ! {
    todo!("0x37c130 boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")
}

// 0x37c134 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::dispose(void)")]
pub fn stub_37c134() -> ! {
    todo!("0x37c134 boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::dispose(void)")
}

// 0x37c144 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_deleter(std::type_info const&)")]
pub fn stub_37c144() -> ! {
    todo!("0x37c144 boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_deleter(std::type_info const&)")
}

// 0x37c148 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_untyped_deleter(void)")]
pub fn stub_37c148() -> ! {
    todo!("0x37c148 boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_untyped_deleter(void)")
}

// 0x37c14c — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_37c14c() -> ! {
    todo!("0x37c14c std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")
}

// 0x37c200 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_37c200() -> ! {
    todo!("0x37c200 std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")
}

// 0x37c24c — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_37c24c() -> ! {
    todo!("0x37c24c std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")
}

// 0x37c2b4 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE14_M_create_nodeERKS9_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_create_node(std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_37c2b4() -> ! {
    todo!("0x37c2b4 std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_create_node(std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>> const&)")
}

// 0x37c3a4 — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_37c3a4() -> ! {
    todo!("0x37c3a4 __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev")
}

// 0x37c440 — __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv
// type: void
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_37c440() -> ! {
    todo!("0x37c440 __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x37c4c8 — __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(__guard *, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator6createEv")]
pub fn stub_37c4c8() -> ! {
    todo!("0x37c4c8 __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator6createEv")
}

// 0x37c60c — __ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv
// type: void
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv")]
pub fn stub_37c60c() -> ! {
    todo!("0x37c60c __ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv")
}

// 0x37c610 — __ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v")]
pub fn stub_37c610() -> ! {
    todo!("0x37c610 __ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v")
}

// 0x37c6f0 — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_37c6f0() -> ! {
    todo!("0x37c6f0 __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev")
}

// 0x37c934 — __ZN3RBX10StockSoundD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "RBX::StockSound::~StockSound()")]
pub fn stub_37c934() -> ! {
    todo!("0x37c934 RBX::StockSound::~StockSound()")
}

// 0x37c938 — __ZN3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "RBX::StockSound::~StockSound()")]
pub fn stub_37c938() -> ! {
    todo!("0x37c938 RBX::StockSound::~StockSound()")
}

// 0x37c9d8 — __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv")]
pub fn stub_37c9d8() -> ! {
    todo!("0x37c9d8 __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv")
}

// 0x37c9e8 — __ZThn32_N3RBX10StockSoundD1Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
pub fn stub_37c9e8() -> ! {
    todo!("0x37c9e8 `non-virtual thunk to'RBX::StockSound::~StockSound()")
}

// 0x37c9f0 — __ZThn32_N3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
pub fn stub_37c9f0() -> ! {
    todo!("0x37c9f0 `non-virtual thunk to'RBX::StockSound::~StockSound()")
}

// 0x37ca94 — __ZThn32_NK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv")]
pub fn stub_37ca94() -> ! {
    todo!("0x37ca94 __ZThn32_NK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv")
}

// 0x37caa4 — __ZThn36_N3RBX10StockSoundD1Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
pub fn stub_37caa4() -> ! {
    todo!("0x37caa4 `non-virtual thunk to'RBX::StockSound::~StockSound()")
}

// 0x37caac — __ZThn36_N3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
pub fn stub_37caac() -> ! {
    todo!("0x37caac `non-virtual thunk to'RBX::StockSound::~StockSound()")
}

// 0x37cb50 — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_37cb50() -> ! {
    todo!("0x37cb50 __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv")
}

// 0x37cbc4 — __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37cbc4() -> ! {
    todo!("0x37cbc4 __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x37cbc8 — __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37cbc8() -> ! {
    todo!("0x37cbc8 __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x37cc68 — __ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37cc68() -> ! {
    todo!("0x37cc68 __ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x37cc70 — __ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37cc70() -> ! {
    todo!("0x37cc70 __ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x37cd14 — __ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37cd14() -> ! {
    todo!("0x37cd14 __ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x37cd1c — __ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37cd1c() -> ! {
    todo!("0x37cd1c __ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x37cdc0 — __ZN5boost10shared_ptrIN3RBX10StockSoundEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::StockSound>::shared_ptr<RBX::StockSound,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37cdc0() -> ! {
    todo!("0x37cdc0 boost::shared_ptr<RBX::StockSound>::shared_ptr<RBX::StockSound,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37ce88 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10StockSoundES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StockSound,RBX::StockSound>(boost::shared_ptr<RBX::StockSound> const*,RBX::StockSound *)const")]
pub fn stub_37ce88() -> ! {
    todo!("0x37ce88 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StockSound,RBX::StockSound>(boost::shared_ptr<RBX::StockSound> const*,RBX::StockSound *)const")
}

// 0x37cf74 — __ZN5boost6detail12shared_countC2IPN3RBX10StockSoundENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37cf74() -> ! {
    todo!("0x37cf74 boost::detail::shared_count::shared_count<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37d07c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37d07c() -> ! {
    todo!("0x37d07c boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37d080 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37d080() -> ! {
    todo!("0x37d080 boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37d084 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_37d084() -> ! {
    todo!("0x37d084 boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x37d0a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_37d0a4() -> ! {
    todo!("0x37d0a4 boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x37d0bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_37d0bc() -> ! {
    todo!("0x37d0bc boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x37d0c0 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>> *)")]
pub fn stub_37d0c0() -> ! {
    todo!("0x37d0c0 std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>> *)")
}

// 0x37d0f0 — __ZNSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2IKS2_S6_EERKS_IT_T0_E
// type: _DWORD *__fastcall(_DWORD *, const shared_count *)
#[doc(alias = "std::pair<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>(std::pair const&<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>)")]
pub fn stub_37d0f0() -> ! {
    todo!("0x37d0f0 std::pair<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>(std::pair const&<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>)")
}

// 0x37d1b4 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>> *)")]
pub fn stub_37d1b4() -> ! {
    todo!("0x37d1b4 std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>> *)")
}

// 0x37d1dc — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>> *)")]
pub fn stub_37d1dc() -> ! {
    todo!("0x37d1dc std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>> *)")
}

// 0x37d1f8 — __ZN5boost10scoped_ptrIN3RBX10Soundscape21CollisionSoundManagerEED2Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(RBX::Soundscape::CollisionSoundManager **)
#[doc(alias = "boost::scoped_ptr<RBX::Soundscape::CollisionSoundManager>::~scoped_ptr()")]
pub fn stub_37d1f8() -> ! {
    todo!("0x37d1f8 boost::scoped_ptr<RBX::Soundscape::CollisionSoundManager>::~scoped_ptr()")
}

// 0x37d2a0 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37d2a0() -> ! {
    todo!("0x37d2a0 __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x37d2a4 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37d2a4() -> ! {
    todo!("0x37d2a4 __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x37d344 — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37d344() -> ! {
    todo!("0x37d344 __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x37d34c — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37d34c() -> ! {
    todo!("0x37d34c __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x37d3f0 — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37d3f0() -> ! {
    todo!("0x37d3f0 __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x37d3f8 — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37d3f8() -> ! {
    todo!("0x37d3f8 __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x37d49c — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::resize(unsigned long,RBX::Soundscape::ReverbType)")]
pub fn stub_37d49c() -> ! {
    todo!("0x37d49c std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::resize(unsigned long,RBX::Soundscape::ReverbType)")
}

// 0x37d4d0 — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::push_back(RBX::Soundscape::ReverbType const&)")]
pub fn stub_37d4d0() -> ! {
    todo!("0x37d4d0 std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::push_back(RBX::Soundscape::ReverbType const&)")
}

// 0x37d4f8 — __ZNSt3mapIPKN3RBX4NameENS0_10Soundscape10ReverbTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::Soundscape::ReverbType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_37d4f8() -> ! {
    todo!("0x37d4f8 std::map<RBX::Name const*,RBX::Soundscape::ReverbType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::operator[](RBX::Name const* const&)")
}

// 0x37d550 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
pub fn stub_37d550() -> ! {
    todo!("0x37d550 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")
}

// 0x37d604 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
pub fn stub_37d604() -> ! {
    todo!("0x37d604 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")
}

// 0x37d65c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
pub fn stub_37d65c() -> ! {
    todo!("0x37d65c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")
}

// 0x37d6c4 — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,RBX::Soundscape::ReverbType const&)")]
pub fn stub_37d6c4() -> ! {
    todo!("0x37d6c4 std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,RBX::Soundscape::ReverbType const&)")
}

// 0x37d7a8 — __ZNSt12_Vector_baseIN3RBX10Soundscape10ReverbTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_allocate(unsigned long)")]
pub fn stub_37d7a8() -> ! {
    todo!("0x37d7a8 std::_Vector_base<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_allocate(unsigned long)")
}

// 0x37d7c0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Soundscape10ReverbTypeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Soundscape::ReverbType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *>(RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *)")]
pub fn stub_37d7c0() -> ! {
    todo!("0x37d7c0 RBX::Soundscape::ReverbType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *>(RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *)")
}

// 0x37d7fc — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,unsigned long,RBX::Soundscape::ReverbType const&)")]
pub fn stub_37d7fc() -> ! {
    todo!("0x37d7fc std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,unsigned long,RBX::Soundscape::ReverbType const&)")
}

// 0x37d98c — __ZN3RBX9CreatableINS_8InstanceEE6createI21SoundServiceStatsItemPKNS_10Soundscape12SoundServiceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, const RBX::Soundscape::SoundService *)
#[doc(alias = "boost::shared_ptr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)")]
pub fn stub_37d98c() -> ! {
    todo!("0x37d98c boost::shared_ptr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)")
}

// 0x37de98 — __ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE
// type: void __fastcall(SoundServiceStatsItem *this, const RBX::Soundscape::SoundService *)
#[doc(alias = "SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")]
pub fn stub_37de98() -> ! {
    todo!("0x37de98 SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")
}

// 0x37e05c — __ZN21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
pub fn stub_37e05c() -> ! {
    todo!("0x37e05c SoundServiceStatsItem::~SoundServiceStatsItem()")
}

// 0x37e098 — __ZN21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
pub fn stub_37e098() -> ! {
    todo!("0x37e098 SoundServiceStatsItem::~SoundServiceStatsItem()")
}

// 0x37e16c — __ZN21SoundServiceStatsItem6updateEv
// type: void __fastcall(SoundServiceStatsItem *this)
#[doc(alias = "SoundServiceStatsItem::update(void)")]
pub fn stub_37e16c() -> ! {
    todo!("0x37e16c SoundServiceStatsItem::update(void)")
}

// 0x37e344 — __ZThn32_N21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "__ZThn32_N21SoundServiceStatsItemD1Ev")]
pub fn stub_37e344() -> ! {
    todo!("0x37e344 __ZThn32_N21SoundServiceStatsItemD1Ev")
}

// 0x37e384 — __ZThn32_N21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "__ZThn32_N21SoundServiceStatsItemD0Ev")]
pub fn stub_37e384() -> ! {
    todo!("0x37e384 __ZThn32_N21SoundServiceStatsItemD0Ev")
}

// 0x37e458 — __ZThn36_N21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "__ZThn36_N21SoundServiceStatsItemD1Ev")]
pub fn stub_37e458() -> ! {
    todo!("0x37e458 __ZThn36_N21SoundServiceStatsItemD1Ev")
}

// 0x37e498 — __ZThn36_N21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "__ZThn36_N21SoundServiceStatsItemD0Ev")]
pub fn stub_37e498() -> ! {
    todo!("0x37e498 __ZThn36_N21SoundServiceStatsItemD0Ev")
}

// 0x37e56c — __ZN5boost10shared_ptrI21SoundServiceStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37e56c() -> ! {
    todo!("0x37e56c boost::shared_ptr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37e634 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI21SoundServiceStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<SoundServiceStatsItem,SoundServiceStatsItem>(boost::shared_ptr<SoundServiceStatsItem> const*,SoundServiceStatsItem *)const")]
pub fn stub_37e634() -> ! {
    todo!("0x37e634 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<SoundServiceStatsItem,SoundServiceStatsItem>(boost::shared_ptr<SoundServiceStatsItem> const*,SoundServiceStatsItem *)const")
}

// 0x37e720 — __ZN5boost6detail12shared_countC2IP21SoundServiceStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37e720() -> ! {
    todo!("0x37e720 boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37e828 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37e828() -> ! {
    todo!("0x37e828 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37e82c — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37e82c() -> ! {
    todo!("0x37e82c boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37e830 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_37e830() -> ! {
    todo!("0x37e830 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x37e850 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_37e850() -> ! {
    todo!("0x37e850 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x37e868 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_37e868() -> ! {
    todo!("0x37e868 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x37e86c — __ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_
// type: RBX::Soundscape::SoundService::SoundJob *__fastcall(RBX::Soundscape::SoundService::SoundJob *this, RBX::Soundscape::SoundService *)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")]
pub fn stub_37e86c() -> ! {
    todo!("0x37e86c RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")
}

// 0x37e9c4 — __ZN3RBX10Soundscape12SoundService8SoundJobD1Ev
// type: void __fastcall(RBX::TaskScheduler::Job *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
pub fn stub_37e9c4() -> ! {
    todo!("0x37e9c4 RBX::Soundscape::SoundService::SoundJob::~SoundJob()")
}

// 0x37e9c8 — __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService::SoundJob *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
pub fn stub_37e9c8() -> ! {
    todo!("0x37e9c8 RBX::Soundscape::SoundService::SoundJob::~SoundJob()")
}

// 0x37ea68 — __ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Soundscape::SoundService::SoundJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_37ea68() -> ! {
    todo!("0x37ea68 RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x37ea84 — __ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_37ea84() -> ! {
    todo!("0x37ea84 RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x37eaa0 — __ZN3RBX10Soundscape12SoundService8SoundJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Soundscape::SoundService **this, const RBX::TaskScheduler::Job::Stats *, int, int (*)(const char *, ...))
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_37eaa0() -> ! {
    todo!("0x37eaa0 RBX::Soundscape::SoundService::SoundJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x37eab0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>> *)")]
pub fn stub_37eab0() -> ! {
    todo!("0x37eab0 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>> *)")
}

// 0x37f4d8 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEEC1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")]
pub fn stub_37f4d8() -> ! {
    todo!("0x37f4d8 RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")
}

// 0x37f4dc — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")]
pub fn stub_37f4dc() -> ! {
    todo!("0x37f4dc RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")
}

// 0x37f7c8 — __ZN3RBX10Reflection7Variant7convertINS_9SoundTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::convert<RBX::SoundType>(void)")]
pub fn stub_37f7c8() -> ! {
    todo!("0x37f7c8 RBX::SoundType & RBX::Reflection::Variant::convert<RBX::SoundType>(void)")
}

// 0x37f7cc — __ZN3RBX15StringConverterINS_9SoundTypeEE14convertToValueERKSsRS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::SoundType>::convertToValue(std::string const&,RBX::SoundType&)")]
pub fn stub_37f7cc() -> ! {
    todo!("0x37f7cc RBX::StringConverter<RBX::SoundType>::convertToValue(std::string const&,RBX::SoundType&)")
}

// 0x37f818 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEE7addPairES2_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::addPair(RBX::SoundType,char const*)")]
pub fn stub_37f818() -> ! {
    todo!("0x37f818 RBX::Reflection::EnumDesc<RBX::SoundType>::addPair(RBX::SoundType,char const*)")
}

// 0x37fb78 — __ZN3RBX10Reflection7Variant14genericConvertINS_9SoundTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")]
pub fn stub_37fb78() -> ! {
    todo!("0x37fb78 RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")
}

// 0x37fd64 — __ZN3rbx8any_castIN3RBX9SoundTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::SoundType * rbx::any_cast<RBX::SoundType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_37fd64() -> ! {
    todo!("0x37fd64 RBX::SoundType * rbx::any_cast<RBX::SoundType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x37fdbc — __ZN3rbx8any_castIRN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::SoundType & rbx::any_cast<RBX::SoundType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_37fdbc() -> ! {
    todo!("0x37fdbc RBX::SoundType & rbx::any_cast<RBX::SoundType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x37feac — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE6resizeEmS1_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::resize(unsigned long,RBX::SoundType)")]
pub fn stub_37feac() -> ! {
    todo!("0x37feac std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::resize(unsigned long,RBX::SoundType)")
}

// 0x37fee0 — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::push_back(RBX::SoundType const&)")]
pub fn stub_37fee0() -> ! {
    todo!("0x37fee0 std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::push_back(RBX::SoundType const&)")
}

// 0x37ff08 — __ZNSt3mapIPKN3RBX4NameENS0_9SoundTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::SoundType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_37ff08() -> ! {
    todo!("0x37ff08 std::map<RBX::Name const*,RBX::SoundType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::operator[](RBX::Name const* const&)")
}

// 0x37ff60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SoundType>>,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
pub fn stub_37ff60() -> ! {
    todo!("0x37ff60 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SoundType>>,std::pair<RBX::Name const* const,RBX::SoundType> const&)")
}

// 0x380014 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
pub fn stub_380014() -> ! {
    todo!("0x380014 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SoundType> const&)")
}

// 0x38006c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
pub fn stub_38006c() -> ! {
    todo!("0x38006c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SoundType> const&)")
}

// 0x3800d4 — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,RBX::SoundType const&)")]
pub fn stub_3800d4() -> ! {
    todo!("0x3800d4 std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,RBX::SoundType const&)")
}

// 0x3801b8 — __ZNSt12_Vector_baseIN3RBX9SoundTypeESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_allocate(unsigned long)")]
pub fn stub_3801b8() -> ! {
    todo!("0x3801b8 std::_Vector_base<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_allocate(unsigned long)")
}

// 0x3801d0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9SoundTypeES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::SoundType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SoundType *,RBX::SoundType *>(RBX::SoundType *,RBX::SoundType *,RBX::SoundType *)")]
pub fn stub_3801d0() -> ! {
    todo!("0x3801d0 RBX::SoundType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SoundType *,RBX::SoundType *>(RBX::SoundType *,RBX::SoundType *,RBX::SoundType *)")
}

// 0x38020c — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,unsigned long,RBX::SoundType const&)")]
pub fn stub_38020c() -> ! {
    todo!("0x38020c std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,unsigned long,RBX::SoundType const&)")
}

// 0x434d00 — __ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(void)const")]
pub fn stub_434d00() -> ! {
    todo!("0x434d00 RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(void)const")
}

// 0x4387d8 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD1Ev")]
pub fn stub_4387d8() -> ! {
    todo!("0x4387d8 __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD1Ev")
}

// 0x44558c — __ZNK3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(void)const")]
pub fn stub_44558c() -> ! {
    todo!("0x44558c RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(void)const")
}

// 0x445700 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv")]
pub fn stub_445700() -> ! {
    todo!("0x445700 __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv")
}

// 0x445768 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv")]
pub fn stub_445768() -> ! {
    todo!("0x445768 __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv")
}

// 0x4457dc — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv")]
pub fn stub_4457dc() -> ! {
    todo!("0x4457dc __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv")
}

// 0x445848 — __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
pub fn stub_445848() -> ! {
    todo!("0x445848 __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")
}

// 0x44588c — __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv")]
pub fn stub_44588c() -> ! {
    todo!("0x44588c __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv")
}

// 0x445890 — __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
pub fn stub_445890() -> ! {
    todo!("0x445890 __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")
}

// 0x445974 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10Soundscape12SoundServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Soundscape::SoundService>(void)")]
pub fn stub_445974() -> ! {
    todo!("0x445974 void RBX::ServiceProvider::callDoGetClassIndex<RBX::Soundscape::SoundService>(void)")
}

// 0x445978 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10Soundscape12SoundServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Soundscape::SoundService>(void)")]
pub fn stub_445978() -> ! {
    todo!("0x445978 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Soundscape::SoundService>(void)")
}

// 0x4528b0 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD2Ev")]
pub fn stub_4528b0() -> ! {
    todo!("0x4528b0 __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD2Ev")
}

// 0x452950 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10Soundscape12SoundServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::SoundService> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundService>(void)")]
pub fn stub_452950() -> ! {
    todo!("0x452950 boost::shared_ptr<RBX::Soundscape::SoundService> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundService>(void)")
}

// 0x452a00 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundServiceEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::SoundService>::shared_ptr<RBX::Soundscape::SoundService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_452a00() -> ! {
    todo!("0x452a00 boost::shared_ptr<RBX::Soundscape::SoundService>::shared_ptr<RBX::Soundscape::SoundService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x452ac8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundService,RBX::Soundscape::SoundService>(boost::shared_ptr<RBX::Soundscape::SoundService> const*,RBX::Soundscape::SoundService *)const")]
pub fn stub_452ac8() -> ! {
    todo!("0x452ac8 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundService,RBX::Soundscape::SoundService>(boost::shared_ptr<RBX::Soundscape::SoundService> const*,RBX::Soundscape::SoundService *)const")
}

// 0x452bb4 — __ZN5boost6detail12shared_countC2IPN3RBX10Soundscape12SoundServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_452bb4() -> ! {
    todo!("0x452bb4 boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x452cbc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_452cbc() -> ! {
    todo!("0x452cbc boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x452cc0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_452cc0() -> ! {
    todo!("0x452cc0 boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x452cc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_452cc4() -> ! {
    todo!("0x452cc4 boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x452ce8 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorC2Ev")]
pub fn stub_452ce8() -> ! {
    todo!("0x452ce8 __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorC2Ev")
}

// 0x452f10 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10Soundscape12SoundServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Soundscape::SoundService>(boost::shared_ptr<RBX::Soundscape::SoundService> const&)")]
pub fn stub_452f10() -> ! {
    todo!("0x452f10 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Soundscape::SoundService>(boost::shared_ptr<RBX::Soundscape::SoundService> const&)")
}

// 0x4ab168 — __ZN3RBX10Reflection4Type12getSingletonINS_9SoundTypeEEERKS1_v
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SoundType>(void)")]
pub fn stub_4ab168() -> ! {
    todo!("0x4ab168 RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SoundType>(void)")
}

// 0x4c2174 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::initSingleton(void)")]
pub fn stub_4c2174() -> ! {
    todo!("0x4c2174 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::initSingleton(void)")
}

// 0x4c2178 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::doGetSingleton(void)")]
pub fn stub_4c2178() -> ! {
    todo!("0x4c2178 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::doGetSingleton(void)")
}

// 0x4c2268 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
pub fn stub_4c2268() -> ! {
    todo!("0x4c2268 RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")
}

// 0x4c226c — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
pub fn stub_4c226c() -> ! {
    todo!("0x4c226c RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")
}

// 0x4c2440 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
pub fn stub_4c2440() -> ! {
    todo!("0x4c2440 RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")
}

// 0x4c24e0 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(char const*)const")]
pub fn stub_4c24e0() -> ! {
    todo!("0x4c24e0 RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(char const*)const")
}

// 0x4c2510 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_4c2510() -> ! {
    todo!("0x4c2510 RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4c2530 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_4c2530() -> ! {
    todo!("0x4c2530 RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4c258c — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(unsigned long,std::string &)const")]
pub fn stub_4c258c() -> ! {
    todo!("0x4c258c RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(unsigned long,std::string &)const")
}

// 0x4c26d0 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringERKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(RBX::SoundType const&)const")]
pub fn stub_4c26d0() -> ! {
    todo!("0x4c26d0 RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(RBX::SoundType const&)const")
}

// 0x4c2870 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9SoundTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SoundType>(RBX::SoundType const&)")]
pub fn stub_4c2870() -> ! {
    todo!("0x4c2870 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SoundType>(RBX::SoundType const&)")
}

// 0x4c28c0 — __ZN3rbx14implementation12typed_holderIN3RBX9SoundTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::SoundType>::singleton(void)")]
pub fn stub_4c28c0() -> ! {
    todo!("0x4c28c0 rbx::implementation::typed_holder<RBX::SoundType>::singleton(void)")
}

// 0x4c292c — __ZN3rbx14implementation12typed_holderIN3RBX9SoundTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::SoundType>::construct_func(char const*,char *)")]
pub fn stub_4c292c() -> ! {
    todo!("0x4c292c rbx::implementation::typed_holder<RBX::SoundType>::construct_func(char const*,char *)")
}

// 0x4c2938 — __ZN3rbx14implementation12typed_holderIN3RBX9SoundTypeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::SoundType>::destruct_func(char *)")]
pub fn stub_4c2938() -> ! {
    todo!("0x4c2938 rbx::implementation::typed_holder<RBX::SoundType>::destruct_func(char *)")
}

// 0x4c293c — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE13convertToItemERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToItem(RBX::SoundType const&)const")]
pub fn stub_4c293c() -> ! {
    todo!("0x4c293c RBX::Reflection::EnumDesc<RBX::SoundType>::convertToItem(RBX::SoundType const&)const")
}

// 0x4c2a08 — __ZN3rbx8any_castIRKN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::SoundType const& rbx::any_cast<RBX::SoundType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_4c2a08() -> ! {
    todo!("0x4c2a08 RBX::SoundType const& rbx::any_cast<RBX::SoundType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4c2af8 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueERKNS_4NameERS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(RBX::Name const&,RBX::SoundType&)const")]
pub fn stub_4c2af8() -> ! {
    todo!("0x4c2af8 RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(RBX::Name const&,RBX::SoundType&)const")
}

// 0x4c2b74 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SoundType>> *)")]
pub fn stub_4c2b74() -> ! {
    todo!("0x4c2b74 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SoundType>> *)")
}

// 0x710c78 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_710c78() -> ! {
    todo!("0x710c78 __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x7aac50 — __ZN3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(RBX::Instance const*)")]
pub fn stub_7aac50() -> ! {
    todo!("0x7aac50 RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(RBX::Instance const*)")
}

// 0x7f937c — __ZN3RBX10Soundscape14CollisionSoundC2ENS0_7SoundIdEPNS0_21CollisionSoundManagerEPKNS_8InstanceE
#[doc(alias = "RBX::Soundscape::CollisionSound::CollisionSound(RBX::Soundscape::SoundId,RBX::Soundscape::CollisionSoundManager *,RBX::Instance const*)")]
pub fn stub_7f937c() -> ! {
    todo!("0x7f937c RBX::Soundscape::CollisionSound::CollisionSound(RBX::Soundscape::SoundId,RBX::Soundscape::CollisionSoundManager *,RBX::Instance const*)")
}

// 0x7f9bb8 — __ZN3RBX10Soundscape21CollisionSoundManager9PlaySoundEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Soundscape::CollisionSoundManager *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::Soundscape::CollisionSoundManager::PlaySound(RBX::Primitive *)")]
pub fn stub_7f9bb8() -> ! {
    todo!("0x7f9bb8 RBX::Soundscape::CollisionSoundManager::PlaySound(RBX::Primitive *)")
}

// 0x7f9ce8 — __ZN3RBX10Soundscape21CollisionSoundManagerD1Ev
// type: void __fastcall(RBX::Soundscape::CollisionSoundManager *__hidden this)
#[doc(alias = "RBX::Soundscape::CollisionSoundManager::~CollisionSoundManager()")]
pub fn stub_7f9ce8() -> ! {
    todo!("0x7f9ce8 RBX::Soundscape::CollisionSoundManager::~CollisionSoundManager()")
}

// 0x7f9cec — __ZN3RBX10Soundscape21CollisionSoundManagerD2Ev
// type: void __fastcall(RBX::Soundscape::CollisionSoundManager *__hidden this)
#[doc(alias = "RBX::Soundscape::CollisionSoundManager::~CollisionSoundManager()")]
pub fn stub_7f9cec() -> ! {
    todo!("0x7f9cec RBX::Soundscape::CollisionSoundManager::~CollisionSoundManager()")
}

// 0x7f9e70 — __ZN3RBX10Soundscape21CollisionSoundManager10LoadSoundsEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Soundscape::CollisionSoundManager *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::CollisionSoundManager::LoadSounds(RBX::Instance const*)")]
pub fn stub_7f9e70() -> ! {
    todo!("0x7f9e70 RBX::Soundscape::CollisionSoundManager::LoadSounds(RBX::Instance const*)")
}

// 0x7fca48 — __ZN3RBX10Soundscape21CollisionSoundManager9LoadSoundENS0_18CollisionSoundTypeESsPKNS_8InstanceE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Soundscape::CollisionSoundManager::LoadSound(RBX::Soundscape::CollisionSoundType,std::string,RBX::Instance const*)")]
pub fn stub_7fca48() -> ! {
    todo!("0x7fca48 RBX::Soundscape::CollisionSoundManager::LoadSound(RBX::Soundscape::CollisionSoundType,std::string,RBX::Instance const*)")
}

// 0x7fcd04 — __ZN3RBX10Soundscape21CollisionSoundManager9PlaySoundESt4pairIPNS_9PrimitiveES4_E
#[doc(alias = "RBX::Soundscape::CollisionSoundManager::PlaySound(std::pair<RBX::Primitive *,RBX::Primitive *>)")]
pub fn stub_7fcd04() -> ! {
    todo!("0x7fcd04 RBX::Soundscape::CollisionSoundManager::PlaySound(std::pair<RBX::Primitive *,RBX::Primitive *>)")
}

// 0x7fcf00 — __ZN3RBX10Soundscape21CollisionSoundManager12IsPartPlayerEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::Soundscape::CollisionSoundManager *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::Soundscape::CollisionSoundManager::IsPartPlayer(RBX::PartInstance *)")]
pub fn stub_7fcf00() -> ! {
    todo!("0x7fcf00 RBX::Soundscape::CollisionSoundManager::IsPartPlayer(RBX::PartInstance *)")
}

// 0x7fcf70 — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEE5resetIS3_EEvPT_
#[doc(alias = "void boost::shared_ptr<RBX::Soundscape::Sound>::reset<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
pub fn stub_7fcf70() -> ! {
    todo!("0x7fcf70 void boost::shared_ptr<RBX::Soundscape::Sound>::reset<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")
}

// 0x7fcf9c — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>> const&)")]
pub fn stub_7fcf9c() -> ! {
    todo!("0x7fcf9c rbx::signals::connection rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>> const&)")
}

// 0x7fd010 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>> const&)")]
pub fn stub_7fd010() -> ! {
    todo!("0x7fd010 rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>> const&)")
}

// 0x7fd084 — __ZNSt3mapIN3RBX10Soundscape18CollisionSoundTypeEN5boost10shared_ptrINS1_14CollisionSoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(int, int *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::Soundscape::CollisionSoundType,boost::shared_ptr<RBX::Soundscape::CollisionSound>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::operator[](RBX::Soundscape::CollisionSoundType const&)")]
pub fn stub_7fd084() -> ! {
    todo!("0x7fd084 std::map<RBX::Soundscape::CollisionSoundType,boost::shared_ptr<RBX::Soundscape::CollisionSound>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::operator[](RBX::Soundscape::CollisionSoundType const&)")
}

// 0x7fd1cc — __ZN5boost10shared_ptrIN3RBX10Soundscape14CollisionSoundEEaSERKS4_
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::CollisionSound>::operator=(boost::shared_ptr<RBX::Soundscape::CollisionSound> const&)")]
pub fn stub_7fd1cc() -> ! {
    todo!("0x7fd1cc boost::shared_ptr<RBX::Soundscape::CollisionSound>::operator=(boost::shared_ptr<RBX::Soundscape::CollisionSound> const&)")
}

// 0x7fd204 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_7fd204() -> ! {
    todo!("0x7fd204 std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>> const&)")
}

// 0x7fd2b8 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_7fd2b8() -> ! {
    todo!("0x7fd2b8 std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>> const&)")
}

// 0x7fd304 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_7fd304() -> ! {
    todo!("0x7fd304 std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>> const&)")
}

// 0x7fd36c — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_create_node(std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_7fd36c() -> ! {
    todo!("0x7fd36c std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_create_node(std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>> const&)")
}

// 0x7fd45c — __ZN5boost10shared_ptrIN3RBX10Soundscape14CollisionSoundEEC2IS3_EEPT_
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::CollisionSound>::shared_ptr<RBX::Soundscape::CollisionSound>(RBX::Soundscape::CollisionSound *)")]
pub fn stub_7fd45c() -> ! {
    todo!("0x7fd45c boost::shared_ptr<RBX::Soundscape::CollisionSound>::shared_ptr<RBX::Soundscape::CollisionSound>(RBX::Soundscape::CollisionSound *)")
}

// 0x7fd530 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape14CollisionSoundEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::CollisionSound>(RBX::Soundscape::CollisionSound *)")]
pub fn stub_7fd530() -> ! {
    todo!("0x7fd530 boost::detail::shared_count::shared_count<RBX::Soundscape::CollisionSound>(RBX::Soundscape::CollisionSound *)")
}

// 0x7fd640 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::~sp_counted_impl_p()")]
pub fn stub_7fd640() -> ! {
    todo!("0x7fd640 boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::~sp_counted_impl_p()")
}

// 0x7fd644 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::~sp_counted_impl_p()")]
pub fn stub_7fd644() -> ! {
    todo!("0x7fd644 boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::~sp_counted_impl_p()")
}

// 0x7fd648 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::dispose(void)")]
pub fn stub_7fd648() -> ! {
    todo!("0x7fd648 boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::dispose(void)")
}

// 0x7fd6f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::get_deleter(std::type_info const&)")]
pub fn stub_7fd6f0() -> ! {
    todo!("0x7fd6f0 boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::get_deleter(std::type_info const&)")
}

// 0x7fd6f4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::get_untyped_deleter(void)")]
pub fn stub_7fd6f4() -> ! {
    todo!("0x7fd6f4 boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::get_untyped_deleter(void)")
}

// 0x7fd6f8 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7fd6f8() -> ! {
    todo!("0x7fd6f8 rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x7fd724 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7fd724() -> ! {
    todo!("0x7fd724 rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x7fd7f8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")]
pub fn stub_7fd7f8() -> ! {
    todo!("0x7fd7f8 rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")
}

// 0x7fd80c — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
pub fn stub_7fd80c() -> ! {
    todo!("0x7fd80c __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")
}

// 0x7fd820 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape21CollisionSoundManagerEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")]
pub fn stub_7fd820() -> ! {
    todo!("0x7fd820 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")
}

// 0x7fd838 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
pub fn stub_7fd838() -> ! {
    todo!("0x7fd838 rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")
}

// 0x7fd864 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
pub fn stub_7fd864() -> ! {
    todo!("0x7fd864 rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")
}

// 0x7fdb68 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7fdb68() -> ! {
    todo!("0x7fdb68 rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x7fdb94 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7fdb94() -> ! {
    todo!("0x7fdb94 rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x7fdd84 — __ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::call(std::pair<RBX::Primitive *,RBX::Primitive *>)")]
pub fn stub_7fdd84() -> ! {
    todo!("0x7fdd84 rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::call(std::pair<RBX::Primitive *,RBX::Primitive *>)")
}

// 0x7fdda8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_7fdda8() -> ! {
    todo!("0x7fdda8 __ZThn4_N3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")
}

// 0x7fddcc — __ZN5boost3_bi5list2INS0_5valueIPN3RBX10Soundscape21CollisionSoundManagerEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_St4pairIPNS3_9PrimitiveESG_EEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list1<std::pair<RBX::Primitive *,RBX::Primitive *>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>> &,boost::_bi::list1<std::pair<RBX::Primitive *,RBX::Primitive *>&> &,int)")]
pub fn stub_7fddcc() -> ! {
    todo!("0x7fddcc void boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list1<std::pair<RBX::Primitive *,RBX::Primitive *>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>> &,boost::_bi::list1<std::pair<RBX::Primitive *,RBX::Primitive *>&> &,int)")
}

// 0x7fe0e4 — __ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::~callable()")]
pub fn stub_7fe0e4() -> ! {
    todo!("0x7fe0e4 rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::~callable()")
}

// 0x7fe110 — __ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::~callable()")]
pub fn stub_7fe110() -> ! {
    todo!("0x7fe110 rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::~callable()")
}

// 0x7fe1e4 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>> *)")]
pub fn stub_7fe1e4() -> ! {
    todo!("0x7fe1e4 std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>> *)")
}

// 0x7fe20c — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>> *)")]
pub fn stub_7fe20c() -> ! {
    todo!("0x7fe20c std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Soundscape::CollisionSoundType const,boost::shared_ptr<RBX::Soundscape::CollisionSound>>> *)")
}

// 0xb29978 — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator6createEv
// type: void __fastcall(RBX::Soundscape::SoundService **, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, pthread_mutex_t *, int, int, void *, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator6createEv")]
pub fn stub_b29978() -> ! {
    todo!("0xb29978 __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator6createEv")
}

// 0xb29db0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_b29db0() -> ! {
    todo!("0xb29db0 boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xb29dc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_b29dc8() -> ! {
    todo!("0xb29dc8 boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xf30314 — j___ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE
// type: _DWORD __fastcall(SoundServiceStatsItem *__hidden this, const RBX::Soundscape::SoundService *)
#[doc(alias = "SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")]
pub fn stub_f30314() -> ! {
    todo!("0xf30314 SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")
}

// 0xf30324 — j___ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE
// type: SoundServiceStatsItem *__fastcall(SoundServiceStatsItem *__hidden this, const RBX::Soundscape::SoundService *)
#[doc(alias = "SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")]
pub fn stub_f30324() -> ! {
    todo!("0xf30324 SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")
}

// 0xf30334 — j___ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::BoundFuncDesc(void (RBX::Soundscape::SoundChannel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f30334() -> ! {
    todo!("0xf30334 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::BoundFuncDesc(void (RBX::Soundscape::SoundChannel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf30344 — j___ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_f30344() -> ! {
    todo!("0xf30344 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf30354 — j___ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EEC2EMS3_FvS4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::BoundFuncDesc(void (RBX::Soundscape::SoundService::*)(RBX::SoundType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f30354() -> ! {
    todo!("0xf30354 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::BoundFuncDesc(void (RBX::Soundscape::SoundService::*)(RBX::SoundType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf30364 — j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEEC2IMS3_KFS4_vEMS3_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::PropDescriptor<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>(char const*,char const*,RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f30364() -> ! {
    todo!("0xf30364 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::PropDescriptor<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>(char const*,char const*,RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf30374 — j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEMS3_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f30374() -> ! {
    todo!("0xf30374 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf30384 — j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,int>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f30384() -> ! {
    todo!("0xf30384 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,int>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf30394 — j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfEC2IMS3_KFfvEMS3_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::PropDescriptor<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>(char const*,char const*,float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f30394() -> ! {
    todo!("0xf30394 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::PropDescriptor<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>(char const*,char const*,float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf303a4 — j___ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiEC2IMS3_KFivEMS3_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::PropDescriptor<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>(char const*,char const*,int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f303a4() -> ! {
    todo!("0xf303a4 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::PropDescriptor<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>(char const*,char const*,int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf303b4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEEC2IMS3_KFS4_vEMS3_FvRKS4_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::EnumPropDescriptor<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>(char const*,char const*,RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f303b4() -> ! {
    todo!("0xf303b4 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::EnumPropDescriptor<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>(char const*,char const*,RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf303c4 — j___ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f303c4() -> ! {
    todo!("0xf303c4 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf303d4 — j___ZN3RBX10Reflection7Variant14genericConvertINS_10Soundscape7SoundIdEEERT_v
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")]
pub fn stub_f303d4() -> ! {
    todo!("0xf303d4 RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")
}

// 0xf303e4 — j___ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE7addPairES3_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbType,char const*)")]
pub fn stub_f303e4() -> ! {
    todo!("0xf303e4 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbType,char const*)")
}

// 0xf303f4 — j___ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
pub fn stub_f303f4() -> ! {
    todo!("0xf303f4 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")
}

// 0xf30404 — j___ZN3RBX10Reflection9ArgHelper6getArgINS_9SoundTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::SoundType RBX::Reflection::ArgHelper::getArg<RBX::SoundType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::SoundType> const&,boost::disable_if<boost::is_same<RBX::SoundType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_f30404() -> ! {
    todo!("0xf30404 RBX::SoundType RBX::Reflection::ArgHelper::getArg<RBX::SoundType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::SoundType> const&,boost::disable_if<boost::is_same<RBX::SoundType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf30414 — j___ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9SoundTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINS9_7is_enumIS7_EEvE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::SoundType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::SoundType &,boost::enable_if<boost::is_enum<RBX::SoundType>,void>::type *)")]
pub fn stub_f30414() -> ! {
    todo!("0xf30414 bool RBX::Reflection::ArgHelper::try_enum<1,RBX::SoundType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::SoundType &,boost::enable_if<boost::is_enum<RBX::SoundType>,void>::type *)")
}

// 0xf30424 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundChannelEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundChannel>(char const*,char const*,bool RBX::Soundscape::SoundChannel::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f30424() -> ! {
    todo!("0xf30424 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundChannel>(char const*,char const*,bool RBX::Soundscape::SoundChannel::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf30434 — j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundServiceEEEPKcS8_MT_fMS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundService>(char const*,char const*,float RBX::Soundscape::SoundService::*,void (RBX::Soundscape::SoundService::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f30434() -> ! {
    todo!("0xf30434 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundService>(char const*,char const*,float RBX::Soundscape::SoundService::*,void (RBX::Soundscape::SoundService::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf30444 — j___ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_f30444() -> ! {
    todo!("0xf30444 j___ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0xf30454 — j___ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_f30454() -> ! {
    todo!("0xf30454 j___ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0xf30464 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Soundscape10ReverbTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType> const>::doGetSingleton(void)")]
pub fn stub_f30464() -> ! {
    todo!("0xf30464 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType> const>::doGetSingleton(void)")
}

// 0xf30474 — j___ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_
// type: _DWORD __fastcall(RBX::Soundscape::SoundService::SoundJob *__hidden this, RBX::Soundscape::SoundService *)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")]
pub fn stub_f30474() -> ! {
    todo!("0xf30474 RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")
}

// 0xf30484 — j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv")]
pub fn stub_f30484() -> ! {
    todo!("0xf30484 j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv")
}

// 0xf30494 — j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev")]
pub fn stub_f30494() -> ! {
    todo!("0xf30494 j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev")
}

// 0xf304a4 — j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev")]
pub fn stub_f304a4() -> ! {
    todo!("0xf304a4 j___ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev")
}
