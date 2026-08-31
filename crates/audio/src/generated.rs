//! audio generated — 150 stubs sorted by EA, from ida/export.json
//! Filter: FMOD/Sound/Audio (2541 total, 2541 distinct EA) — 2011 distinct EA (2157 stubs) prior in lib.rs + 150 this batch = 2161 distinct (2307 stubs) total, 380 distinct remaining
//! Batch: 0x376198..0x37b1c4 | SharedPtr = rbx_core::SharedPtr (Arc) not boost::shared_ptr | strip '

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
#[doc(alias = "`non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
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

