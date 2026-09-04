//! audio generated — 980 stubs sorted by EA, from ida/export.json
//! Filter: FMOD|Sound|Audio case-sensitive (2541 distinct) — 2161 distinct EA (2307 stubs) in lib.rs + 980 in generated.rs (530 +300 watchdog +150 next) = 2541 distinct (3287 stubs) total, 0 distinct remaining (1561 remaining vs generated.rs only)
//! Batch: 0x376198..0xf54904 + watchdog 0x686a4..0x72a28 (150) + 0x72a40..0x7df78 (150 watchdog) + 0x7e58c..0x91454 (150 next, EA-sorted, SharedPtr = rbx_core::SharedPtr not boost::shared_ptr)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

use crate::{
    AsyncOs, AsyncRegistry, AsyncSoundView, AsyncThread, ClientNet, DspSnapshot, Floor1Info,
    Floor1Look, MemTracker, OggpackBuffer, Profile, ProfileClient, ProfileCpu, ProfileDsp,
    ProfileLive, ProfileMemExtra, ProfileModule, ProfileOs, VorbisBlock, VorbisCodebook,
    VorbisCodecOs, VorbisDspState, VorbisHeap, VorbisStaticBook,
};

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
#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")]
pub fn stub_37677c() {
    // IDA 0x37677c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x376a24 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>& rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const&)")]
pub fn stub_376a24() {
    // IDA 0x376a24: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x376a58 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::operator=(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const&)")]
pub fn stub_376a58() {
    // IDA 0x376a58: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x376a90 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI21SoundServiceStatsItemEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const&)")]
pub fn stub_376a90() {
    // IDA 0x376a90: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_376fc0() {
    // IDA 0x376fc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x376fe4 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")]
pub fn stub_376fe4() {
    // IDA 0x376fe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x377024 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::~PropDescriptor()")]
pub fn stub_377024() {
    // IDA 0x377024: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x377048 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::~PropDescriptor()")]
pub fn stub_377048() {
    // IDA 0x377048: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_377074() {
    // IDA 0x377074: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x377098 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::~PropDescriptor()")]
pub fn stub_377098() {
    // IDA 0x377098: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3770bc — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_3770bc() {
    // IDA 0x3770bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3770e0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>> const&)")]
pub fn stub_3770e0() {
    // IDA 0x3770e0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0x377154 — __ZN3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")]
pub fn stub_377154() -> ! {
    todo!("0x377154 RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")
}

// 0x37716c — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEaSERKS4_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::Sound>::operator=(rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")]
pub fn stub_37716c() {
    // IDA 0x37716c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3771a4 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_3771a4() -> ! {
    todo!("0x3771a4 __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x3772c0 — __ZNSt3mapIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(int, const std::string *)
#[doc(alias = "std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::operator[](RBX::Soundscape::SoundId const&)")]
pub fn stub_3772c0() {
    // IDA 0x3772c0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_37751c() {
    // IDA 0x37751c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37753c() {
    // IDA 0x37753c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37754c — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_37754c() {
    // IDA 0x37754c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x377550 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev")]
pub fn stub_377550() {
    // IDA 0x377550: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x377554 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
pub fn stub_377554() {
    // IDA 0x377554: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x377558 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
pub fn stub_377558() {
    // IDA 0x377558: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_377c8c() {
    // IDA 0x377c8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x377e60 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev")]
pub fn stub_377e60() {
    // IDA 0x377e60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundChannel>(void)")]
pub fn stub_3780c8() {
    // IDA 0x3780c8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x378178 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::shared_ptr<RBX::Soundscape::SoundChannel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_378178() {
    // IDA 0x378178: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x378240 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundChannelES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundChannel>(rbx_core::SharedPtr<RBX::Soundscape::SoundChannel> const*,RBX::Soundscape::SoundChannel *)const")]
pub fn stub_378240() {
    // IDA 0x378240: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x37832c — __ZN5boost6detail12shared_countC2IPN3RBX10Soundscape12SoundChannelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37832c() {
    // IDA 0x37832c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x378434 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_378434() {
    // IDA 0x378434: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x378438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_378438() {
    // IDA 0x378438: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37843c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_37843c() {
    // IDA 0x37843c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37845c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_37845c() {
    // IDA 0x37845c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x378474 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_378474() {
    // IDA 0x378474: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
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
#[doc(alias = "std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>::pair(RBX::Soundscape::SoundId const&,rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")]
pub fn stub_378814() {
    // IDA 0x378814: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3788dc — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_3788dc() {
    // IDA 0x3788dc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3789c4 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_3789c4() {
    // IDA 0x3789c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x378a14 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_378a14() {
    // IDA 0x378a14: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x378a94 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_create_node(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_378a94() {
    // IDA 0x378a94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x378ba0 — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::Sound>::shared_ptr<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
pub fn stub_378ba0() {
    // IDA 0x378ba0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x378c74 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape5SoundEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
pub fn stub_378c74() {
    // IDA 0x378c74: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x378d80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::~sp_counted_impl_p()")]
pub fn stub_378d80() {
    // IDA 0x378d80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x378d84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::~sp_counted_impl_p()")]
pub fn stub_378d84() {
    // IDA 0x378d84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x378d88 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::dispose(void)")]
pub fn stub_378d88() {
    // IDA 0x378d88: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x378e2c — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::get_deleter(std::type_info const&)")]
pub fn stub_378e2c() {
    // IDA 0x378e2c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x378e30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::get_untyped_deleter(void)")]
pub fn stub_378e30() {
    // IDA 0x378e30: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x378e34 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE4findERS4_
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::find(RBX::Soundscape::SoundId const&)")]
pub fn stub_378e34() {
    // IDA 0x378e34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_379094() {
    // IDA 0x379094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3790c0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_3790c0() {
    // IDA 0x3790c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x379194 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub fn stub_379194() {
    // IDA 0x379194: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0x37919c — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub fn stub_37919c() {
    // IDA 0x37919c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0x3791a4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_
// type: int __fastcall(int)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
pub fn stub_3791a4() {
    // IDA 0x3791a4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

// 0x3791bc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub fn stub_3791bc() {
    // IDA 0x3791bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3791e8 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub fn stub_3791e8() {
    // IDA 0x3791e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3792bc — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>)")]
pub fn stub_3792bc() {
    // IDA 0x3792bc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3792e4 — __ZN9__gnu_cxx13new_allocatorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEE7destroyEPSA_
// type: void __fastcall(int, std::string *)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>::destroy(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>*)")]
pub fn stub_3792e4() {
    // IDA 0x3792e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x379388 — __ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_379388() {
    // IDA 0x379388: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37938c — __ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37938c() {
    // IDA 0x37938c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37942c — __ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37942c() {
    // IDA 0x37942c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x379434 — __ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_379434() {
    // IDA 0x379434: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3794d8 — __ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3794d8() {
    // IDA 0x3794d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3794e0 — __ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3794e0() {
    // IDA 0x3794e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x379584 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_379584() {
    // IDA 0x379584: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x379588 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_379588() {
    // IDA 0x379588: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x379628 — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_379628() {
    // IDA 0x379628: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x379630 — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_379630() {
    // IDA 0x379630: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3796d4 — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_3796d4() {
    // IDA 0x3796d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3796dc — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_3796dc() {
    // IDA 0x3796dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_379884() {
    // IDA 0x379884: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_379c58() {
    // IDA 0x379c58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37a048() {
    // IDA 0x37a048: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37a1d4() {
    // IDA 0x37a1d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37a484() {
    // IDA 0x37a484: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37abbc() {
    // IDA 0x37abbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37abe0 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::~TypedPropertyDescriptor()")]
pub fn stub_37abe0() {
    // IDA 0x37abe0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37af2c() {
    // IDA 0x37af2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37b000 — __ZNK3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_37b000() -> ! {
    todo!("0x37b000 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x37b034 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9SoundTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int (__fastcall ***__fastcall(int (__fastcall ***)(_DWORD), int))(_DWORD)
#[doc(alias = "RBX::SoundType RBX::Reflection::ArgHelper::getArg<RBX::SoundType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::SoundType> const&,boost::disable_if<boost::is_same<RBX::SoundType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_37b034() {
    // IDA 0x37b034: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_37b3cc() {
    // IDA 0x37b3cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::shared_ptr<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
pub fn stub_37be68() {
    // IDA 0x37be68: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37bf50 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10Soundscape12SoundService8SoundJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Soundscape::SoundService::SoundJob,RBX::Soundscape::SoundService::SoundJob>(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const*,RBX::Soundscape::SoundService::SoundJob *)const")]
pub fn stub_37bf50() {
    // IDA 0x37bf50: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x37c034 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape12SoundService8SoundJobEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
pub fn stub_37c034() {
    // IDA 0x37c034: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37c12c — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")]
pub fn stub_37c12c() {
    // IDA 0x37c12c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37c130 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")]
pub fn stub_37c130() {
    // IDA 0x37c130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37c134 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::dispose(void)")]
pub fn stub_37c134() {
    // IDA 0x37c134: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37c144 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_deleter(std::type_info const&)")]
pub fn stub_37c144() {
    // IDA 0x37c144: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37c148 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_untyped_deleter(void)")]
pub fn stub_37c148() {
    // IDA 0x37c148: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37c14c — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_37c14c() {
    // IDA 0x37c14c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37c200 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_37c200() {
    // IDA 0x37c200: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37c24c — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_37c24c() {
    // IDA 0x37c24c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37c2b4 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE14_M_create_nodeERKS9_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_create_node(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_37c2b4() {
    // IDA 0x37c2b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37c3a4 — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_37c3a4() {
    // IDA 0x37c3a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37c934() {
    // IDA 0x37c934: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37c938 — __ZN3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "RBX::StockSound::~StockSound()")]
pub fn stub_37c938() {
    // IDA 0x37c938: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37c9e8() {
    // IDA 0x37c9e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37c9f0 — __ZThn32_N3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
pub fn stub_37c9f0() {
    // IDA 0x37c9f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37ca94 — __ZThn32_NK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv")]
pub fn stub_37ca94() {
    // IDA 0x37ca94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37caa4 — __ZThn36_N3RBX10StockSoundD1Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
pub fn stub_37caa4() {
    // IDA 0x37caa4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37caac — __ZThn36_N3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
pub fn stub_37caac() {
    // IDA 0x37caac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37cbc4() {
    // IDA 0x37cbc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37cbc8 — __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37cbc8() {
    // IDA 0x37cbc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37cc68 — __ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37cc68() {
    // IDA 0x37cc68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37cc70 — __ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37cc70() {
    // IDA 0x37cc70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37cd14 — __ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37cd14() {
    // IDA 0x37cd14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37cd1c — __ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37cd1c() {
    // IDA 0x37cd1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37cdc0 — __ZN5boost10shared_ptrIN3RBX10StockSoundEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound>::shared_ptr<RBX::StockSound,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37cdc0() {
    // IDA 0x37cdc0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37ce88 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10StockSoundES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StockSound,RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const*,RBX::StockSound *)const")]
pub fn stub_37ce88() {
    // IDA 0x37ce88: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x37cf74 — __ZN5boost6detail12shared_countC2IPN3RBX10StockSoundENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37cf74() {
    // IDA 0x37cf74: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37d07c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37d07c() {
    // IDA 0x37d07c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37d080 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37d080() {
    // IDA 0x37d080: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37d084 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_37d084() {
    // IDA 0x37d084: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37d0a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_37d0a4() {
    // IDA 0x37d0a4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37d0bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_37d0bc() {
    // IDA 0x37d0bc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37d0c0 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>> *)")]
pub fn stub_37d0c0() {
    // IDA 0x37d0c0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37d0f0 — __ZNSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2IKS2_S6_EERKS_IT_T0_E
// type: _DWORD *__fastcall(_DWORD *, const shared_count *)
#[doc(alias = "std::pair<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>(std::pair const&<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)")]
pub fn stub_37d0f0() {
    // IDA 0x37d0f0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37d1b4 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
pub fn stub_37d1b4() {
    // IDA 0x37d1b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37d1dc — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
pub fn stub_37d1dc() {
    // IDA 0x37d1dc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37d1f8 — __ZN5boost10scoped_ptrIN3RBX10Soundscape21CollisionSoundManagerEED2Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(RBX::Soundscape::CollisionSoundManager **)
#[doc(alias = "boost::scoped_ptr<RBX::Soundscape::CollisionSoundManager>::~scoped_ptr()")]
pub fn stub_37d1f8() {
    // IDA 0x37d1f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37d2a0 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37d2a0() {
    // IDA 0x37d2a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37d2a4 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37d2a4() {
    // IDA 0x37d2a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37d344 — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37d344() {
    // IDA 0x37d344: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37d34c — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37d34c() {
    // IDA 0x37d34c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37d3f0 — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_37d3f0() {
    // IDA 0x37d3f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37d3f8 — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_37d3f8() {
    // IDA 0x37d3f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
#[doc(alias = "rbx_core::SharedPtr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)")]
pub fn stub_37d98c() {
    // IDA 0x37d98c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_37e05c() {
    // IDA 0x37e05c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37e098 — __ZN21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
pub fn stub_37e098() {
    // IDA 0x37e098: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37e344() {
    // IDA 0x37e344: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37e384 — __ZThn32_N21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "__ZThn32_N21SoundServiceStatsItemD0Ev")]
pub fn stub_37e384() {
    // IDA 0x37e384: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37e458 — __ZThn36_N21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "__ZThn36_N21SoundServiceStatsItemD1Ev")]
pub fn stub_37e458() {
    // IDA 0x37e458: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37e498 — __ZThn36_N21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
#[doc(alias = "__ZThn36_N21SoundServiceStatsItemD0Ev")]
pub fn stub_37e498() {
    // IDA 0x37e498: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37e56c — __ZN5boost10shared_ptrI21SoundServiceStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37e56c() {
    // IDA 0x37e56c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x37e634 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI21SoundServiceStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<SoundServiceStatsItem,SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const*,SoundServiceStatsItem *)const")]
pub fn stub_37e634() {
    // IDA 0x37e634: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x37e720 — __ZN5boost6detail12shared_countC2IP21SoundServiceStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37e720() {
    // IDA 0x37e720: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37e828 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37e828() {
    // IDA 0x37e828: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37e82c — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37e82c() {
    // IDA 0x37e82c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37e830 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_37e830() {
    // IDA 0x37e830: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37e850 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_37e850() {
    // IDA 0x37e850: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x37e868 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_37e868() {
    // IDA 0x37e868: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
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
pub fn stub_37e9c4() {
    // IDA 0x37e9c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x37e9c8 — __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService::SoundJob *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
pub fn stub_37e9c8() {
    // IDA 0x37e9c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_37eab0() {
    // IDA 0x37eab0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_4387d8() {
    // IDA 0x4387d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_4528b0() {
    // IDA 0x4528b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x452950 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10Soundscape12SoundServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundService>(void)")]
pub fn stub_452950() {
    // IDA 0x452950: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x452a00 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundServiceEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService>::shared_ptr<RBX::Soundscape::SoundService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_452a00() {
    // IDA 0x452a00: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x452ac8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundService,RBX::Soundscape::SoundService>(rbx_core::SharedPtr<RBX::Soundscape::SoundService> const*,RBX::Soundscape::SoundService *)const")]
pub fn stub_452ac8() {
    // IDA 0x452ac8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x452bb4 — __ZN5boost6detail12shared_countC2IPN3RBX10Soundscape12SoundServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_452bb4() {
    // IDA 0x452bb4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x452cbc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_452cbc() {
    // IDA 0x452cbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x452cc0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_452cc0() {
    // IDA 0x452cc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x452cc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_452cc4() {
    // IDA 0x452cc4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x452ce8 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorC2Ev")]
pub fn stub_452ce8() -> ! {
    todo!("0x452ce8 __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorC2Ev")
}

// 0x452f10 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10Soundscape12SoundServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Soundscape::SoundService>(rbx_core::SharedPtr<RBX::Soundscape::SoundService> const&)")]
pub fn stub_452f10() {
    // IDA 0x452f10: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_4c2268() {
    // IDA 0x4c2268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4c226c — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
pub fn stub_4c226c() {
    // IDA 0x4c226c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4c2440 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
pub fn stub_4c2440() {
    // IDA 0x4c2440: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
pub fn stub_4c2b74() {
    // IDA 0x4c2b74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_7f9ce8() {
    // IDA 0x7f9ce8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7f9cec — __ZN3RBX10Soundscape21CollisionSoundManagerD2Ev
// type: void __fastcall(RBX::Soundscape::CollisionSoundManager *__hidden this)
#[doc(alias = "RBX::Soundscape::CollisionSoundManager::~CollisionSoundManager()")]
pub fn stub_7f9cec() {
    // IDA 0x7f9cec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
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
#[doc(alias = "void rbx_core::SharedPtr<RBX::Soundscape::Sound>::reset<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
pub fn stub_7fcf70() {
    // IDA 0x7fcf70: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x7fcf9c — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>> const&)")]
pub fn stub_7fcf9c() {
    // IDA 0x7fcf9c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0x7fd010 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>> const&)")]
pub fn stub_7fd010() {
    // IDA 0x7fd010: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0x7fd084 — __ZNSt3mapIN3RBX10Soundscape18CollisionSoundTypeEN5boost10shared_ptrINS1_14CollisionSoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(int, int *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::Soundscape::CollisionSoundType,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::operator[](RBX::Soundscape::CollisionSoundType const&)")]
pub fn stub_7fd084() {
    // IDA 0x7fd084: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x7fd1cc — __ZN5boost10shared_ptrIN3RBX10Soundscape14CollisionSoundEEaSERKS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>::operator=(rbx_core::SharedPtr<RBX::Soundscape::CollisionSound> const&)")]
pub fn stub_7fd1cc() {
    // IDA 0x7fd1cc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x7fd204 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_7fd204() {
    // IDA 0x7fd204: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x7fd2b8 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_7fd2b8() {
    // IDA 0x7fd2b8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x7fd304 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_7fd304() {
    // IDA 0x7fd304: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x7fd36c — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_create_node(std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_7fd36c() {
    // IDA 0x7fd36c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x7fd45c — __ZN5boost10shared_ptrIN3RBX10Soundscape14CollisionSoundEEC2IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>::shared_ptr<RBX::Soundscape::CollisionSound>(RBX::Soundscape::CollisionSound *)")]
pub fn stub_7fd45c() {
    // IDA 0x7fd45c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x7fd530 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape14CollisionSoundEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::CollisionSound>(RBX::Soundscape::CollisionSound *)")]
pub fn stub_7fd530() {
    // IDA 0x7fd530: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x7fd640 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::~sp_counted_impl_p()")]
pub fn stub_7fd640() {
    // IDA 0x7fd640: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fd644 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::~sp_counted_impl_p()")]
pub fn stub_7fd644() {
    // IDA 0x7fd644: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fd648 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::dispose(void)")]
pub fn stub_7fd648() {
    // IDA 0x7fd648: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x7fd6f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::get_deleter(std::type_info const&)")]
pub fn stub_7fd6f0() {
    // IDA 0x7fd6f0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x7fd6f4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape14CollisionSoundEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::CollisionSound>::get_untyped_deleter(void)")]
pub fn stub_7fd6f4() {
    // IDA 0x7fd6f4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x7fd6f8 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7fd6f8() {
    // IDA 0x7fd6f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fd724 — __ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7fd724() {
    // IDA 0x7fd724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fd7f8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::call(RBX::Primitive *)")]
pub fn stub_7fd7f8() {
    // IDA 0x7fd7f8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0x7fd80c — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
pub fn stub_7fd80c() {
    // IDA 0x7fd80c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fd820 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape21CollisionSoundManagerEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")]
pub fn stub_7fd820() {
    // IDA 0x7fd820: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

// 0x7fd838 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
pub fn stub_7fd838() {
    // IDA 0x7fd838: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fd864 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9PrimitiveEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Primitive *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(RBX::Primitive *)>::~callable()")]
pub fn stub_7fd864() {
    // IDA 0x7fd864: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fdb68 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7fdb68() {
    // IDA 0x7fdb68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fdb94 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7fdb94() {
    // IDA 0x7fdb94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fdd84 — __ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::call(std::pair<RBX::Primitive *,RBX::Primitive *>)")]
pub fn stub_7fdd84() {
    // IDA 0x7fdd84: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0x7fdda8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_7fdda8() {
    // IDA 0x7fdda8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fddcc — __ZN5boost3_bi5list2INS0_5valueIPN3RBX10Soundscape21CollisionSoundManagerEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_St4pairIPNS3_9PrimitiveESG_EEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list1<std::pair<RBX::Primitive *,RBX::Primitive *>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>> &,boost::_bi::list1<std::pair<RBX::Primitive *,RBX::Primitive *>&> &,int)")]
pub fn stub_7fddcc() {
    // IDA 0x7fddcc: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

// 0x7fe0e4 — __ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::~callable()")]
pub fn stub_7fe0e4() {
    // IDA 0x7fe0e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fe110 — __ZN3rbx8callableINS_7signals6signalIFvSt4pairIPN3RBX9PrimitiveES6_EEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS4_10Soundscape21CollisionSoundManagerES7_EENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>,1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::~callable()")]
pub fn stub_7fe110() {
    // IDA 0x7fe110: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7fe1e4 — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>> *)")]
pub fn stub_7fe1e4() {
    // IDA 0x7fe1e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x7fe20c — __ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>> *)")]
pub fn stub_7fe20c() {
    // IDA 0x7fe20c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_b29db0() {
    // IDA 0xb29db0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xb29dc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_b29dc8() {
    // IDA 0xb29dc8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
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
pub fn stub_f303f4() {
    // IDA 0xf303f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf30404 — j___ZN3RBX10Reflection9ArgHelper6getArgINS_9SoundTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::SoundType RBX::Reflection::ArgHelper::getArg<RBX::SoundType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::SoundType> const&,boost::disable_if<boost::is_same<RBX::SoundType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_f30404() {
    // IDA 0xf30404: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_f304a4() {
    // IDA 0xf304a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf304b4 — j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_f304b4() -> ! {
    todo!("0xf304b4 j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv")
}

// 0xf304c4 — j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_f304c4() -> ! {
    todo!("0xf304c4 j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev")
}

// 0xf304d4 — j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_f304d4() {
    // IDA 0xf304d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf304e4 — j___ZN3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")]
pub fn stub_f304e4() -> ! {
    todo!("0xf304e4 RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")
}

// 0xf304f4 — j___ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v")]
pub fn stub_f304f4() -> ! {
    todo!("0xf304f4 j___ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v")
}

// 0xf30504 — j___ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v")]
pub fn stub_f30504() -> ! {
    todo!("0xf30504 j___ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v")
}

// 0xf30524 — j___ZN3RBX9CreatableINS_8InstanceEE6createI21SoundServiceStatsItemPKNS_10Soundscape12SoundServiceEEEN5boost10shared_ptrIT_EET0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)")]
pub fn stub_f30524() {
    // IDA 0xf30524: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30534 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10Soundscape12SoundChannelEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundChannel>(void)")]
pub fn stub_f30534() {
    // IDA 0xf30534: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30544 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10StockSoundEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(__guard *, int, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")]
pub fn stub_f30544() {
    // IDA 0xf30544: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30564 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape10ReverbTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::ReverbType>(RBX::Soundscape::ReverbType const&)")]
pub fn stub_f30564() -> ! {
    todo!("0xf30564 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::ReverbType>(RBX::Soundscape::ReverbType const&)")
}

// 0xf30574 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")]
pub fn stub_f30574() -> ! {
    todo!("0xf30574 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")
}

// 0xf30584 — j___ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv
// type: int(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")]
pub fn stub_f30584() -> ! {
    todo!("0xf30584 rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")
}

// 0xf30594 — j___ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE9singletonEv
// type: int(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::singleton(void)")]
pub fn stub_f30594() -> ! {
    todo!("0xf30594 rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::singleton(void)")
}

// 0xf305a4 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>> const&)")]
pub fn stub_f305a4() {
    // IDA 0xf305a4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0xf305b4 — j___ZN3rbx8any_castIN3RBX10Soundscape7SoundIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Soundscape::SoundId * rbx::any_cast<RBX::Soundscape::SoundId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_f305b4() -> ! {
    todo!("0xf305b4 RBX::Soundscape::SoundId * rbx::any_cast<RBX::Soundscape::SoundId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0xf305c4 — j___ZN3rbx8any_castIRKN3RBX10Soundscape10ReverbTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Soundscape::ReverbType const& rbx::any_cast<RBX::Soundscape::ReverbType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_f305c4() -> ! {
    todo!("0xf305c4 RBX::Soundscape::ReverbType const& rbx::any_cast<RBX::Soundscape::ReverbType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf305d4 — j___ZN3rbx8any_castIRKN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Soundscape::SoundId const& rbx::any_cast<RBX::Soundscape::SoundId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_f305d4() -> ! {
    todo!("0xf305d4 RBX::Soundscape::SoundId const& rbx::any_cast<RBX::Soundscape::SoundId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf305e4 — j___ZN3rbx8any_castIRN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Soundscape::SoundId & rbx::any_cast<RBX::Soundscape::SoundId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_f305e4() -> ! {
    todo!("0xf305e4 RBX::Soundscape::SoundId & rbx::any_cast<RBX::Soundscape::SoundId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf305f4 — j___ZN5boost10scoped_ptrIN3RBX10Soundscape21CollisionSoundManagerEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::scoped_ptr<RBX::Soundscape::CollisionSoundManager>::~scoped_ptr()")]
pub fn stub_f305f4() {
    // IDA 0xf305f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf30604 — j___ZN5boost10shared_ptrI21SoundServiceStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f30604() {
    // IDA 0xf30604: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30614 — j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::shared_ptr<RBX::Soundscape::SoundChannel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f30614() {
    // IDA 0xf30614: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30624 — j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>& rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const&)")]
pub fn stub_f30624() {
    // IDA 0xf30624: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30634 — j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEC2IS4_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::shared_ptr<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
pub fn stub_f30634() {
    // IDA 0xf30634: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30644 — j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::operator=(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const&)")]
pub fn stub_f30644() {
    // IDA 0xf30644: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30654 — j___ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEC2IS3_EEPT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::Sound>::shared_ptr<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
pub fn stub_f30654() {
    // IDA 0xf30654: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30664 — j___ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEaSERKS4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::Sound>::operator=(rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")]
pub fn stub_f30664() {
    // IDA 0xf30664: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30674 — j___ZN5boost10shared_ptrIN3RBX10StockSoundEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound>::shared_ptr<RBX::StockSound,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f30674() {
    // IDA 0xf30674: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30684 — j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSI21SoundServiceStatsItemEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const&)")]
pub fn stub_f30684() {
    // IDA 0xf30684: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30694 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
pub fn stub_f30694() {
    // IDA 0xf30694: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

// 0xf306a4 — j___ZN5boost6detail12shared_countC2IN3RBX10Soundscape12SoundService8SoundJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
pub fn stub_f306a4() {
    // IDA 0xf306a4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf306b4 — j___ZN5boost6detail12shared_countC2IN3RBX10Soundscape5SoundEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
pub fn stub_f306b4() {
    // IDA 0xf306b4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf306c4 — j___ZN5boost6detail12shared_countC2IP21SoundServiceStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f306c4() {
    // IDA 0xf306c4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf306d4 — j___ZN5boost6detail12shared_countC2IPN3RBX10Soundscape12SoundChannelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f306d4() {
    // IDA 0xf306d4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf306e4 — j___ZN5boost6detail12shared_countC2IPN3RBX10StockSoundENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f306e4() {
    // IDA 0xf306e4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf30704 — j___ZN9__gnu_cxx13new_allocatorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEE7destroyEPSA_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>::destroy(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>*)")]
pub fn stub_f30704() {
    // IDA 0xf30704: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30714 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_f30714() -> ! {
    todo!("0xf30714 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf30724 — j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE13convertToItemERKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToItem(RBX::Soundscape::ReverbType const&)const")]
pub fn stub_f30724() -> ! {
    todo!("0xf30724 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToItem(RBX::Soundscape::ReverbType const&)const")
}

// 0xf30734 — j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToIndex(RBX::Soundscape::ReverbType)const")]
pub fn stub_f30734() -> ! {
    todo!("0xf30734 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToIndex(RBX::Soundscape::ReverbType)const")
}

// 0xf30744 — j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToValue(RBX::Name const&,RBX::Soundscape::ReverbType&)const")]
pub fn stub_f30744() -> ! {
    todo!("0xf30744 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToValue(RBX::Name const&,RBX::Soundscape::ReverbType&)const")
}

// 0xf30754 — j___ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToString(RBX::Soundscape::ReverbType const&)const")]
pub fn stub_f30754() -> ! {
    todo!("0xf30754 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToString(RBX::Soundscape::ReverbType const&)const")
}

// 0xf30764 — j___ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv
// type: int __fastcall(_DWORD)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv")]
pub fn stub_f30764() -> ! {
    todo!("0xf30764 j___ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv")
}

// 0xf30774 — j___ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv
// type: int __fastcall(_DWORD)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_f30774() -> ! {
    todo!("0xf30774 j___ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0xf30784 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI21SoundServiceStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<SoundServiceStatsItem,SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const*,SoundServiceStatsItem *)const")]
pub fn stub_f30784() {
    // IDA 0xf30784: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf30794 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundChannelES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundChannel>(rbx_core::SharedPtr<RBX::Soundscape::SoundChannel> const*,RBX::Soundscape::SoundChannel *)const")]
pub fn stub_f30794() {
    // IDA 0xf30794: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf307a4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10StockSoundES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StockSound,RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const*,RBX::StockSound *)const")]
pub fn stub_f307a4() {
    // IDA 0xf307a4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf307b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10Soundscape12SoundService8SoundJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Soundscape::SoundService::SoundJob,RBX::Soundscape::SoundService::SoundJob>(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const*,RBX::Soundscape::SoundService::SoundJob *)const")]
pub fn stub_f307b4() {
    // IDA 0xf307b4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf307c4 — j___ZNSt12_Vector_baseIN3RBX10Soundscape10ReverbTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Vector_base<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_allocate(unsigned long)")]
pub fn stub_f307c4() -> ! {
    todo!("0xf307c4 std::_Vector_base<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_allocate(unsigned long)")
}

// 0xf307d4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Soundscape10ReverbTypeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Soundscape::ReverbType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *>(RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *)")]
pub fn stub_f307d4() -> ! {
    todo!("0xf307d4 RBX::Soundscape::ReverbType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *>(RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *)")
}

// 0xf307e4 — j___ZNSt3mapIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::operator[](RBX::Soundscape::SoundId const&)")]
pub fn stub_f307e4() {
    // IDA 0xf307e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf307f4 — j___ZNSt3mapIN3RBX9SoundTypeEN5boost10shared_ptrINS0_10Soundscape12SoundChannelEEESt4lessIS1_ESaISt4pairIKS1_S6_EEEixERSA_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::SoundType,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::operator[](RBX::SoundType const&)")]
pub fn stub_f307f4() {
    // IDA 0xf307f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30804 — j___ZNSt3mapIPKN3RBX4NameENS0_10Soundscape10ReverbTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::Soundscape::ReverbType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_f30804() -> ! {
    todo!("0xf30804 std::map<RBX::Name const*,RBX::Soundscape::ReverbType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::operator[](RBX::Name const* const&)")
}

// 0xf30814 — j___ZNSt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2ERS3_RKS7_
#[doc(alias = "std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>::pair(RBX::Soundscape::SoundId const&,rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")]
pub fn stub_f30814() {
    // IDA 0xf30814: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30824 — j___ZNSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2IKS2_S6_EERKS_IT_T0_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::pair<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>(std::pair const&<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)")]
pub fn stub_f30824() {
    // IDA 0xf30824: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30834 — j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,RBX::Soundscape::ReverbType const&)")]
pub fn stub_f30834() -> ! {
    todo!("0xf30834 std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,RBX::Soundscape::ReverbType const&)")
}

// 0xf30844 — j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(_DWORD)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,unsigned long,RBX::Soundscape::ReverbType const&)")]
pub fn stub_f30844() -> ! {
    todo!("0xf30844 std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,unsigned long,RBX::Soundscape::ReverbType const&)")
}

// 0xf30854 — j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::resize(unsigned long,RBX::Soundscape::ReverbType)")]
pub fn stub_f30854() -> ! {
    todo!("0xf30854 std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::resize(unsigned long,RBX::Soundscape::ReverbType)")
}

// 0xf30864 — j___ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::push_back(RBX::Soundscape::ReverbType const&)")]
pub fn stub_f30864() -> ! {
    todo!("0xf30864 std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::push_back(RBX::Soundscape::ReverbType const&)")
}

// 0xf30874 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_create_node(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_f30874() {
    // IDA 0xf30874: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30884 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_f30884() {
    // IDA 0xf30884: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30894 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_f30894() {
    // IDA 0xf30894: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf308a4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE4findERS4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::find(RBX::Soundscape::SoundId const&)")]
pub fn stub_f308a4() {
    // IDA 0xf308a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf308b4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>)")]
pub fn stub_f308b4() {
    // IDA 0xf308b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf308c4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>> *)")]
pub fn stub_f308c4() {
    // IDA 0xf308c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf308d4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_f308d4() {
    // IDA 0xf308d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf308e4 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE14_M_create_nodeERKS9_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_create_node(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_f308e4() {
    // IDA 0xf308e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf308f4 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
pub fn stub_f308f4() {
    // IDA 0xf308f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30904 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_f30904() {
    // IDA 0xf30904: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30914 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_f30914() {
    // IDA 0xf30914: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30924 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
pub fn stub_f30924() {
    // IDA 0xf30924: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30934 — j___ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_f30934() {
    // IDA 0xf30934: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30944 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
pub fn stub_f30944() -> ! {
    todo!("0xf30944 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")
}

// 0xf30954 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
pub fn stub_f30954() -> ! {
    todo!("0xf30954 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")
}

// 0xf30964 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>> *)")]
pub fn stub_f30964() {
    // IDA 0xf30964: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30974 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
pub fn stub_f30974() -> ! {
    todo!("0xf30974 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")
}

// 0xf30984 — j___ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEEPFvRKS1_IS4_S9_EEET0_T_SI_SH_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int)
#[doc(alias = "void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>) std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)>(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>))")]
pub fn stub_f30984() {
    // IDA 0xf30984: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf30994 — j___ZN3RBX10Reflection7Variant14genericConvertINS_9SoundTypeEEERT_v
#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")]
pub fn stub_f30994() -> ! {
    todo!("0xf30994 RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")
}

// 0xf309a4 — j___ZN3RBX10Reflection8EnumDescINS_9SoundTypeEE7addPairES2_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::addPair(RBX::SoundType,char const*)")]
pub fn stub_f309a4() -> ! {
    todo!("0xf309a4 RBX::Reflection::EnumDesc<RBX::SoundType>::addPair(RBX::SoundType,char const*)")
}

// 0xf309b4 — j___ZN3rbx8any_castIN3RBX9SoundTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::SoundType * rbx::any_cast<RBX::SoundType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_f309b4() -> ! {
    todo!("0xf309b4 RBX::SoundType * rbx::any_cast<RBX::SoundType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0xf309c4 — j___ZN3rbx8any_castIRN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::SoundType & rbx::any_cast<RBX::SoundType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_f309c4() -> ! {
    todo!("0xf309c4 RBX::SoundType & rbx::any_cast<RBX::SoundType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf309d4 — j___ZNSt12_Vector_baseIN3RBX9SoundTypeESaIS1_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Vector_base<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_allocate(unsigned long)")]
pub fn stub_f309d4() -> ! {
    todo!("0xf309d4 std::_Vector_base<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_allocate(unsigned long)")
}

// 0xf309e4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9SoundTypeES5_EET0_T_S7_S6_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::SoundType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SoundType *,RBX::SoundType *>(RBX::SoundType *,RBX::SoundType *,RBX::SoundType *)")]
pub fn stub_f309e4() -> ! {
    todo!("0xf309e4 RBX::SoundType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SoundType *,RBX::SoundType *>(RBX::SoundType *,RBX::SoundType *,RBX::SoundType *)")
}

// 0xf309f4 — j___ZNSt3mapIPKN3RBX4NameENS0_9SoundTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::SoundType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_f309f4() -> ! {
    todo!("0xf309f4 std::map<RBX::Name const*,RBX::SoundType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::operator[](RBX::Name const* const&)")
}

// 0xf30a04 — j___ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,RBX::SoundType const&)")]
pub fn stub_f30a04() -> ! {
    todo!("0xf30a04 std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,RBX::SoundType const&)")
}

// 0xf30a14 — j___ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(_DWORD)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,unsigned long,RBX::SoundType const&)")]
pub fn stub_f30a14() -> ! {
    todo!("0xf30a14 std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,unsigned long,RBX::SoundType const&)")
}

// 0xf30a24 — j___ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE6resizeEmS1_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::resize(unsigned long,RBX::SoundType)")]
pub fn stub_f30a24() -> ! {
    todo!("0xf30a24 std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::resize(unsigned long,RBX::SoundType)")
}

// 0xf30a34 — j___ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE9push_backERKS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::push_back(RBX::SoundType const&)")]
pub fn stub_f30a34() -> ! {
    todo!("0xf30a34 std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::push_back(RBX::SoundType const&)")
}

// 0xf30a44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
pub fn stub_f30a44() -> ! {
    todo!("0xf30a44 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SoundType> const&)")
}

// 0xf30a54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SoundType>>,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
pub fn stub_f30a54() -> ! {
    todo!("0xf30a54 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SoundType>>,std::pair<RBX::Name const* const,RBX::SoundType> const&)")
}

// 0xf30a64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
pub fn stub_f30a64() -> ! {
    todo!("0xf30a64 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SoundType> const&)")
}

// 0xf35dc4 — j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv
// type: int(void)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv")]
pub fn stub_f35dc4() -> ! {
    todo!("0xf35dc4 j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E15isNullClassNameEv")
}

// 0xf35dd4 — j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv
// type: void *__fastcall(int)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv")]
pub fn stub_f35dd4() -> ! {
    todo!("0xf35dd4 j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E17static_getCreatorEv")
}

// 0xf35de4 — j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorC2Ev
// type: int()
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorC2Ev")]
pub fn stub_f35de4() -> ! {
    todo!("0xf35de4 j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorC2Ev")
}

// 0xf35df4 — j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD2Ev
// type: int()
#[doc(alias = "j___ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD2Ev")]
pub fn stub_f35df4() {
    // IDA 0xf35df4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf360a4 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_10Soundscape12SoundServiceEEEmv
// type: int(void)
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Soundscape::SoundService>(void)")]
pub fn stub_f360a4() -> ! {
    todo!("0xf360a4 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Soundscape::SoundService>(void)")
}

// 0xf36404 — j___ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
pub fn stub_f36404() -> ! {
    todo!("0xf36404 j___ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")
}

// 0xf36614 — j___ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
pub fn stub_f36614() -> ! {
    todo!("0xf36614 j___ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")
}

// 0xf36824 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10Soundscape12SoundServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundService>(void)")]
pub fn stub_f36824() {
    // IDA 0xf36824: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf36ee4 — j___ZN5boost10shared_ptrIN3RBX10Soundscape12SoundServiceEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService>::shared_ptr<RBX::Soundscape::SoundService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f36ee4() {
    // IDA 0xf36ee4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf37154 — j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10Soundscape12SoundServiceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Soundscape::SoundService>(rbx_core::SharedPtr<RBX::Soundscape::SoundService> const&)")]
pub fn stub_f37154() {
    // IDA 0xf37154: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf37844 — j___ZN5boost6detail12shared_countC2IPN3RBX10Soundscape12SoundServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f37844() {
    // IDA 0xf37844: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf37ef4 — j___ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv
// type: int __fastcall(int)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv")]
pub fn stub_f37ef4() -> ! {
    todo!("0xf37ef4 j___ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7Creator12getClassNameEv")
}

// 0xf37f84 — j___ZNK3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(void)const")]
pub fn stub_f37f84() -> ! {
    todo!("0xf37f84 RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(void)const")
}

// 0xf38154 — j___ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(void)const")]
pub fn stub_f38154() -> ! {
    todo!("0xf38154 RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(void)const")
}

// 0xf38404 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundService,RBX::Soundscape::SoundService>(rbx_core::SharedPtr<RBX::Soundscape::SoundService> const*,RBX::Soundscape::SoundService *)const")]
pub fn stub_f38404() {
    // IDA 0xf38404: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0xf3b284 — j___ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
pub fn stub_f3b284() {
    // IDA 0xf3b284: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf3c0e4 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE14doGetSingletonEv
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::doGetSingleton(void)")]
pub fn stub_f3c0e4() -> ! {
    todo!("0xf3c0e4 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::doGetSingleton(void)")
}

// 0xf3c744 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9SoundTypeEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SoundType>(RBX::SoundType const&)")]
pub fn stub_f3c744() -> ! {
    todo!("0xf3c744 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SoundType>(RBX::SoundType const&)")
}

// 0xf3ca94 — j___ZN3rbx14implementation12typed_holderIN3RBX9SoundTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::SoundType>::singleton(void)")]
pub fn stub_f3ca94() -> ! {
    todo!("0xf3ca94 rbx::implementation::typed_holder<RBX::SoundType>::singleton(void)")
}

// 0xf3ce54 — j___ZN3rbx8any_castIRKN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::SoundType const& rbx::any_cast<RBX::SoundType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_f3ce54() -> ! {
    todo!("0xf3ce54 RBX::SoundType const& rbx::any_cast<RBX::SoundType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf3d9f4 — j___ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE13convertToItemERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToItem(RBX::SoundType const&)const")]
pub fn stub_f3d9f4() -> ! {
    todo!("0xf3d9f4 RBX::Reflection::EnumDesc<RBX::SoundType>::convertToItem(RBX::SoundType const&)const")
}

// 0xf3da04 — j___ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueERKNS_4NameERS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(RBX::Name const&,RBX::SoundType&)const")]
pub fn stub_f3da04() -> ! {
    todo!("0xf3da04 RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(RBX::Name const&,RBX::SoundType&)const")
}

// 0xf3da14 — j___ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringERKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(RBX::SoundType const&)const")]
pub fn stub_f3da14() -> ! {
    todo!("0xf3da14 RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(RBX::SoundType const&)const")
}

// 0xf3dee4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SoundType>> *)")]
pub fn stub_f3dee4() {
    // IDA 0xf3dee4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf4cfe4 — j___ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_f4cfe4() -> ! {
    todo!("0xf4cfe4 j___ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0xf526a4 — j___ZN3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(RBX::Instance const*)")]
pub fn stub_f526a4() -> ! {
    todo!("0xf526a4 RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(RBX::Instance const*)")
}

// 0xf547e4 — j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_10Soundscape21CollisionSoundManagerES4_EENS9_5list2INS9_5valueIPSE_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Primitive *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>> const&)")]
pub fn stub_f547e4() {
    // IDA 0xf547e4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0xf54824 — j___ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape21CollisionSoundManagerES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>> const&)")]
pub fn stub_f54824() {
    // IDA 0xf54824: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0xf54834 — j___ZN5boost10shared_ptrIN3RBX10Soundscape14CollisionSoundEEC2IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>::shared_ptr<RBX::Soundscape::CollisionSound>(RBX::Soundscape::CollisionSound *)")]
pub fn stub_f54834() {
    // IDA 0xf54834: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf54844 — j___ZN5boost10shared_ptrIN3RBX10Soundscape14CollisionSoundEEaSERKS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>::operator=(rbx_core::SharedPtr<RBX::Soundscape::CollisionSound> const&)")]
pub fn stub_f54844() {
    // IDA 0xf54844: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf54854 — j___ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEE5resetIS3_EEvPT_
#[doc(alias = "void rbx_core::SharedPtr<RBX::Soundscape::Sound>::reset<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
pub fn stub_f54854() {
    // IDA 0xf54854: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf54874 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX10Soundscape21CollisionSoundManagerEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_St4pairIPNS3_9PrimitiveESG_EEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>>,boost::_bi::list1<std::pair<RBX::Primitive *,RBX::Primitive *>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,std::pair<RBX::Primitive *,RBX::Primitive *>> &,boost::_bi::list1<std::pair<RBX::Primitive *,RBX::Primitive *>&> &,int)")]
pub fn stub_f54874() {
    // IDA 0xf54874: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

// 0xf54884 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape21CollisionSoundManagerEPNS4_9PrimitiveEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::CollisionSoundManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::CollisionSoundManager*>,boost::arg<1>>>::operator()<RBX::Primitive *>(RBX::Primitive * &)")]
pub fn stub_f54884() {
    // IDA 0xf54884: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

// 0xf54894 — j___ZN5boost6detail12shared_countC2IN3RBX10Soundscape14CollisionSoundEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::CollisionSound>(RBX::Soundscape::CollisionSound *)")]
pub fn stub_f54894() {
    // IDA 0xf54894: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf548a4 — j___ZNSt3mapIN3RBX10Soundscape18CollisionSoundTypeEN5boost10shared_ptrINS1_14CollisionSoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::Soundscape::CollisionSoundType,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::operator[](RBX::Soundscape::CollisionSoundType const&)")]
pub fn stub_f548a4() {
    // IDA 0xf548a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf548b4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_create_node(std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_f548b4() {
    // IDA 0xf548b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf548c4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>> *)")]
pub fn stub_f548c4() {
    // IDA 0xf548c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf548d4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_f548d4() {
    // IDA 0xf548d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf548e4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_f548e4() {
    // IDA 0xf548e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf548f4 — j___ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>> *)")]
pub fn stub_f548f4() {
    // IDA 0xf548f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0xf54904 — j___ZNSt8_Rb_treeIN3RBX10Soundscape18CollisionSoundTypeESt4pairIKS2_N5boost10shared_ptrINS1_14CollisionSoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::CollisionSoundType,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>,std::_Select1st<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>,std::less<RBX::Soundscape::CollisionSoundType>,std::allocator<std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::CollisionSoundType const,rbx_core::SharedPtr<RBX::Soundscape::CollisionSound>> const&)")]
pub fn stub_f54904() {
    // IDA 0xf54904: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// watchdog audio — 150 duplicate stubs for FMOD/Sound/Audio (EA-sorted 0x686a4..0x72a28) using rbx_core::SharedPtr
// 0x686a4 — __ZN4FMOD10ProfileCpu4initEv [watchdog]
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::init(void)")]
pub fn stub_686a4_wdog000(_cpu: &mut ProfileCpu) -> i32 {
    // Watchdog copy of IDA 0x686a4; see stub_686a4.
    crate::stub_686a4(_cpu)
}

// 0x686ac — __ZN4FMOD10ProfileCpu6updateEPNS_7SystemIEj [watchdog]
// type: int __fastcall(FMOD::ProfileCpu *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::ProfileCpu::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_686ac_wdog001(usage: Result<[f32; 4], i32>, add_packet: impl FnOnce(&[u8]) -> i32) -> i32 {
    // Watchdog copy of IDA 0x686ac; see stub_686ac.
    crate::stub_686ac(usage, add_packet)
}

// 0x68758 — __ZN4FMOD10ProfileCpu7releaseEv [watchdog]
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::release(void)")]
pub fn stub_68758_wdog002(_cpu: Box<ProfileCpu>) -> i32 {
    // Watchdog copy of IDA 0x68758; see stub_68758.
    crate::stub_68758(_cpu)
}

// 0x68794 — __ZN4FMOD10ProfileCpuC2Ev [watchdog]
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::ProfileCpu(void)")]
pub fn stub_68794_wdog003(cpu: &mut ProfileCpu) -> &mut ProfileCpu {
    // Watchdog copy of IDA 0x68794; see stub_68794.
    crate::stub_68794(cpu)
}

// 0x687bc — __ZN4FMOD10ProfileCpuC1Ev [watchdog]
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::ProfileCpu(void)")]
pub fn stub_687bc_wdog004(cpu: &mut ProfileCpu) -> &mut ProfileCpu {
    // Watchdog copy of IDA 0x687bc; see stub_687bc.
    crate::stub_687bc(cpu)
}

// 0x687c0 — __ZN4FMOD22FMOD_ProfileCpu_CreateEv [watchdog]
// type: int __fastcall(FMOD *this)
#[doc(alias = "FMOD::FMOD_ProfileCpu_Create(void)")]
pub fn stub_687c0_wdog005(slot: &mut Option<Box<ProfileCpu>>, profile: &mut Profile) -> i32 {
    // Watchdog copy of IDA 0x687c0; see stub_687c0.
    crate::stub_687c0(slot, profile)
}

// 0x68864 — __ZN4FMOD10ProfileDsp15isNodeDuplicateEy [watchdog]
// type: int __fastcall(FMOD::ProfileDsp *this, unsigned __int64)
#[doc(alias = "FMOD::ProfileDsp::isNodeDuplicate(unsigned long long)")]
pub fn stub_68864_wdog006(dsp: &ProfileDsp, id: u64) -> i32 {
    // Watchdog copy of IDA 0x68864; see stub_68864.
    crate::stub_68864(dsp, id)
}

// 0x68944 — __ZN4FMOD10ProfileDsp10sendPacketEPNS_7SystemIE [watchdog]
// type: int __fastcall(FMOD::ProfileDsp *this, FMOD::SystemI *)
#[doc(alias = "FMOD::ProfileDsp::sendPacket(FMOD::SystemI *)")]
pub fn stub_68944_wdog007(
    dsp: &mut ProfileDsp,
    usage: Result<f32, i32>,
    max_channels: u8,
    add_packet: impl FnOnce(&[u8]) -> i32,
) -> i32 {
    // Watchdog copy of IDA 0x68944; see stub_68944.
    crate::stub_68944(dsp, usage, max_channels, add_packet)
}

// 0x68a6c — __ZN4FMOD10ProfileDsp18growNodeStackSpaceEv [watchdog]
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::growNodeStackSpace(void)")]
pub fn stub_68a6c_wdog008(dsp: &mut ProfileDsp) -> i32 {
    // Watchdog copy of IDA 0x68a6c; see stub_68a6c.
    crate::stub_68a6c(dsp)
}

// 0x68adc — __ZN4FMOD10ProfileDsp15growPacketSpaceEv [watchdog]
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::growPacketSpace(void)")]
pub fn stub_68adc_wdog009(dsp: &mut ProfileDsp) -> i32 {
    // Watchdog copy of IDA 0x68adc; see stub_68adc.
    crate::stub_68adc(dsp)
}

// 0x68b68 — __ZN4FMOD10ProfileDsp6updateEPNS_7SystemIEj [watchdog]
// type: int __fastcall(FMOD::ProfileDsp *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::ProfileDsp::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_68b68_wdog010(
    dsp: &mut ProfileDsp,
    head: Result<usize, i32>,
    graph: &[DspSnapshot],
    dsp_usage: f32,
    max_channels: u8,
    add_packet: impl FnOnce(&[u8]) -> i32,
) -> i32 {
    // Watchdog copy of IDA 0x68b68; see stub_68b68.
    crate::stub_68b68(dsp, head, graph, dsp_usage, max_channels, add_packet)
}

// 0x68dfc — __ZN4FMOD10ProfileDsp7releaseEv [watchdog]
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::release(void)")]
pub fn stub_68dfc_wdog011(_dsp: Box<ProfileDsp>) -> i32 {
    // Watchdog copy of IDA 0x68dfc; see stub_68dfc.
    crate::stub_68dfc(_dsp)
}

// 0x68ebc — __ZN4FMOD10ProfileDsp4initEv [watchdog]
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::init(void)")]
pub fn stub_68ebc_wdog012(dsp: &mut ProfileDsp) -> i32 {
    // Watchdog copy of IDA 0x68ebc; see stub_68ebc.
    crate::stub_68ebc(dsp)
}

// 0x69028 — __ZN4FMOD10ProfileDspC2Ev [watchdog]
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::ProfileDsp(void)")]
pub fn stub_69028_wdog013(dsp: &mut ProfileDsp) -> &mut ProfileDsp {
    // Watchdog copy of IDA 0x69028; see stub_69028.
    crate::stub_69028(dsp)
}

// 0x69078 — __ZN4FMOD10ProfileDspC1Ev [watchdog]
// type: int __fastcall(FMOD::ProfileDsp *this)
#[doc(alias = "FMOD::ProfileDsp::ProfileDsp(void)")]
pub fn stub_69078_wdog014(dsp: &mut ProfileDsp) -> &mut ProfileDsp {
    // Watchdog copy of IDA 0x69078; see stub_69078.
    crate::stub_69078(dsp)
}

// 0x6907c — __ZN4FMOD22FMOD_ProfileDsp_CreateEv [watchdog]
// type: int __fastcall(FMOD *this)
#[doc(alias = "FMOD::FMOD_ProfileDsp_Create(void)")]
pub fn stub_6907c_wdog015(slot: &mut Option<Box<ProfileDsp>>, profile: &mut Profile) -> i32 {
    // Watchdog copy of IDA 0x6907c; see stub_6907c.
    crate::stub_6907c(slot, profile)
}

// 0x6914c — __ZN4FMOD7ProfileC2Ev [watchdog]
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::Profile::Profile(void)")]
pub fn stub_6914c_wdog016(profile: &mut Profile) -> &mut Profile {
    // Watchdog copy of IDA 0x6914c; see stub_6914c.
    crate::stub_6914c(profile)
}

// 0x6919c — __ZN4FMOD7ProfileC1Ev [watchdog]
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::Profile::Profile(void)")]
pub fn stub_6919c_wdog017(profile: &mut Profile) -> &mut Profile {
    // Watchdog copy of IDA 0x6919c; see stub_6919c.
    crate::stub_6919c(profile)
}

// 0x691a0 — __ZN4FMOD7Profile14registerModuleEPNS_13ProfileModuleE [watchdog]
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::Profile::registerModule(FMOD::ProfileModule *)")]
pub fn stub_691a0_wdog018(profile: &mut Profile, module: &mut ProfileModule) -> i32 {
    // Watchdog copy of IDA 0x691a0; see stub_691a0.
    crate::stub_691a0(profile, module)
}

// 0x691c8 — __ZN4FMOD13ProfileModuleC2Ev [watchdog]
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::ProfileModule::ProfileModule(void)")]
pub fn stub_691c8_wdog019(module: &mut ProfileModule) -> &mut ProfileModule {
    // Watchdog copy of IDA 0x691c8; see stub_691c8.
    crate::stub_691c8(module)
}

// 0x691fc — __ZN4FMOD13ProfileModule4initEv [watchdog]
// type: int __fastcall(FMOD::ProfileModule *this)
#[doc(alias = "FMOD::ProfileModule::init(void)")]
pub fn stub_691fc_wdog020(_module: &mut ProfileModule) -> i32 {
    // Watchdog copy of IDA 0x691fc; see stub_691fc.
    crate::stub_691fc(_module)
}

// 0x69204 — __ZN4FMOD13ProfileModule7releaseEv [watchdog]
// type: int __fastcall(FMOD::ProfileModule *this)
#[doc(alias = "FMOD::ProfileModule::release(void)")]
pub fn stub_69204_wdog021(_module: &mut ProfileModule) -> i32 {
    // Watchdog copy of IDA 0x69204; see stub_69204.
    crate::stub_69204(_module)
}

// 0x6920c — __ZN4FMOD13ProfileModule6updateEPNS_7SystemIEj [watchdog]
// type: int()
#[doc(alias = "FMOD::ProfileModule::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_6920c_wdog022() -> i32 {
    // Watchdog copy of IDA 0x6920c; see stub_6920c.
    crate::stub_6920c()
}

// 0x69214 — __ZN4FMOD13ProfileClientC2Ev [watchdog]
// type: char *__fastcall(FMOD::ProfileClient *this)
#[doc(alias = "FMOD::ProfileClient::ProfileClient(void)")]
pub fn stub_69214_wdog023(client: &mut ProfileClient) -> &mut ProfileClient {
    // Watchdog copy of IDA 0x69214; see stub_69214.
    crate::stub_69214(client)
}

// 0x69280 — __ZN4FMOD13ProfileClientC1Ev [watchdog]
// type: char *__fastcall(FMOD::ProfileClient *this)
#[doc(alias = "FMOD::ProfileClient::ProfileClient(void)")]
pub fn stub_69280_wdog024(client: &mut ProfileClient) -> &mut ProfileClient {
    // Watchdog copy of IDA 0x69280; see stub_69280.
    crate::stub_69280(client)
}

// 0x69284 — __ZN4FMOD13ProfileClient15requestDataTypeEhhj [watchdog]
// type: int __fastcall(FMOD::ProfileClient *this, int, int, unsigned int)
#[doc(alias = "FMOD::ProfileClient::requestDataType(unsigned char,unsigned char,unsigned int)")]
pub fn stub_69284_wdog025(client: &mut ProfileClient, ty_a: u8, ty_b: u8, interval: u32) -> i32 {
    // Watchdog copy of IDA 0x69284; see stub_69284.
    crate::stub_69284(client, ty_a, ty_b, interval)
}

// 0x69358 — __ZN4FMOD13ProfileClient9wantsDataEPNS_19ProfilePacketHeaderE [watchdog]
// type: bool __fastcall(int, unsigned __int8 *)
#[doc(alias = "FMOD::ProfileClient::wantsData(FMOD::ProfilePacketHeader *)")]
pub fn stub_69358_wdog026(client: &ProfileClient, packet: &[u8]) -> bool {
    // Watchdog copy of IDA 0x69358; see stub_69358.
    crate::stub_69358(client, packet)
}

// 0x693f4 — __ZN4FMOD13ProfileClient8sendDataEv [watchdog]
// type: int __fastcall(FMOD::ProfileClient *this)
#[doc(alias = "FMOD::ProfileClient::sendData(void)")]
pub fn stub_693f4_wdog027(client: &mut ProfileClient, net: &mut impl ClientNet) -> i32 {
    // Watchdog copy of IDA 0x693f4; see stub_693f4.
    crate::stub_693f4(client, net)
}

// 0x69480 — __ZN4FMOD13ProfileClient8readDataEv [watchdog]
// type: int __fastcall(const void **this)
#[doc(alias = "FMOD::ProfileClient::readData(void)")]
pub fn stub_69480_wdog028(client: &mut ProfileClient, net: &mut impl ClientNet) -> i32 {
    // Watchdog copy of IDA 0x69480; see stub_69480.
    crate::stub_69480(client, net)
}

// 0x695dc — __ZN4FMOD13ProfileClient6updateEj [watchdog]
// type: int __fastcall(FMOD::ProfileClient *this, unsigned int)
#[doc(alias = "FMOD::ProfileClient::update(unsigned int)")]
pub fn stub_695dc_wdog029(client: &mut ProfileClient, net: &mut impl ClientNet) -> i32 {
    // Watchdog copy of IDA 0x695dc; see stub_695dc.
    crate::stub_695dc(client, net)
}

// 0x69634 — __ZN4FMOD13ProfileClient9addPacketEPNS_19ProfilePacketHeaderE [watchdog]
// type: int __fastcall(FMOD::ProfileClient *this, unsigned __int8 *__src)
#[doc(alias = "FMOD::ProfileClient::addPacket(FMOD::ProfilePacketHeader *)")]
pub fn stub_69634_wdog030(client: &mut ProfileClient, packet: &[u8], net: &mut impl ClientNet) -> i32 {
    // Watchdog copy of IDA 0x69634; see stub_69634.
    crate::stub_69634(client, packet, net)
}

// 0x69820 — __ZN4FMOD13ProfileClient7releaseEv [watchdog]
// type: int __fastcall(const void **this)
#[doc(alias = "FMOD::ProfileClient::release(void)")]
pub fn stub_69820_wdog031(client: Box<ProfileClient>, close: impl FnOnce(i32)) -> i32 {
    // Watchdog copy of IDA 0x69820; see stub_69820.
    crate::stub_69820(client, close)
}

// 0x6989c — __ZN4FMOD13ProfileClient4initEPv [watchdog]
// type: int __fastcall(FMOD::ProfileClient *this, void *)
#[doc(alias = "FMOD::ProfileClient::init(void *)")]
pub fn stub_6989c_wdog032(client: &mut ProfileClient, socket: i32) -> i32 {
    // Watchdog copy of IDA 0x6989c; see stub_6989c.
    crate::stub_6989c(client, socket)
}

// 0x69910 — __ZN4FMOD7Profile17getMemoryUsedImplEPNS_13MemoryTrackerE [watchdog]
// type: int __fastcall(FMOD::Profile *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::Profile::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_69910_wdog033(
    profile: &Profile,
    extra: &ProfileMemExtra<'_>,
    sink: &mut impl MemTracker,
) -> i32 {
    // Watchdog copy of IDA 0x69910; see stub_69910.
    crate::stub_69910(profile, extra, sink)
}

// 0x69a78 — __ZN4FMOD7Profile7releaseEv [watchdog]
// type: int __fastcall(FMOD::Profile *this)
#[doc(alias = "FMOD::Profile::release(void)")]
pub fn stub_69a78_wdog034<N>(
    live: &mut ProfileLive<N>,
    os: &mut impl ProfileOs<N>,
    mut release_module: impl FnMut(usize, &mut ProfileModule) -> i32,
    mut clear_singleton: impl FnMut(usize),
) -> i32 {
    // Watchdog copy of IDA 0x69a78; see stub_69a78.
    crate::stub_69a78(live, os, release_module, clear_singleton)
}

// 0x69be8 — __ZN4FMOD20FMOD_Profile_ReleaseEv [watchdog]
// type: int __fastcall(FMOD *this)
#[doc(alias = "FMOD::FMOD_Profile_Release(void)")]
pub fn stub_69be8_wdog035<N>(
    slot: &mut Option<Box<ProfileLive<N>>>,
    os: &mut impl ProfileOs<N>,
    release_module: impl FnMut(usize, &mut ProfileModule) -> i32,
    clear_singleton: impl FnMut(usize),
) -> i32 {
    // Watchdog copy of IDA 0x69be8; see stub_69be8.
    crate::stub_69be8(slot, os, release_module, clear_singleton)
}

// 0x69c20 — __ZN4FMOD7Profile4initEt [watchdog]
// type: int __fastcall(FMOD::Profile *this, unsigned __int16)
#[doc(alias = "FMOD::Profile::init(unsigned short)")]
pub fn stub_69c20_wdog036<N>(profile: &mut Profile, port: u16, os: &mut impl ProfileOs<N>) -> i32 {
    // Watchdog copy of IDA 0x69c20; see stub_69c20.
    crate::stub_69c20(profile, port, os)
}

// 0x69c9c — __ZN4FMOD19FMOD_Profile_CreateEt [watchdog]
// type: int __fastcall(FMOD *this, unsigned __int16)
#[doc(alias = "FMOD::FMOD_Profile_Create(unsigned short)")]
pub fn stub_69c9c_wdog037<N>(
    slot: &mut Option<Box<ProfileLive<N>>>,
    port: u16,
    os: &mut impl ProfileOs<N>,
    release_module: impl FnMut(usize, &mut ProfileModule) -> i32,
    clear_singleton: impl FnMut(usize),
) -> i32 {
    // Watchdog copy of IDA 0x69c9c; see stub_69c9c.
    crate::stub_69c9c(slot, port, os, release_module, clear_singleton)
}

// 0x69d50 — __ZN4FMOD7Profile9addPacketEPNS_19ProfilePacketHeaderE [watchdog]
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "FMOD::Profile::addPacket(FMOD::ProfilePacketHeader *)")]
pub fn stub_69d50_wdog038<N>(live: &mut ProfileLive<N>, packet: &mut [u8], os: &mut impl ProfileOs<N>) -> i32
where
    N: ClientNet, {
    // Watchdog copy of IDA 0x69d50; see stub_69d50.
    crate::stub_69d50(live, packet, os)
}

// 0x69e0c — __ZN4FMOD7Profile6updateEPNS_7SystemIEj [watchdog]
// type: int __fastcall(FMOD::Profile *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::Profile::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_69e0c_wdog039<N>(
    live: &mut ProfileLive<N>,
    tick_ms: u32,
    os: &mut impl ProfileOs<N>,
    mut module_update: impl FnMut(usize, &mut ProfileModule) -> i32,
) -> i32
where
    N: ClientNet, {
    // Watchdog copy of IDA 0x69e0c; see stub_69e0c.
    crate::stub_69e0c(live, tick_ms, os, module_update)
}

// 0x6a018 — __ZN4FMOD19FMOD_Profile_UpdateEPNS_7SystemIEj [watchdog]
// type: int __fastcall(FMOD *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::FMOD_Profile_Update(FMOD::SystemI *,unsigned int)")]
pub fn stub_6a018_wdog040<N>(
    slot: &mut Option<Box<ProfileLive<N>>>,
    tick_ms: u32,
    os: &mut impl ProfileOs<N>,
    module_update: impl FnMut(usize, &mut ProfileModule) -> i32,
) -> i32
where
    N: ClientNet, {
    // Watchdog copy of IDA 0x6a018; see stub_6a018.
    crate::stub_6a018(slot, tick_ms, os, module_update)
}

// 0x6a04c — __ZN4FMOD7Profile13getMemoryUsedEPNS_13MemoryTrackerE [watchdog]
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::Profile::getMemoryUsed(FMOD::MemoryTracker *)")]
pub fn stub_6a04c_wdog041(profile: &mut Profile, flag: bool, run_impl: impl FnOnce() -> i32) -> i32 {
    // Watchdog copy of IDA 0x6a04c; see stub_6a04c.
    crate::stub_6a04c(profile, flag, run_impl)
}

// 0x6d26c — _FMOD_oggpack_look [watchdog]
// type: int __fastcall(int *, int)
#[doc(alias = "_FMOD_oggpack_look")]
pub unsafe fn stub_6d26c_wdog042(b: *mut OggpackBuffer, bits: i32) -> i32 {
    // Watchdog copy of IDA 0x6d26c; see stub_6d26c.
    crate::stub_6d26c(b, bits)
}

// 0x6d318 — _FMOD_oggpack_adv [watchdog]
// type: _DWORD *__fastcall(_DWORD *result, int)
#[doc(alias = "_FMOD_oggpack_adv")]
pub unsafe fn stub_6d318_wdog043(b: *mut OggpackBuffer, bits: i32) -> *mut OggpackBuffer {
    // Watchdog copy of IDA 0x6d318; see stub_6d318.
    crate::stub_6d318(b, bits)
}

// 0x6d354 — _FMOD_oggpack_read [watchdog]
// type: int __fastcall(int *, int)
#[doc(alias = "_FMOD_oggpack_read")]
pub unsafe fn stub_6d354_wdog044(b: *mut OggpackBuffer, bits: i32) -> i32 {
    // Watchdog copy of IDA 0x6d354; see stub_6d354.
    crate::stub_6d354(b, bits)
}

// 0x6d434 — _FMOD_oggpack_bytes [watchdog]
// type: int __fastcall(int *)
#[doc(alias = "_FMOD_oggpack_bytes")]
pub unsafe fn stub_6d434_wdog045(b: *const OggpackBuffer) -> i32 {
    // Watchdog copy of IDA 0x6d434; see stub_6d434.
    crate::stub_6d434(b)
}

// 0x6d44c — _FMOD_oggpack_readinit [watchdog]
// type: _DWORD *__fastcall(_DWORD *result, int, int)
#[doc(alias = "_FMOD_oggpack_readinit")]
pub unsafe fn stub_6d44c_wdog046(b: *mut OggpackBuffer, buf: *const u8, bytes: i32) -> *mut OggpackBuffer {
    // Watchdog copy of IDA 0x6d44c; see stub_6d44c.
    crate::stub_6d44c(b, buf, bytes)
}

// 0x6d4b4 — _FMOD_vorbis_synthesis_restart [watchdog]
// type: int __fastcall(int **)
#[doc(alias = "_FMOD_vorbis_synthesis_restart")]
pub unsafe fn stub_6d4b4_wdog047(v: *mut VorbisDspState) -> i32 {
    // Watchdog copy of IDA 0x6d4b4; see stub_6d4b4.
    crate::stub_6d4b4(v)
}

// 0x6d538 — _FMOD_vorbis_synthesis_pcmout [watchdog]
// type: int __fastcall(int *, _DWORD *)
#[doc(alias = "_FMOD_vorbis_synthesis_pcmout")]
pub unsafe fn stub_6d538_wdog048(v: *const VorbisDspState, pcm: *mut *mut *mut f32) -> i32 {
    // Watchdog copy of IDA 0x6d538; see stub_6d538.
    crate::stub_6d538(v, pcm)
}

// 0x6d5c8 — _FMOD_vorbis_synthesis_read [watchdog]
// type: int __fastcall(int, int)
#[doc(alias = "_FMOD_vorbis_synthesis_read")]
pub unsafe fn stub_6d5c8_wdog049(v: *mut VorbisDspState, n: i32) -> i32 {
    // Watchdog copy of IDA 0x6d5c8; see stub_6d5c8.
    crate::stub_6d5c8(v, n)
}

// 0x6d600 — _FMOD_vorbis_synthesis_blockin [watchdog]
// type: int __fastcall(int *, int)
#[doc(alias = "_FMOD_vorbis_synthesis_blockin")]
pub unsafe fn stub_6d600_wdog050(os: &impl VorbisCodecOs, v: *mut VorbisDspState, vb: *mut VorbisBlock) -> i32 {
    // Watchdog copy of IDA 0x6d600; see stub_6d600.
    crate::stub_6d600(os, v, vb)
}

// 0x6dee8 — __FMOD_vorbis_block_alloc [watchdog]
// type: int __fastcall(int, _DWORD *, int)
#[doc(alias = "__FMOD_vorbis_block_alloc")]
pub unsafe fn stub_6dee8_wdog051(vb: *mut VorbisBlock, bytes: i32, heap: &mut impl VorbisHeap) -> *mut u8 {
    // Watchdog copy of IDA 0x6dee8; see stub_6dee8.
    crate::stub_6dee8(vb, bytes, heap)
}

// 0x6df94 — __FMOD_vorbis_block_ripcord [watchdog]
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "__FMOD_vorbis_block_ripcord")]
pub unsafe fn stub_6df94_wdog052(vb: *mut VorbisBlock, heap: &mut impl VorbisHeap) -> i32 {
    // Watchdog copy of IDA 0x6df94; see stub_6df94.
    crate::stub_6df94(vb, heap)
}

// 0x6e044 — _FMOD_vorbis_block_init [watchdog]
// type: int __fastcall(int, int, void *__b)
#[doc(alias = "_FMOD_vorbis_block_init")]
pub unsafe fn stub_6e044_wdog053(vb: *mut VorbisBlock, vd: *mut VorbisDspState) -> i32 {
    // Watchdog copy of IDA 0x6e044; see stub_6e044.
    crate::stub_6e044(vb, vd)
}

// 0x6e078 — _FMOD_vorbis_dsp_clear [watchdog]
// type: void *__fastcall(void *result, int *, int, int)
#[doc(alias = "_FMOD_vorbis_dsp_clear")]
pub unsafe fn stub_6e078_wdog054(os: &mut impl VorbisCodecOs, dsp: *mut VorbisDspState) -> *mut VorbisDspState {
    // Watchdog copy of IDA 0x6e078; see stub_6e078.
    crate::stub_6e078(os, dsp)
}

// 0x6e2c4 — _FMOD_vorbis_synthesis_init [watchdog]
// type: int __fastcall(void *, int *__b, int, int)
#[doc(alias = "_FMOD_vorbis_synthesis_init")]
pub unsafe fn stub_6e2c4_wdog055(os: &mut impl VorbisCodecOs, dsp: *mut VorbisDspState, vi: *mut u8) -> i32 {
    // Watchdog copy of IDA 0x6e2c4; see stub_6e2c4.
    crate::stub_6e2c4(os, dsp, vi)
}

// 0x6e6c0 — _FMOD_vorbis_block_clear [watchdog]
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "_FMOD_vorbis_block_clear")]
pub unsafe fn stub_6e6c0_wdog056(vb: *mut VorbisBlock, heap: &mut impl VorbisHeap) -> i32 {
    // Watchdog copy of IDA 0x6e6c0; see stub_6e6c0.
    crate::stub_6e6c0(vb, heap)
}

// 0x6e778 — _FMOD_vorbis_book_decode [watchdog]
// type: int __fastcall(int *, int *)
#[doc(alias = "_FMOD_vorbis_book_decode")]
pub unsafe fn stub_6e778_wdog057(bk: *const VorbisCodebook, opb: *mut OggpackBuffer) -> i32 {
    // Watchdog copy of IDA 0x6e778; see stub_6e778.
    crate::stub_6e778(bk, opb)
}

// 0x6e8c4 — _FMOD_vorbis_staticbook_unpack [watchdog]
// type: int __fastcall(int, int *, int *)
#[doc(alias = "_FMOD_vorbis_staticbook_unpack")]
pub unsafe fn stub_6e8c4_wdog058(
    heap: &mut impl VorbisHeap,
    opb: *mut OggpackBuffer,
    book: *mut VorbisStaticBook,
) -> i32 {
    // Watchdog copy of IDA 0x6e8c4; see stub_6e8c4.
    crate::stub_6e8c4(heap, opb, book)
}

// 0x6ec78 — _FMOD_vorbis_book_decodevv_add [watchdog]
// type: int __fastcall(int *, int, int, int, int *, int)
#[doc(alias = "_FMOD_vorbis_book_decodevv_add")]
pub unsafe fn stub_6ec78_wdog059(
    bk: *const VorbisCodebook,
    chans: *const *mut f32,
    off: i32,
    nch: i32,
    opb: *mut OggpackBuffer,
    count: i32,
) -> i32 {
    // Watchdog copy of IDA 0x6ec78; see stub_6ec78.
    crate::stub_6ec78(bk, chans, off, nch, opb, count)
}

// 0x6ee98 — _FMOD_vorbis_book_decodev_add [watchdog]
// type: int __fastcall(int *, int, int *, int)
#[doc(alias = "_FMOD_vorbis_book_decodev_add")]
pub unsafe fn stub_6ee98_wdog060(
    bk: *const VorbisCodebook,
    out: *mut f32,
    opb: *mut OggpackBuffer,
    n: i32,
) -> i32 {
    // Watchdog copy of IDA 0x6ee98; see stub_6ee98.
    crate::stub_6ee98(bk, out, opb, n)
}

// 0x6f37c — _FMOD_vorbis_book_decodevs_add [watchdog]
// type: int __fastcall(int *, __int32 *, int *, int)
#[doc(alias = "_FMOD_vorbis_book_decodevs_add")]
pub unsafe fn stub_6f37c_wdog061(
    bk: *const VorbisCodebook,
    out: *mut f32,
    opb: *mut OggpackBuffer,
    total: i32,
) -> i32 {
    // Watchdog copy of IDA 0x6f37c; see stub_6f37c.
    crate::stub_6f37c(bk, out, opb, total)
}

// 0x6f840 — _FMOD_floor1_inverse1 [watchdog]
// type: int *__fastcall(int, int, _DWORD *)
#[doc(alias = "_FMOD_floor1_inverse1")]
pub unsafe fn stub_6f840_wdog062(
    heap: &mut impl VorbisHeap,
    vb: *mut VorbisBlock,
    look: *const Floor1Look,
) -> *mut i32 {
    // Watchdog copy of IDA 0x6f840; see stub_6f840.
    crate::stub_6f840(heap, vb, look)
}

// 0x6fbac — _FMOD_floor1_free_look [watchdog]
// type: int __fastcall(int result, void *)
#[doc(alias = "_FMOD_floor1_free_look")]
pub unsafe fn stub_6fbac_wdog063(heap: &mut impl VorbisHeap, look: *mut Floor1Look) -> i32 {
    // Watchdog copy of IDA 0x6fbac; see stub_6fbac.
    crate::stub_6fbac(heap, look)
}

// 0x6fbe0 — _FMOD_floor1_look [watchdog]
// type: _DWORD *__fastcall(int, int, int *)
#[doc(alias = "_FMOD_floor1_look")]
pub unsafe fn stub_6fbe0_wdog064(heap: &mut impl VorbisHeap, info: *const Floor1Info) -> *mut Floor1Look {
    // Watchdog copy of IDA 0x6fbe0; see stub_6fbe0.
    crate::stub_6fbe0(heap, info)
}

// 0x6fe68 — _FMOD_floor1_free_info [watchdog]
// type: int __fastcall(int result, void *)
#[doc(alias = "_FMOD_floor1_free_info")]
pub unsafe fn stub_6fe68_wdog065(heap: &mut impl VorbisHeap, info: *mut Floor1Info) -> i32 {
    // Watchdog copy of IDA 0x6fe68; see stub_6fe68.
    crate::stub_6fe68(heap, info)
}

// 0x6fe9c — _FMOD_floor1_unpack [watchdog]
// type: int *__fastcall(int, int, int *)
#[doc(alias = "_FMOD_floor1_unpack")]
pub unsafe fn stub_6fe9c_wdog066(
    heap: &mut impl VorbisHeap,
    vi: *const u32,
    opb: *mut OggpackBuffer,
) -> *mut Floor1Info {
    // Watchdog copy of IDA 0x6fe9c; see stub_6fe9c.
    crate::stub_6fe9c(heap, vi, opb)
}

// 0x701fc — _FMOD_floor1_inverse2 [watchdog]
// type: int __fastcall(int, int, int, _DWORD *, char *__b)
#[doc(alias = "_FMOD_floor1_inverse2")]
pub unsafe fn stub_701fc_wdog067(
    vb: *mut VorbisBlock,
    look: *const Floor1Look,
    fit: *const i32,
    out: *mut f32,
) -> i32 {
    // Watchdog copy of IDA 0x701fc; see stub_701fc.
    crate::stub_701fc(vb, look, fit, out)
}

// 0x70458 — _FMOD_Channel_GetUserData [watchdog]
// type: int __fastcall(FMOD::Channel *, void **)
#[doc(alias = "_FMOD_Channel_GetUserData")]
pub unsafe fn stub_70458_wdog068(
    ch: *const u8,
    out: *mut *mut u8,
    get_user_data: impl FnOnce(*const u8, *mut *mut u8) -> i32,
) -> i32 {
    // Watchdog copy of IDA 0x70458; see stub_70458.
    crate::stub_70458(ch, out, get_user_data)
}

// 0x70474 — _FMOD_System_Create [watchdog]
// type: int __fastcall(FMOD::SystemI **)
#[doc(alias = "_FMOD_System_Create")]
pub fn stub_70474_wdog069() -> ! {
    todo!("0x70474 _FMOD_System_Create")
}

// 0x705cc — _FMOD_Memory_GetStats [watchdog]
// type: int __fastcall(_DWORD *, _DWORD *, int)
#[doc(alias = "_FMOD_Memory_GetStats")]
pub fn stub_705cc_wdog070() -> ! {
    todo!("0x705cc _FMOD_Memory_GetStats")
}

// 0x7069c — __ZN4FMOD11AsyncThread7releaseEv [watchdog]
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::release(void)")]
pub unsafe fn stub_7069c_wdog071(t: *mut AsyncThread) -> i32 {
    // Watchdog copy of IDA 0x7069c; see stub_7069c.
    crate::stub_7069c(t)
}

// 0x706b4 — __ZN4FMOD11AsyncThread10threadFuncEv [watchdog]
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::threadFunc(void)")]
pub fn stub_706b4_wdog072() -> ! {
    todo!("0x706b4 FMOD::AsyncThread::threadFunc(void)")
}

// 0x70ab0 — __ZN4FMOD15asyncThreadFuncEPv [watchdog]
// type: int __fastcall(FMOD::AsyncThread *this, void *)
#[doc(alias = "FMOD::asyncThreadFunc(void *)")]
pub fn stub_70ab0_wdog073() -> ! {
    todo!("0x70ab0 FMOD::asyncThreadFunc(void *)")
}

// 0x70ab4 — __ZN4FMOD11AsyncThread13reallyReleaseEv [watchdog]
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::reallyRelease(void)")]
pub unsafe fn stub_70ab4_wdog074(t: *mut AsyncThread, os: &mut impl AsyncOs) -> i32 {
    // Watchdog copy of IDA 0x70ab4; see stub_70ab4.
    crate::stub_70ab4(t, os)
}

// 0x70bbc — __ZN4FMOD11AsyncThread4initEbPNS_7SystemIE [watchdog]
// type: int __fastcall(FMOD::AsyncThread *this, bool, FMOD::SystemI *)
#[doc(alias = "FMOD::AsyncThread::init(bool,FMOD::SystemI *)")]
pub unsafe fn stub_70bbc_wdog075(
    reg: &mut AsyncRegistry,
    global_lock: *mut u8,
    t: *mut AsyncThread,
    blocking: bool,
    sys: *mut u8,
    os: &mut impl AsyncOs,
) -> i32 {
    // Watchdog copy of IDA 0x70bbc; see stub_70bbc.
    crate::stub_70bbc(reg, global_lock, t, blocking, sys, os)
}

// 0x70c98 — __ZN4FMOD11AsyncThreadC2Ev [watchdog]
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::AsyncThread(void)")]
pub unsafe fn stub_70c98_wdog076(t: *mut AsyncThread, os: &mut impl AsyncOs) -> i32 {
    // Watchdog copy of IDA 0x70c98; see stub_70c98.
    crate::stub_70c98(t, os)
}

// 0x70cec — __ZN4FMOD11AsyncThreadC1Ev [watchdog]
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::AsyncThread(void)")]
pub unsafe fn stub_70cec_wdog077(t: *mut AsyncThread, os: &mut impl AsyncOs) -> i32 {
    // Watchdog copy of IDA 0x70cec; see stub_70cec.
    crate::stub_70cec(t, os)
}

// 0x70cf0 — __ZN4FMOD11AsyncThread14getAsyncThreadEPNS_6SoundIE [watchdog]
// type: int __fastcall(FMOD::AsyncThread *this, FMOD::SoundI *)
#[doc(alias = "FMOD::AsyncThread::getAsyncThread(FMOD::SoundI *)")]
pub unsafe fn stub_70cf0_wdog078(
    reg: &mut AsyncRegistry,
    global_lock: *mut u8,
    owner: *mut AsyncSoundView,
    os: &mut impl AsyncOs,
) -> i32 {
    // Watchdog copy of IDA 0x70cf0; see stub_70cf0.
    crate::stub_70cf0(reg, global_lock, owner, os)
}

// 0x70ddc — __ZN4FMOD11AsyncThread8shutDownEv [watchdog]
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::shutDown(void)")]
pub fn stub_70ddc_wdog079() -> ! {
    todo!("0x70ddc FMOD::AsyncThread::shutDown(void)")
}

// 0x70e5c — __ZN4FMOD11AsyncThread6updateEv [watchdog]
// type: int __fastcall(FMOD::AsyncThread *this)
#[doc(alias = "FMOD::AsyncThread::update(void)")]
pub fn stub_70e5c_wdog080() -> ! {
    todo!("0x70e5c FMOD::AsyncThread::update(void)")
}

// 0x70f2c — __GLOBAL__I__ZN4FMOD11AsyncThread10gAsyncHeadE [watchdog]
// type: int()
#[doc(alias = "global constructor keyed to FMOD::AsyncThread::gAsyncHead")]
pub fn stub_70f2c_wdog081() -> ! {
    todo!("0x70f2c global constructor keyed to FMOD::AsyncThread::gAsyncHead")
}

// 0x70f38 — __ZN4FMOD7Channel11getUserDataEPPv [watchdog]
// type: int __fastcall(FMOD::Channel *this, void **, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::getUserData(void **)")]
pub fn stub_70f38_wdog082() -> ! {
    todo!("0x70f38 FMOD::Channel::getUserData(void **)")
}

// 0x70f7c — __ZN4FMOD7Channel11setUserDataEPv [watchdog]
// type: int __fastcall(FMOD::Channel *this, void *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setUserData(void *)")]
pub fn stub_70f7c_wdog083() -> ! {
    todo!("0x70f7c FMOD::Channel::setUserData(void *)")
}

// 0x70fb0 — __ZN4FMOD7Channel12setLoopCountEi [watchdog]
// type: int __fastcall(FMOD::Channel *this, int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setLoopCount(int)")]
pub fn stub_70fb0_wdog084() -> ! {
    todo!("0x70fb0 FMOD::Channel::setLoopCount(int)")
}

// 0x70fe4 — __ZN4FMOD7Channel7getModeEPj [watchdog]
// type: int __fastcall(FMOD::Channel *this, unsigned int *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::getMode(unsigned int *)")]
pub fn stub_70fe4_wdog085() -> ! {
    todo!("0x70fe4 FMOD::Channel::getMode(unsigned int *)")
}

// 0x71028 — __ZN4FMOD7Channel7setModeEj [watchdog]
// type: int __fastcall(FMOD::Channel *this, unsigned int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setMode(unsigned int)")]
pub fn stub_71028_wdog086() -> ! {
    todo!("0x71028 FMOD::Channel::setMode(unsigned int)")
}

// 0x7105c — __ZN4FMOD7Channel9isPlayingEPb [watchdog]
// type: int __fastcall(FMOD::Channel *this, bool *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::isPlaying(bool *)")]
pub fn stub_7105c_wdog087() -> ! {
    todo!("0x7105c FMOD::Channel::isPlaying(bool *)")
}

// 0x710a0 — __ZN4FMOD7Channel15set3DAttributesEPK11FMOD_VECTORS3_ [watchdog]
// type: int __fastcall(FMOD::ChannelI *, int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")]
pub fn stub_710a0_wdog088() -> ! {
    todo!("0x710a0 FMOD::Channel::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")
}

// 0x710dc — __ZN4FMOD7Channel11setCallbackEPF11FMOD_RESULTP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS5_E [watchdog]
// type: int __fastcall(FMOD::ChannelI *, int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")]
pub fn stub_710dc_wdog089() -> ! {
    todo!("0x710dc FMOD::Channel::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")
}

// 0x71110 — __ZN4FMOD7Channel15setChannelGroupEPNS_12ChannelGroupE [watchdog]
// type: int __fastcall(FMOD::ChannelI *, FMOD::ChannelGroupI *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setChannelGroup(FMOD::ChannelGroup *)")]
pub fn stub_71110_wdog090() -> ! {
    todo!("0x71110 FMOD::Channel::setChannelGroup(FMOD::ChannelGroup *)")
}

// 0x71144 — __ZN4FMOD7Channel11setPriorityEi [watchdog]
// type: int __fastcall(FMOD::Channel *this, int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setPriority(int)")]
pub fn stub_71144_wdog091() -> ! {
    todo!("0x71144 FMOD::Channel::setPriority(int)")
}

// 0x71178 — __ZN4FMOD7Channel7setMuteEb [watchdog]
// type: int __fastcall(FMOD::Channel *this, bool, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setMute(bool)")]
pub fn stub_71178_wdog092() -> ! {
    todo!("0x71178 FMOD::Channel::setMute(bool)")
}

// 0x711ac — __ZN4FMOD7Channel12getFrequencyEPf [watchdog]
// type: int __fastcall(FMOD::Channel *this, float *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::getFrequency(float *)")]
pub fn stub_711ac_wdog093() -> ! {
    todo!("0x711ac FMOD::Channel::getFrequency(float *)")
}

// 0x711f0 — __ZN4FMOD7Channel12setFrequencyEf [watchdog]
// type: int __fastcall(FMOD::Channel *this, float, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setFrequency(float)")]
pub fn stub_711f0_wdog094() -> ! {
    todo!("0x711f0 FMOD::Channel::setFrequency(float)")
}

// 0x71224 — __ZN4FMOD7Channel9setVolumeEf [watchdog]
// type: int __fastcall(FMOD::Channel *this, float, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setVolume(float)")]
pub fn stub_71224_wdog095() -> ! {
    todo!("0x71224 FMOD::Channel::setVolume(float)")
}

// 0x71260 — __ZN4FMOD7Channel9getPausedEPb [watchdog]
// type: int __fastcall(FMOD::Channel *this, bool *, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::getPaused(bool *)")]
pub fn stub_71260_wdog096() -> ! {
    todo!("0x71260 FMOD::Channel::getPaused(bool *)")
}

// 0x712a4 — __ZN4FMOD7Channel9setPausedEb [watchdog]
// type: int __fastcall(FMOD::Channel *this, bool, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::setPaused(bool)")]
pub fn stub_712a4_wdog097() -> ! {
    todo!("0x712a4 FMOD::Channel::setPaused(bool)")
}

// 0x712d8 — __ZN4FMOD7Channel4stopEv [watchdog]
// type: int __fastcall(FMOD::Channel *this, int, FMOD::ChannelI **)
#[doc(alias = "FMOD::Channel::stop(void)")]
pub fn stub_712d8_wdog098() -> ! {
    todo!("0x712d8 FMOD::Channel::stop(void)")
}

// 0x71304 — __ZN4FMOD15ChannelEmulated9isVirtualEPb [watchdog]
// type: int __fastcall(FMOD::ChannelEmulated *this, bool *, int, bool)
#[doc(alias = "FMOD::ChannelEmulated::isVirtual(bool *)")]
pub fn stub_71304_wdog099() -> ! {
    todo!("0x71304 FMOD::ChannelEmulated::isVirtual(bool *)")
}

// 0x7131c — __ZN4FMOD15ChannelEmulated10getDSPHeadEPPNS_4DSPIE [watchdog]
// type: int __fastcall(int, int *)
#[doc(alias = "FMOD::ChannelEmulated::getDSPHead(FMOD::DSPI **)")]
pub fn stub_7131c_wdog100() -> ! {
    todo!("0x7131c FMOD::ChannelEmulated::getDSPHead(FMOD::DSPI **)")
}

// 0x71334 — __ZN4FMOD15ChannelEmulated16setSpeakerLevelsEiPfi [watchdog]
// type: int __fastcall(FMOD::ChannelEmulated *this, int, float *, int)
#[doc(alias = "FMOD::ChannelEmulated::setSpeakerLevels(int,float *,int)")]
pub fn stub_71334_wdog101() -> ! {
    todo!("0x71334 FMOD::ChannelEmulated::setSpeakerLevels(int,float *,int)")
}

// 0x7133c — __ZN4FMOD15ChannelEmulated13setSpeakerMixEffffffff [watchdog]
// type: int __fastcall(FMOD::ChannelEmulated *this, float, float, float, float, float, float, float, float)
#[doc(alias = "FMOD::ChannelEmulated::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_7133c_wdog102() -> ! {
    todo!("0x7133c FMOD::ChannelEmulated::setSpeakerMix(float,float,float,float,float,float,float,float)")
}

// 0x71344 — __ZN4FMOD15ChannelEmulated6updateEi [watchdog]
// type: int __fastcall(FMOD::ChannelEmulated *this, int)
#[doc(alias = "FMOD::ChannelEmulated::update(int)")]
pub fn stub_71344_wdog103() -> ! {
    todo!("0x71344 FMOD::ChannelEmulated::update(int)")
}

// 0x71540 — __ZN4FMOD15ChannelEmulated5closeEv [watchdog]
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::close(void)")]
pub fn stub_71540_wdog104() -> ! {
    todo!("0x71540 FMOD::ChannelEmulated::close(void)")
}

// 0x71580 — __ZN4FMOD15ChannelEmulated5allocEv [watchdog]
// type: int __fastcall(FMOD::DSPI **this)
#[doc(alias = "FMOD::ChannelEmulated::alloc(void)")]
pub fn stub_71580_wdog105() -> ! {
    todo!("0x71580 FMOD::ChannelEmulated::alloc(void)")
}

// 0x715e8 — __ZN4FMOD15ChannelEmulated4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE [watchdog]
// type: int __fastcall(FMOD::ChannelEmulated *this, int, FMOD::SystemI *, FMOD::Output *, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelEmulated::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
pub fn stub_715e8_wdog106() -> ! {
    todo!("0x715e8 FMOD::ChannelEmulated::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")
}

// 0x71698 — __ZN4FMOD15ChannelEmulatedC2Ev [watchdog]
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::ChannelEmulated(void)")]
pub fn stub_71698_wdog107() -> ! {
    todo!("0x71698 FMOD::ChannelEmulated::ChannelEmulated(void)")
}

// 0x716e4 — __ZN4FMOD15ChannelEmulatedC1Ev [watchdog]
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::ChannelEmulated(void)")]
pub fn stub_716e4_wdog108() -> ! {
    todo!("0x716e4 FMOD::ChannelEmulated::ChannelEmulated(void)")
}

// 0x716e8 — __ZN4FMOD15ChannelEmulated4stopEv [watchdog]
// type: int __fastcall(FMOD::ChannelEmulated *this)
#[doc(alias = "FMOD::ChannelEmulated::stop(void)")]
pub fn stub_716e8_wdog109() -> ! {
    todo!("0x716e8 FMOD::ChannelEmulated::stop(void)")
}

// 0x71818 — __ZN4FMOD15ChannelEmulatedD0Ev [watchdog]
// type: void __fastcall(FMOD::ChannelEmulated *__hidden this)
#[doc(alias = "FMOD::ChannelEmulated::~ChannelEmulated()")]
pub fn stub_71818_wdog110() -> ! {
    todo!("0x71818 FMOD::ChannelEmulated::~ChannelEmulated()")
}

// 0x7183c — __ZN4FMOD15ChannelEmulatedD1Ev [watchdog]
// type: void __fastcall(FMOD::ChannelEmulated *__hidden this)
#[doc(alias = "FMOD::ChannelEmulated::~ChannelEmulated()")]
pub fn stub_7183c_wdog111() -> ! {
    todo!("0x7183c FMOD::ChannelEmulated::~ChannelEmulated()")
}

// 0x71854 — __ZN4FMOD11ChannelRealC2Ev [watchdog]
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::ChannelReal::ChannelReal(void)")]
pub fn stub_71854_wdog112() -> ! {
    todo!("0x71854 FMOD::ChannelReal::ChannelReal(void)")
}

// 0x718a0 — __ZN4FMOD11ChannelReal4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE [watchdog]
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "FMOD::ChannelReal::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
pub fn stub_718a0_wdog113() -> ! {
    todo!("0x718a0 FMOD::ChannelReal::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")
}

// 0x718dc — __ZN4FMOD11ChannelReal5closeEv [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::close(void)")]
pub fn stub_718dc_wdog114() -> ! {
    todo!("0x718dc FMOD::ChannelReal::close(void)")
}

// 0x718e8 — __ZN4FMOD11ChannelReal5allocEv [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::alloc(void)")]
pub fn stub_718e8_wdog115() -> ! {
    todo!("0x718e8 FMOD::ChannelReal::alloc(void)")
}

// 0x7190c — __ZN4FMOD11ChannelReal5allocEPNS_4DSPIE [watchdog]
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelReal::alloc(FMOD::DSPI *)")]
pub fn stub_7190c_wdog116() -> ! {
    todo!("0x7190c FMOD::ChannelReal::alloc(FMOD::DSPI *)")
}

// 0x71930 — __ZN4FMOD11ChannelReal23set2DFreqVolumePanFor3DEv [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::set2DFreqVolumePanFor3D(void)")]
pub fn stub_71930_wdog117() -> ! {
    todo!("0x71930 FMOD::ChannelReal::set2DFreqVolumePanFor3D(void)")
}

// 0x71938 — __ZN4FMOD11ChannelReal6updateEi [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, int)
#[doc(alias = "FMOD::ChannelReal::update(int)")]
pub fn stub_71938_wdog118() -> ! {
    todo!("0x71938 FMOD::ChannelReal::update(int)")
}

// 0x71940 — __ZN4FMOD11ChannelReal12updateStreamEv [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::updateStream(void)")]
pub fn stub_71940_wdog119() -> ! {
    todo!("0x71940 FMOD::ChannelReal::updateStream(void)")
}

// 0x71948 — __ZN4FMOD11ChannelReal5startEv [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::start(void)")]
pub fn stub_71948_wdog120() -> ! {
    todo!("0x71948 FMOD::ChannelReal::start(void)")
}

// 0x71950 — __ZN4FMOD11ChannelReal4stopEv [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::stop(void)")]
pub fn stub_71950_wdog121() -> ! {
    todo!("0x71950 FMOD::ChannelReal::stop(void)")
}

// 0x7197c — __ZN4FMOD11ChannelReal9setPausedEb [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, bool)
#[doc(alias = "FMOD::ChannelReal::setPaused(bool)")]
pub fn stub_7197c_wdog122() -> ! {
    todo!("0x7197c FMOD::ChannelReal::setPaused(bool)")
}

// 0x719a0 — __ZN4FMOD11ChannelReal9getPausedEPb [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, bool *)
#[doc(alias = "FMOD::ChannelReal::getPaused(bool *)")]
pub fn stub_719a0_wdog123() -> ! {
    todo!("0x719a0 FMOD::ChannelReal::getPaused(bool *)")
}

// 0x719c0 — __ZN4FMOD11ChannelReal9setVolumeEf [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, float)
#[doc(alias = "FMOD::ChannelReal::setVolume(float)")]
pub fn stub_719c0_wdog124() -> ! {
    todo!("0x719c0 FMOD::ChannelReal::setVolume(float)")
}

// 0x719c8 — __ZN4FMOD11ChannelReal12setFrequencyEf [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, float)
#[doc(alias = "FMOD::ChannelReal::setFrequency(float)")]
pub fn stub_719c8_wdog125() -> ! {
    todo!("0x719c8 FMOD::ChannelReal::setFrequency(float)")
}

// 0x719d0 — __ZN4FMOD11ChannelReal6setPanEff [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, float, float)
#[doc(alias = "FMOD::ChannelReal::setPan(float,float)")]
pub fn stub_719d0_wdog126() -> ! {
    todo!("0x719d0 FMOD::ChannelReal::setPan(float,float)")
}

// 0x719d8 — __ZN4FMOD11ChannelReal16setDSPClockDelayEv [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::setDSPClockDelay(void)")]
pub fn stub_719d8_wdog127() -> ! {
    todo!("0x719d8 FMOD::ChannelReal::setDSPClockDelay(void)")
}

// 0x719e0 — __ZN4FMOD11ChannelReal13setSpeakerMixEffffffff [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t, float32_t)
#[doc(alias = "FMOD::ChannelReal::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_719e0_wdog128() -> ! {
    todo!("0x719e0 FMOD::ChannelReal::setSpeakerMix(float,float,float,float,float,float,float,float)")
}

// 0x71e34 — __ZN4FMOD11ChannelReal11setPositionEjj [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelReal::setPosition(unsigned int,unsigned int)")]
pub fn stub_71e34_wdog129() -> ! {
    todo!("0x71e34 FMOD::ChannelReal::setPosition(unsigned int,unsigned int)")
}

// 0x72008 — __ZN4FMOD11ChannelReal11getPositionEPjj [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelReal::getPosition(unsigned int *,unsigned int)")]
pub fn stub_72008_wdog130() -> ! {
    todo!("0x72008 FMOD::ChannelReal::getPosition(unsigned int *,unsigned int)")
}

// 0x722f0 — __ZN4FMOD11ChannelReal13setLoopPointsEjj [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelReal::setLoopPoints(unsigned int,unsigned int)")]
pub fn stub_722f0_wdog131() -> ! {
    todo!("0x722f0 FMOD::ChannelReal::setLoopPoints(unsigned int,unsigned int)")
}

// 0x72328 — __ZN4FMOD11ChannelReal12setLoopCountEi [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, int)
#[doc(alias = "FMOD::ChannelReal::setLoopCount(int)")]
pub fn stub_72328_wdog132() -> ! {
    todo!("0x72328 FMOD::ChannelReal::setLoopCount(int)")
}

// 0x72334 — __ZN4FMOD11ChannelReal12getLoopCountEPi [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, int *)
#[doc(alias = "FMOD::ChannelReal::getLoopCount(int *)")]
pub fn stub_72334_wdog133() -> ! {
    todo!("0x72334 FMOD::ChannelReal::getLoopCount(int *)")
}

// 0x7234c — __ZN4FMOD11ChannelReal14setLowPassGainEf [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, float)
#[doc(alias = "FMOD::ChannelReal::setLowPassGain(float)")]
pub fn stub_7234c_wdog134() -> ! {
    todo!("0x7234c FMOD::ChannelReal::setLowPassGain(float)")
}

// 0x72354 — __ZN4FMOD11ChannelReal15set3DAttributesEv [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::set3DAttributes(void)")]
pub fn stub_72354_wdog135() -> ! {
    todo!("0x72354 FMOD::ChannelReal::set3DAttributes(void)")
}

// 0x7235c — __ZN4FMOD11ChannelReal19set3DMinMaxDistanceEv [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this)
#[doc(alias = "FMOD::ChannelReal::set3DMinMaxDistance(void)")]
pub fn stub_7235c_wdog136() -> ! {
    todo!("0x7235c FMOD::ChannelReal::set3DMinMaxDistance(void)")
}

// 0x72364 — __ZN4FMOD11ChannelReal14set3DOcclusionEff [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, float, float)
#[doc(alias = "FMOD::ChannelReal::set3DOcclusion(float,float)")]
pub fn stub_72364_wdog137() -> ! {
    todo!("0x72364 FMOD::ChannelReal::set3DOcclusion(float,float)")
}

// 0x72388 — __ZN4FMOD11ChannelReal9isPlayingEPbb [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, bool *, bool)
#[doc(alias = "FMOD::ChannelReal::isPlaying(bool *,bool)")]
pub fn stub_72388_wdog138() -> ! {
    todo!("0x72388 FMOD::ChannelReal::isPlaying(bool *,bool)")
}

// 0x723b0 — __ZN4FMOD11ChannelReal9isVirtualEPb [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, bool *)
#[doc(alias = "FMOD::ChannelReal::isVirtual(bool *)")]
pub fn stub_723b0_wdog139() -> ! {
    todo!("0x723b0 FMOD::ChannelReal::isVirtual(bool *)")
}

// 0x723c4 — __ZN4FMOD11ChannelReal11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW [watchdog]
// type: int()
#[doc(alias = "FMOD::ChannelReal::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
pub fn stub_723c4_wdog140() -> ! {
    todo!("0x723c4 FMOD::ChannelReal::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")
}

// 0x723cc — __ZN4FMOD11ChannelReal11getWaveDataEPfii [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, float *, int, int)
#[doc(alias = "FMOD::ChannelReal::getWaveData(float *,int,int)")]
pub fn stub_723cc_wdog141() -> ! {
    todo!("0x723cc FMOD::ChannelReal::getWaveData(float *,int,int)")
}

// 0x723d4 — __ZN4FMOD11ChannelReal10getDSPHeadEPPNS_4DSPIE [watchdog]
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelReal::getDSPHead(FMOD::DSPI **)")]
pub fn stub_723d4_wdog142() -> ! {
    todo!("0x723d4 FMOD::ChannelReal::getDSPHead(FMOD::DSPI **)")
}

// 0x723e4 — __ZN4FMOD11ChannelReal7setModeEj [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, int)
#[doc(alias = "FMOD::ChannelReal::setMode(unsigned int)")]
pub fn stub_723e4_wdog143() -> ! {
    todo!("0x723e4 FMOD::ChannelReal::setMode(unsigned int)")
}

// 0x72528 — __ZN4FMOD11ChannelReal19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES [watchdog]
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelReal::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_72528_wdog144() -> ! {
    todo!("0x72528 FMOD::ChannelReal::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")
}

// 0x725a0 — __ZN4FMOD11ChannelReal19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES [watchdog]
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelReal::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_725a0_wdog145() -> ! {
    todo!("0x725a0 FMOD::ChannelReal::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")
}

// 0x726d8 — __ZN4FMOD11ChannelReal19updateSpeakerLevelsEf [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, float32_t)
#[doc(alias = "FMOD::ChannelReal::updateSpeakerLevels(float)")]
pub fn stub_726d8_wdog146() -> ! {
    todo!("0x726d8 FMOD::ChannelReal::updateSpeakerLevels(float)")
}

// 0x72910 — __ZN4FMOD11ChannelReal16setSpeakerLevelsEiPfi [watchdog]
// type: int __fastcall(FMOD::ChannelReal *this, int, float *, int)
#[doc(alias = "FMOD::ChannelReal::setSpeakerLevels(int,float *,int)")]
pub fn stub_72910_wdog147() -> ! {
    todo!("0x72910 FMOD::ChannelReal::setSpeakerLevels(int,float *,int)")
}

// 0x72a04 — __ZN4FMOD11ChannelRealD0Ev [watchdog]
// type: void __fastcall(FMOD::ChannelReal *__hidden this)
#[doc(alias = "FMOD::ChannelReal::~ChannelReal()")]
pub fn stub_72a04_wdog148() -> ! {
    todo!("0x72a04 FMOD::ChannelReal::~ChannelReal()")
}

// 0x72a28 — __ZN4FMOD11ChannelRealD1Ev [watchdog]
// type: void __fastcall(FMOD::ChannelReal *__hidden this)
#[doc(alias = "FMOD::ChannelReal::~ChannelReal()")]
pub fn stub_72a28_wdog149() -> ! {
    todo!("0x72a28 FMOD::ChannelReal::~ChannelReal()")
}


 // watchdog audio — next 150 duplicate stubs for FMOD/Sound/Audio (EA-sorted 0x72a40..0x7df78) using rbx_core::SharedPtr
// 0x72a40 — __ZN4FMOD19ChannelRealManual3D5allocEv [watchdog]
// type: int __fastcall(FMOD::ChannelRealManual3D *this)
#[doc(alias = "FMOD::ChannelRealManual3D::alloc(void)")]
pub fn stub_72a40_wdog150() -> ! {
    todo!("0x72a40 FMOD::ChannelRealManual3D::alloc(void)")
}

// 0x72a58 — __ZN4FMOD19ChannelRealManual3DC2Ev [watchdog]
// type: _DWORD *__fastcall(FMOD::ChannelRealManual3D *this)
#[doc(alias = "FMOD::ChannelRealManual3D::ChannelRealManual3D(void)")]
pub fn stub_72a58_wdog151() -> ! {
    todo!("0x72a58 FMOD::ChannelRealManual3D::ChannelRealManual3D(void)")
}

// 0x72a88 — __ZN4FMOD19ChannelRealManual3D23set2DFreqVolumePanFor3DEv [watchdog]
// type: int __fastcall(FMOD::ChannelRealManual3D *this)
#[doc(alias = "FMOD::ChannelRealManual3D::set2DFreqVolumePanFor3D(void)")]
pub fn stub_72a88_wdog152() -> ! {
    todo!("0x72a88 FMOD::ChannelRealManual3D::set2DFreqVolumePanFor3D(void)")
}

// 0x73de4 — __ZN4FMOD19ChannelRealManual3DD0Ev [watchdog]
// type: void __fastcall(FMOD::ChannelRealManual3D *__hidden this)
#[doc(alias = "FMOD::ChannelRealManual3D::~ChannelRealManual3D()")]
pub fn stub_73de4_wdog153() -> ! {
    todo!("0x73de4 FMOD::ChannelRealManual3D::~ChannelRealManual3D()")
}

// 0x73e08 — __ZN4FMOD19ChannelRealManual3DD1Ev [watchdog]
// type: void __fastcall(FMOD::ChannelRealManual3D *__hidden this)
#[doc(alias = "FMOD::ChannelRealManual3D::~ChannelRealManual3D()")]
pub fn stub_73e08_wdog154() -> ! {
    todo!("0x73e08 FMOD::ChannelRealManual3D::~ChannelRealManual3D()")
}

// 0x73e20 — __ZN4FMOD15ChannelSoftware14setLowPassGainEf [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, float)
#[doc(alias = "FMOD::ChannelSoftware::setLowPassGain(float)")]
pub fn stub_73e20_wdog155() -> ! {
    todo!("0x73e20 FMOD::ChannelSoftware::setLowPassGain(float)")
}

// 0x73e34 — __ZN4FMOD15ChannelSoftware16setDSPClockDelayEv [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::setDSPClockDelay(void)")]
pub fn stub_73e34_wdog156() -> ! {
    todo!("0x73e34 FMOD::ChannelSoftware::setDSPClockDelay(void)")
}

// 0x73f0c — __ZN4FMOD15ChannelSoftware11setPositionEjj [watchdog]
// type: int __fastcall(unsigned __int64 this, unsigned int)
#[doc(alias = "FMOD::ChannelSoftware::setPosition(unsigned int,unsigned int)")]
pub fn stub_73f0c_wdog157() -> ! {
    todo!("0x73f0c FMOD::ChannelSoftware::setPosition(unsigned int,unsigned int)")
}

// 0x741f4 — __ZN4FMOD15ChannelSoftware11getPositionEPjj [watchdog]
// type: int __fastcall(int this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelSoftware::getPosition(unsigned int *,unsigned int)")]
pub fn stub_741f4_wdog158() -> ! {
    todo!("0x741f4 FMOD::ChannelSoftware::getPosition(unsigned int *,unsigned int)")
}

// 0x74554 — __ZN4FMOD15ChannelSoftware10getDSPHeadEPPNS_4DSPIE [watchdog]
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelSoftware::getDSPHead(FMOD::DSPI **)")]
pub fn stub_74554_wdog159() -> ! {
    todo!("0x74554 FMOD::ChannelSoftware::getDSPHead(FMOD::DSPI **)")
}

// 0x74564 — __ZN4FMOD15ChannelSoftware16moveChannelGroupEPNS_13ChannelGroupIES2_b [watchdog]
// type: FMOD::DSPI *__fastcall(FMOD::DSPI **this, FMOD::DSPI **, FMOD::DSPI **, bool)
#[doc(alias = "FMOD::ChannelSoftware::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")]
pub fn stub_74564_wdog160() -> ! {
    todo!("0x74564 FMOD::ChannelSoftware::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")
}

// 0x745d4 — __ZN4FMOD15ChannelSoftware19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES [watchdog]
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelSoftware::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_745d4_wdog161() -> ! {
    todo!("0x745d4 FMOD::ChannelSoftware::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")
}

// 0x7464c — __ZN4FMOD15ChannelSoftware12addToReverbsEPNS_4DSPIE [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::addToReverbs(FMOD::DSPI *)")]
pub fn stub_7464c_wdog162() -> ! {
    todo!("0x7464c FMOD::ChannelSoftware::addToReverbs(FMOD::DSPI *)")
}

// 0x748b4 — __ZN4FMOD15ChannelSoftware11getWaveDataEPfii [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, float *, int, int)
#[doc(alias = "FMOD::ChannelSoftware::getWaveData(float *,int,int)")]
pub fn stub_748b4_wdog163() -> ! {
    todo!("0x748b4 FMOD::ChannelSoftware::getWaveData(float *,int,int)")
}

// 0x749c4 — __ZN4FMOD15ChannelSoftware11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW [watchdog]
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "FMOD::ChannelSoftware::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
pub fn stub_749c4_wdog164() -> ! {
    todo!("0x749c4 FMOD::ChannelSoftware::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")
}

// 0x74b20 — __ZN4FMOD15ChannelSoftware9isPlayingEPbb [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, bool *, bool)
#[doc(alias = "FMOD::ChannelSoftware::isPlaying(bool *,bool)")]
pub fn stub_74b20_wdog165() -> ! {
    todo!("0x74b20 FMOD::ChannelSoftware::isPlaying(bool *,bool)")
}

// 0x74bd0 — __ZN4FMOD15ChannelSoftware7setModeEj [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, int)
#[doc(alias = "FMOD::ChannelSoftware::setMode(unsigned int)")]
pub fn stub_74bd0_wdog166() -> ! {
    todo!("0x74bd0 FMOD::ChannelSoftware::setMode(unsigned int)")
}

// 0x74c04 — __ZN4FMOD15ChannelSoftware12getLoopCountEPi [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, int *)
#[doc(alias = "FMOD::ChannelSoftware::getLoopCount(int *)")]
pub fn stub_74c04_wdog167() -> ! {
    todo!("0x74c04 FMOD::ChannelSoftware::getLoopCount(int *)")
}

// 0x74c44 — __ZN4FMOD15ChannelSoftware12setLoopCountEi [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, int)
#[doc(alias = "FMOD::ChannelSoftware::setLoopCount(int)")]
pub fn stub_74c44_wdog168() -> ! {
    todo!("0x74c44 FMOD::ChannelSoftware::setLoopCount(int)")
}

// 0x74c90 — __ZN4FMOD15ChannelSoftware13setLoopPointsEjj [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelSoftware::setLoopPoints(unsigned int,unsigned int)")]
pub fn stub_74c90_wdog169() -> ! {
    todo!("0x74c90 FMOD::ChannelSoftware::setLoopPoints(unsigned int,unsigned int)")
}

// 0x74cd8 — __ZN4FMOD15ChannelSoftware6setPanEff [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, float32_t, float)
#[doc(alias = "FMOD::ChannelSoftware::setPan(float,float)")]
pub fn stub_74cd8_wdog170() -> ! {
    todo!("0x74cd8 FMOD::ChannelSoftware::setPan(float,float)")
}

// 0x74de8 — __ZN4FMOD15ChannelSoftware12setFrequencyEf [watchdog]
// type: FMOD::DSPWaveTable *__fastcall(FMOD::ChannelSoftware *this, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::setFrequency(float)")]
pub fn stub_74de8_wdog171() -> ! {
    todo!("0x74de8 FMOD::ChannelSoftware::setFrequency(float)")
}

// 0x74edc — __ZN4FMOD15ChannelSoftware15updateReverbMixEPNS_7ReverbIEf [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, FMOD::ReverbI *, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::updateReverbMix(FMOD::ReverbI *,float)")]
pub fn stub_74edc_wdog172() -> ! {
    todo!("0x74edc FMOD::ChannelSoftware::updateReverbMix(FMOD::ReverbI *,float)")
}

// 0x751dc — __ZN4FMOD15ChannelSoftware15updateDirectMixEf [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::updateDirectMix(float)")]
pub fn stub_751dc_wdog173() -> ! {
    todo!("0x751dc FMOD::ChannelSoftware::updateDirectMix(float)")
}

// 0x75408 — __ZN4FMOD15ChannelSoftware13setupDSPCodecEPNS_4DSPIE [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::setupDSPCodec(FMOD::DSPI *)")]
pub fn stub_75408_wdog174() -> ! {
    todo!("0x75408 FMOD::ChannelSoftware::setupDSPCodec(FMOD::DSPI *)")
}

// 0x75738 — __ZN4FMOD15ChannelSoftware5closeEv [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::close(void)")]
pub fn stub_75738_wdog175() -> ! {
    todo!("0x75738 FMOD::ChannelSoftware::close(void)")
}

// 0x757fc — __ZN4FMOD15ChannelSoftware4initEiPNS_7SystemIEPNS_6OutputEPNS_4DSPIE [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, int, FMOD::SystemI *, FMOD::Output *, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")]
pub fn stub_757fc_wdog176() -> ! {
    todo!("0x757fc FMOD::ChannelSoftware::init(int,FMOD::SystemI *,FMOD::Output *,FMOD::DSPI *)")
}

// 0x759c0 — __ZN4FMOD15ChannelSoftwareC2Ev [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::ChannelSoftware(void)")]
pub fn stub_759c0_wdog177() -> ! {
    todo!("0x759c0 FMOD::ChannelSoftware::ChannelSoftware(void)")
}

// 0x75a44 — __ZN4FMOD15ChannelSoftwareC1Ev [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::ChannelSoftware(void)")]
pub fn stub_75a44_wdog178() -> ! {
    todo!("0x75a44 FMOD::ChannelSoftware::ChannelSoftware(void)")
}

// 0x75a48 — __ZN4FMOD15ChannelSoftware9setPausedEb [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, bool)
#[doc(alias = "FMOD::ChannelSoftware::setPaused(bool)")]
pub fn stub_75a48_wdog179() -> ! {
    todo!("0x75a48 FMOD::ChannelSoftware::setPaused(bool)")
}

// 0x75b50 — __ZN4FMOD15ChannelSoftware5startEv [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::start(void)")]
pub fn stub_75b50_wdog180() -> ! {
    todo!("0x75b50 FMOD::ChannelSoftware::start(void)")
}

// 0x75be0 — __ZN4FMOD15ChannelSoftware5allocEv [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::alloc(void)")]
pub fn stub_75be0_wdog181() -> ! {
    todo!("0x75be0 FMOD::ChannelSoftware::alloc(void)")
}

// 0x75f8c — __ZN4FMOD15ChannelSoftware4stopEv [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this)
#[doc(alias = "FMOD::ChannelSoftware::stop(void)")]
pub fn stub_75f8c_wdog182() -> ! {
    todo!("0x75f8c FMOD::ChannelSoftware::stop(void)")
}

// 0x762c4 — __ZN4FMOD15ChannelSoftware16setSpeakerLevelsEiPfi [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, int, float *, int)
#[doc(alias = "FMOD::ChannelSoftware::setSpeakerLevels(int,float *,int)")]
pub fn stub_762c4_wdog183() -> ! {
    todo!("0x762c4 FMOD::ChannelSoftware::setSpeakerLevels(int,float *,int)")
}

// 0x76584 — __ZN4FMOD15ChannelSoftware13setSpeakerMixEffffffff [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, int, int, int, int, float, float, float, float)
#[doc(alias = "FMOD::ChannelSoftware::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_76584_wdog184() -> ! {
    todo!("0x76584 FMOD::ChannelSoftware::setSpeakerMix(float,float,float,float,float,float,float,float)")
}

// 0x76988 — __ZN4FMOD15ChannelSoftware9setVolumeEf [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, float32_t)
#[doc(alias = "FMOD::ChannelSoftware::setVolume(float)")]
pub fn stub_76988_wdog185() -> ! {
    todo!("0x76988 FMOD::ChannelSoftware::setVolume(float)")
}

// 0x76a80 — __ZN4FMOD15ChannelSoftware14set3DOcclusionEff [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, float, float)
#[doc(alias = "FMOD::ChannelSoftware::set3DOcclusion(float,float)")]
pub fn stub_76a80_wdog186() -> ! {
    todo!("0x76a80 FMOD::ChannelSoftware::set3DOcclusion(float,float)")
}

// 0x76b3c — __ZN4FMOD15ChannelSoftware19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, int *)
#[doc(alias = "FMOD::ChannelSoftware::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_76b3c_wdog187() -> ! {
    todo!("0x76b3c FMOD::ChannelSoftware::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")
}

// 0x7709c — __ZN4FMOD15ChannelSoftware9getPausedEPb [watchdog]
// type: int __fastcall(FMOD::ChannelSoftware *this, bool *)
#[doc(alias = "FMOD::ChannelSoftware::getPaused(bool *)")]
pub fn stub_7709c_wdog188() -> ! {
    todo!("0x7709c FMOD::ChannelSoftware::getPaused(bool *)")
}

// 0x77138 — __ZN4FMOD15ChannelSoftware5allocEPNS_4DSPIE [watchdog]
// type: int __fastcall(FMOD::DSPI **this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelSoftware::alloc(FMOD::DSPI *)")]
pub fn stub_77138_wdog189() -> ! {
    todo!("0x77138 FMOD::ChannelSoftware::alloc(FMOD::DSPI *)")
}

// 0x773c4 — __ZN4FMOD15ChannelSoftwareD1Ev [watchdog]
// type: void __fastcall(FMOD::ChannelSoftware *__hidden this)
#[doc(alias = "FMOD::ChannelSoftware::~ChannelSoftware()")]
pub fn stub_773c4_wdog190() -> ! {
    todo!("0x773c4 FMOD::ChannelSoftware::~ChannelSoftware()")
}

// 0x773f0 — __ZN4FMOD15ChannelSoftwareD0Ev [watchdog]
// type: void __fastcall(FMOD::ChannelSoftware *__hidden this)
#[doc(alias = "FMOD::ChannelSoftware::~ChannelSoftware()")]
pub fn stub_773f0_wdog191() -> ! {
    todo!("0x773f0 FMOD::ChannelSoftware::~ChannelSoftware()")
}

// 0x77428 — __ZN4FMOD13ChannelStream23set2DFreqVolumePanFor3DEv [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::set2DFreqVolumePanFor3D(void)")]
pub fn stub_77428_wdog192() -> ! {
    todo!("0x77428 FMOD::ChannelStream::set2DFreqVolumePanFor3D(void)")
}

// 0x77474 — __ZN4FMOD13ChannelStream16moveChannelGroupEPNS_13ChannelGroupIES2_b [watchdog]
// type: int __fastcall(int, int, int, unsigned __int8)
#[doc(alias = "FMOD::ChannelStream::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")]
pub fn stub_77474_wdog193() -> ! {
    todo!("0x77474 FMOD::ChannelStream::moveChannelGroup(FMOD::ChannelGroupI *,FMOD::ChannelGroupI *,bool)")
}

// 0x774e0 — __ZN4FMOD13ChannelStream5startEv [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::start(void)")]
pub fn stub_774e0_wdog194() -> ! {
    todo!("0x774e0 FMOD::ChannelStream::start(void)")
}

// 0x77574 — __ZN4FMOD13ChannelStream6updateEi [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, int)
#[doc(alias = "FMOD::ChannelStream::update(int)")]
pub fn stub_77574_wdog195() -> ! {
    todo!("0x77574 FMOD::ChannelStream::update(int)")
}

// 0x775d0 — __ZN4FMOD13ChannelStream9setVolumeEf [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, float)
#[doc(alias = "FMOD::ChannelStream::setVolume(float)")]
pub fn stub_775d0_wdog196() -> ! {
    todo!("0x775d0 FMOD::ChannelStream::setVolume(float)")
}

// 0x77718 — __ZN4FMOD13ChannelStream12setFrequencyEf [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, float)
#[doc(alias = "FMOD::ChannelStream::setFrequency(float)")]
pub fn stub_77718_wdog197() -> ! {
    todo!("0x77718 FMOD::ChannelStream::setFrequency(float)")
}

// 0x77774 — __ZN4FMOD13ChannelStream6setPanEff [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, float, float)
#[doc(alias = "FMOD::ChannelStream::setPan(float,float)")]
pub fn stub_77774_wdog198() -> ! {
    todo!("0x77774 FMOD::ChannelStream::setPan(float,float)")
}

// 0x7781c — __ZN4FMOD13ChannelStream16setDSPClockDelayEv [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::setDSPClockDelay(void)")]
pub fn stub_7781c_wdog199() -> ! {
    todo!("0x7781c FMOD::ChannelStream::setDSPClockDelay(void)")
}

// 0x77868 — __ZN4FMOD13ChannelStream13setSpeakerMixEffffffff [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, float, float, float, float, float, float, float, float)
#[doc(alias = "FMOD::ChannelStream::setSpeakerMix(float,float,float,float,float,float,float,float)")]
pub fn stub_77868_wdog200() -> ! {
    todo!("0x77868 FMOD::ChannelStream::setSpeakerMix(float,float,float,float,float,float,float,float)")
}

// 0x77904 — __ZN4FMOD13ChannelStream16setSpeakerLevelsEiPfi [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, int, float *, int)
#[doc(alias = "FMOD::ChannelStream::setSpeakerLevels(int,float *,int)")]
pub fn stub_77904_wdog201() -> ! {
    todo!("0x77904 FMOD::ChannelStream::setSpeakerLevels(int,float *,int)")
}

// 0x77970 — __ZN4FMOD13ChannelStream15set3DAttributesEv [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::set3DAttributes(void)")]
pub fn stub_77970_wdog202() -> ! {
    todo!("0x77970 FMOD::ChannelStream::set3DAttributes(void)")
}

// 0x779bc — __ZN4FMOD13ChannelStream14setLowPassGainEf [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, float)
#[doc(alias = "FMOD::ChannelStream::setLowPassGain(float)")]
pub fn stub_779bc_wdog203() -> ! {
    todo!("0x779bc FMOD::ChannelStream::setLowPassGain(float)")
}

// 0x77a18 — __ZN4FMOD13ChannelStream19set3DMinMaxDistanceEv [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::set3DMinMaxDistance(void)")]
pub fn stub_77a18_wdog204() -> ! {
    todo!("0x77a18 FMOD::ChannelStream::set3DMinMaxDistance(void)")
}

// 0x77a64 — __ZN4FMOD13ChannelStream14set3DOcclusionEff [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, float, float)
#[doc(alias = "FMOD::ChannelStream::set3DOcclusion(float,float)")]
pub fn stub_77a64_wdog205() -> ! {
    todo!("0x77a64 FMOD::ChannelStream::set3DOcclusion(float,float)")
}

// 0x77ac8 — __ZN4FMOD13ChannelStream19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES [watchdog]
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelStream::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_77ac8_wdog206() -> ! {
    todo!("0x77ac8 FMOD::ChannelStream::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")
}

// 0x77b24 — __ZN4FMOD13ChannelStream19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES [watchdog]
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelStream::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_77b24_wdog207() -> ! {
    todo!("0x77b24 FMOD::ChannelStream::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")
}

// 0x77b48 — __ZN4FMOD13ChannelStream9isPlayingEPbb [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, bool *, bool)
#[doc(alias = "FMOD::ChannelStream::isPlaying(bool *,bool)")]
pub fn stub_77b48_wdog208() -> ! {
    todo!("0x77b48 FMOD::ChannelStream::isPlaying(bool *,bool)")
}

// 0x77b5c — __ZN4FMOD13ChannelStream11getSpectrumEPfii19FMOD_DSP_FFT_WINDOW [watchdog]
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelStream::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")]
pub fn stub_77b5c_wdog209() -> ! {
    todo!("0x77b5c FMOD::ChannelStream::getSpectrum(float *,int,int,FMOD_DSP_FFT_WINDOW)")
}

// 0x77b6c — __ZN4FMOD13ChannelStream11getWaveDataEPfii [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, float *, int, int)
#[doc(alias = "FMOD::ChannelStream::getWaveData(float *,int,int)")]
pub fn stub_77b6c_wdog210() -> ! {
    todo!("0x77b6c FMOD::ChannelStream::getWaveData(float *,int,int)")
}

// 0x77b7c — __ZN4FMOD13ChannelStream10getDSPHeadEPPNS_4DSPIE [watchdog]
// type: int __fastcall(int)
#[doc(alias = "FMOD::ChannelStream::getDSPHead(FMOD::DSPI **)")]
pub fn stub_77b7c_wdog211() -> ! {
    todo!("0x77b7c FMOD::ChannelStream::getDSPHead(FMOD::DSPI **)")
}

// 0x77b8c — __ZN4FMOD13ChannelStream12setLoopCountEi [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, int)
#[doc(alias = "FMOD::ChannelStream::setLoopCount(int)")]
pub fn stub_77b8c_wdog212() -> ! {
    todo!("0x77b8c FMOD::ChannelStream::setLoopCount(int)")
}

// 0x77bc0 — __ZN4FMOD13ChannelStream13setLoopPointsEjj [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelStream::setLoopPoints(unsigned int,unsigned int)")]
pub fn stub_77bc0_wdog213() -> ! {
    todo!("0x77bc0 FMOD::ChannelStream::setLoopPoints(unsigned int,unsigned int)")
}

// 0x77c14 — __ZN4FMOD13ChannelStream11getPositionEPjj [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelStream::getPosition(unsigned int *,unsigned int)")]
pub fn stub_77c14_wdog214() -> ! {
    todo!("0x77c14 FMOD::ChannelStream::getPosition(unsigned int *,unsigned int)")
}

// 0x77f74 — __ZN4FMOD13ChannelStream4stopEv [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::stop(void)")]
pub fn stub_77f74_wdog215() -> ! {
    todo!("0x77f74 FMOD::ChannelStream::stop(void)")
}

// 0x78168 — __ZN4FMOD13ChannelStream7setModeEj [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, int)
#[doc(alias = "FMOD::ChannelStream::setMode(unsigned int)")]
pub fn stub_78168_wdog216() -> ! {
    todo!("0x78168 FMOD::ChannelStream::setMode(unsigned int)")
}

// 0x781f0 — __ZN4FMOD13ChannelStreamC2Ev [watchdog]
// type: _DWORD *__fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::ChannelStream(void)")]
pub fn stub_781f0_wdog217() -> ! {
    todo!("0x781f0 FMOD::ChannelStream::ChannelStream(void)")
}

// 0x7826c — __ZN4FMOD13ChannelStreamC1Ev [watchdog]
// type: _DWORD *__fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::ChannelStream(void)")]
pub fn stub_7826c_wdog218() -> ! {
    todo!("0x7826c FMOD::ChannelStream::ChannelStream(void)")
}

// 0x78270 — __ZN4FMOD13ChannelStream5allocEv [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, int, int)
#[doc(alias = "FMOD::ChannelStream::alloc(void)")]
pub fn stub_78270_wdog219() -> ! {
    todo!("0x78270 FMOD::ChannelStream::alloc(void)")
}

// 0x78540 — __ZN4FMOD13ChannelStream13setPositionExEjjb [watchdog]
// type: int __fastcall(unsigned __int64 this, unsigned int, bool)
#[doc(alias = "FMOD::ChannelStream::setPositionEx(unsigned int,unsigned int,bool)")]
pub fn stub_78540_wdog220() -> ! {
    todo!("0x78540 FMOD::ChannelStream::setPositionEx(unsigned int,unsigned int,bool)")
}

// 0x78af0 — __ZN4FMOD13ChannelStream9setPausedEb [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, bool)
#[doc(alias = "FMOD::ChannelStream::setPaused(bool)")]
pub fn stub_78af0_wdog221() -> ! {
    todo!("0x78af0 FMOD::ChannelStream::setPaused(bool)")
}

// 0x78b80 — __ZN4FMOD13ChannelStream12updateStreamEv [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::updateStream(void)")]
pub fn stub_78b80_wdog222() -> ! {
    todo!("0x78b80 FMOD::ChannelStream::updateStream(void)")
}

// 0x78fac — __ZN4FMOD13ChannelStream8isStreamEv [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this)
#[doc(alias = "FMOD::ChannelStream::isStream(void)")]
pub fn stub_78fac_wdog223() -> ! {
    todo!("0x78fac FMOD::ChannelStream::isStream(void)")
}

// 0x78fb4 — __ZN4FMOD13ChannelStream11setPositionEjj [watchdog]
// type: int __fastcall(FMOD::ChannelStream *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelStream::setPosition(unsigned int,unsigned int)")]
pub fn stub_78fb4_wdog224() -> ! {
    todo!("0x78fb4 FMOD::ChannelStream::setPosition(unsigned int,unsigned int)")
}

// 0x78fc4 — __ZN4FMOD13ChannelStreamD0Ev [watchdog]
// type: void __fastcall(FMOD::ChannelStream *__hidden this)
#[doc(alias = "FMOD::ChannelStream::~ChannelStream()")]
pub fn stub_78fc4_wdog225() -> ! {
    todo!("0x78fc4 FMOD::ChannelStream::~ChannelStream()")
}

// 0x78fe8 — __ZN4FMOD13ChannelStreamD1Ev [watchdog]
// type: void __fastcall(FMOD::ChannelStream *__hidden this)
#[doc(alias = "FMOD::ChannelStream::~ChannelStream()")]
pub fn stub_78fe8_wdog226() -> ! {
    todo!("0x78fe8 FMOD::ChannelStream::~ChannelStream()")
}

// 0x79000 — __ZN4FMOD12ChannelGroup9setVolumeEf [watchdog]
// type: int __fastcall(FMOD::ChannelGroup *this, float, FMOD::ChannelGroupI **)
#[doc(alias = "FMOD::ChannelGroup::setVolume(float)")]
pub fn stub_79000_wdog227() -> ! {
    todo!("0x79000 FMOD::ChannelGroup::setVolume(float)")
}

// 0x79034 — __ZN4FMOD13ChannelGroupI8validateEPNS_12ChannelGroupEPPS0_ [watchdog]
// type: int __fastcall(int result, int *)
#[doc(alias = "FMOD::ChannelGroupI::validate(FMOD::ChannelGroup *,FMOD::ChannelGroupI**)")]
pub fn stub_79034_wdog228() -> ! {
    todo!("0x79034 FMOD::ChannelGroupI::validate(FMOD::ChannelGroup *,FMOD::ChannelGroupI**)")
}

// 0x79054 — __ZN4FMOD13ChannelGroupI9getPausedEPb [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this, bool *)
#[doc(alias = "FMOD::ChannelGroupI::getPaused(bool *)")]
pub fn stub_79054_wdog229() -> ! {
    todo!("0x79054 FMOD::ChannelGroupI::getPaused(bool *)")
}

// 0x7906c — __ZN4FMOD13ChannelGroupI17getMemoryUsedImplEPNS_13MemoryTrackerE [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::ChannelGroupI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_7906c_wdog230() -> ! {
    todo!("0x7906c FMOD::ChannelGroupI::getMemoryUsedImpl(FMOD::MemoryTracker *)")
}

// 0x790fc — __ZN4FMOD13ChannelGroupI20updateChildMixTargetEPNS_4DSPIE [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::DSPI *)
#[doc(alias = "FMOD::ChannelGroupI::updateChildMixTarget(FMOD::DSPI *)")]
pub fn stub_790fc_wdog231() -> ! {
    todo!("0x790fc FMOD::ChannelGroupI::updateChildMixTarget(FMOD::DSPI *)")
}

// 0x791e8 — __ZN4FMOD13ChannelGroupI7setMuteEbb [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this, bool, bool)
#[doc(alias = "FMOD::ChannelGroupI::setMute(bool,bool)")]
pub fn stub_791e8_wdog232() -> ! {
    todo!("0x791e8 FMOD::ChannelGroupI::setMute(bool,bool)")
}

// 0x79280 — __ZN4FMOD13ChannelGroupI9setPausedEbb [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this, bool, bool)
#[doc(alias = "FMOD::ChannelGroupI::setPaused(bool,bool)")]
pub fn stub_79280_wdog233() -> ! {
    todo!("0x79280 FMOD::ChannelGroupI::setPaused(bool,bool)")
}

// 0x79334 — __ZN4FMOD13ChannelGroupI16setPitchInternalEv [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this)
#[doc(alias = "FMOD::ChannelGroupI::setPitchInternal(void)")]
pub fn stub_79334_wdog234() -> ! {
    todo!("0x79334 FMOD::ChannelGroupI::setPitchInternal(void)")
}

// 0x793e4 — __ZN4FMOD13ChannelGroupI17setVolumeInternalEv [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this)
#[doc(alias = "FMOD::ChannelGroupI::setVolumeInternal(void)")]
pub fn stub_793e4_wdog235() -> ! {
    todo!("0x793e4 FMOD::ChannelGroupI::setVolumeInternal(void)")
}

// 0x794c4 — __ZN4FMOD13ChannelGroupI8addGroupEPS0_ [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this, FMOD::ChannelGroupI *)
#[doc(alias = "FMOD::ChannelGroupI::addGroup(FMOD::ChannelGroupI*)")]
pub fn stub_794c4_wdog236() -> ! {
    todo!("0x794c4 FMOD::ChannelGroupI::addGroup(FMOD::ChannelGroupI*)")
}

// 0x796a4 — __ZN4FMOD13ChannelGroupI9setVolumeEf [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this, float)
#[doc(alias = "FMOD::ChannelGroupI::setVolume(float)")]
pub fn stub_796a4_wdog237() -> ! {
    todo!("0x796a4 FMOD::ChannelGroupI::setVolume(float)")
}

// 0x796d4 — __ZN4FMOD13ChannelGroupI15releaseInternalEb [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this, bool)
#[doc(alias = "FMOD::ChannelGroupI::releaseInternal(bool)")]
pub fn stub_796d4_wdog238() -> ! {
    todo!("0x796d4 FMOD::ChannelGroupI::releaseInternal(bool)")
}

// 0x7995c — __ZN4FMOD13ChannelGroupI7releaseEv [watchdog]
// type: int __fastcall(FMOD::ChannelGroupI *this)
#[doc(alias = "FMOD::ChannelGroupI::release(void)")]
pub fn stub_7995c_wdog239() -> ! {
    todo!("0x7995c FMOD::ChannelGroupI::release(void)")
}

// 0x79980 — __ZN4FMOD20ChannelGroupSoftware17getMemoryUsedImplEPNS_13MemoryTrackerE [watchdog]
// type: int __fastcall(FMOD::ChannelGroupSoftware *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::ChannelGroupSoftware::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_79980_wdog240() -> ! {
    todo!("0x79980 FMOD::ChannelGroupSoftware::getMemoryUsedImpl(FMOD::MemoryTracker *)")
}

// 0x79a38 — __ZN4FMOD13ChannelGroupI13getMemoryUsedEPNS_13MemoryTrackerE [watchdog]
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelGroupI::getMemoryUsed(FMOD::MemoryTracker *)")]
pub fn stub_79a38_wdog241() -> ! {
    todo!("0x79a38 FMOD::ChannelGroupI::getMemoryUsed(FMOD::MemoryTracker *)")
}

// 0x79a90 — __ZN4FMOD20ChannelGroupSoftware13getMemoryUsedEPNS_13MemoryTrackerE [watchdog]
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelGroupSoftware::getMemoryUsed(FMOD::MemoryTracker *)")]
pub fn stub_79a90_wdog242() -> ! {
    todo!("0x79a90 FMOD::ChannelGroupSoftware::getMemoryUsed(FMOD::MemoryTracker *)")
}

// 0x79ae8 — __ZN4FMOD8ChannelI16returnToFreeListEv [watchdog]
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::returnToFreeList(void)")]
pub fn stub_79ae8_wdog243() -> ! {
    todo!("0x79ae8 FMOD::ChannelI::returnToFreeList(void)")
}

// 0x79b98 — __ZN4FMOD8ChannelI14referenceStampEb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::referenceStamp(bool)")]
pub fn stub_79b98_wdog244() -> ! {
    todo!("0x79b98 FMOD::ChannelI::referenceStamp(bool)")
}

// 0x79bdc — __ZN4FMOD8ChannelI14getRealChannelEPPNS_11ChannelRealEPi [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelReal **, int *)
#[doc(alias = "FMOD::ChannelI::getRealChannel(FMOD::ChannelReal **,int *)")]
pub fn stub_79bdc_wdog245() -> ! {
    todo!("0x79bdc FMOD::ChannelI::getRealChannel(FMOD::ChannelReal **,int *)")
}

// 0x79ca8 — __ZN4FMOD8ChannelI4initEv [watchdog]
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::init(void)")]
pub fn stub_79ca8_wdog246() -> ! {
    todo!("0x79ca8 FMOD::ChannelI::init(void)")
}

// 0x79dd4 — __ZN4FMOD8ChannelIC2EiPNS_7SystemIE [watchdog]
// type: int __fastcall(FMOD::ChannelI *, int, int)
#[doc(alias = "FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")]
pub fn stub_79dd4_wdog247() -> ! {
    todo!("0x79dd4 FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")
}

// 0x79e84 — __ZN4FMOD8ChannelIC1EiPNS_7SystemIE [watchdog]
// type: int __fastcall(FMOD::ChannelI *, int, int)
#[doc(alias = "FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")]
pub fn stub_79e84_wdog248() -> ! {
    todo!("0x79e84 FMOD::ChannelI::ChannelI(int,FMOD::SystemI *)")
}

// 0x79e88 — __ZN4FMOD8ChannelIC2Ev [watchdog]
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::ChannelI(void)")]
pub fn stub_79e88_wdog249() -> ! {
    todo!("0x79e88 FMOD::ChannelI::ChannelI(void)")
}

// 0x79ef0 — __ZN4FMOD8ChannelIC1Ev [watchdog]
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::ChannelI(void)")]
pub fn stub_79ef0_wdog250() -> ! {
    todo!("0x79ef0 FMOD::ChannelI::ChannelI(void)")
}

// 0x79ef4 — __ZN4FMOD8ChannelI5allocEPNS_4DSPIEb [watchdog]
// type: int __fastcall(_DWORD *, int, char)
#[doc(alias = "FMOD::ChannelI::alloc(FMOD::DSPI *,bool)")]
pub fn stub_79ef4_wdog251() -> ! {
    todo!("0x79ef4 FMOD::ChannelI::alloc(FMOD::DSPI *,bool)")
}

// 0x7a0f8 — __ZN4FMOD8ChannelI5startEv [watchdog]
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::start(void)")]
pub fn stub_7a0f8_wdog252() -> ! {
    todo!("0x7a0f8 FMOD::ChannelI::start(void)")
}

// 0x7a198 — __ZN4FMOD8ChannelI9getPausedEPb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::getPaused(bool *)")]
pub fn stub_7a198_wdog253() -> ! {
    todo!("0x7a198 FMOD::ChannelI::getPaused(bool *)")
}

// 0x7a1ec — __ZN4FMOD8ChannelI9getVolumeEPf [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, float *)
#[doc(alias = "FMOD::ChannelI::getVolume(float *)")]
pub fn stub_7a1ec_wdog254() -> ! {
    todo!("0x7a1ec FMOD::ChannelI::getVolume(float *)")
}

// 0x7a214 — __ZN4FMOD8ChannelI12getFrequencyEPf [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, float *)
#[doc(alias = "FMOD::ChannelI::getFrequency(float *)")]
pub fn stub_7a214_wdog255() -> ! {
    todo!("0x7a214 FMOD::ChannelI::getFrequency(float *)")
}

// 0x7a23c — __ZN4FMOD8ChannelI6setPanEfb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, float, bool)
#[doc(alias = "FMOD::ChannelI::setPan(float,bool)")]
pub fn stub_7a23c_wdog256() -> ! {
    todo!("0x7a23c FMOD::ChannelI::setPan(float,bool)")
}

// 0x7a358 — __ZN4FMOD8ChannelI8setDelayE14FMOD_DELAYTYPEjj [watchdog]
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "FMOD::ChannelI::setDelay(FMOD_DELAYTYPE,unsigned int,unsigned int)")]
pub fn stub_7a358_wdog257() -> ! {
    todo!("0x7a358 FMOD::ChannelI::setDelay(FMOD_DELAYTYPE,unsigned int,unsigned int)")
}

// 0x7a50c — __ZN4FMOD8ChannelI13setSpeakerMixEffffffffb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, float, float, float, float, float, float, float, float, bool)
#[doc(alias = "FMOD::ChannelI::setSpeakerMix(float,float,float,float,float,float,float,float,bool)")]
pub fn stub_7a50c_wdog258() -> ! {
    todo!("0x7a50c FMOD::ChannelI::setSpeakerMix(float,float,float,float,float,float,float,float,bool)")
}

// 0x7a7dc — __ZN4FMOD8ChannelI16getSpeakerLevelsE12FMOD_SPEAKERPfi [watchdog]
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "FMOD::ChannelI::getSpeakerLevels(FMOD_SPEAKER,float *,int)")]
pub fn stub_7a7dc_wdog259() -> ! {
    todo!("0x7a7dc FMOD::ChannelI::getSpeakerLevels(FMOD_SPEAKER,float *,int)")
}

// 0x7a8b0 — __ZN4FMOD8ChannelI7getMuteEPb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::getMute(bool *)")]
pub fn stub_7a8b0_wdog260() -> ! {
    todo!("0x7a8b0 FMOD::ChannelI::getMute(bool *)")
}

// 0x7a8d8 — __ZN4FMOD8ChannelI15set3DAttributesEPK11FMOD_VECTORS3_ [watchdog]
// type: int __fastcall(int, float *, float *)
#[doc(alias = "FMOD::ChannelI::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")]
pub fn stub_7a8d8_wdog261() -> ! {
    todo!("0x7a8d8 FMOD::ChannelI::set3DAttributes(FMOD_VECTOR const*,FMOD_VECTOR const*)")
}

// 0x7aa4c — __ZN4FMOD8ChannelI19setReverbPropertiesEPK29FMOD_REVERB_CHANNELPROPERTIES [watchdog]
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")]
pub fn stub_7aa4c_wdog262() -> ! {
    todo!("0x7aa4c FMOD::ChannelI::setReverbProperties(FMOD_REVERB_CHANNELPROPERTIES const*)")
}

// 0x7aae0 — __ZN4FMOD8ChannelI19getReverbPropertiesEP29FMOD_REVERB_CHANNELPROPERTIES [watchdog]
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")]
pub fn stub_7aae0_wdog263() -> ! {
    todo!("0x7aae0 FMOD::ChannelI::getReverbProperties(FMOD_REVERB_CHANNELPROPERTIES *)")
}

// 0x7ab74 — __ZN4FMOD8ChannelI9isVirtualEPb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::isVirtual(bool *)")]
pub fn stub_7ab74_wdog264() -> ! {
    todo!("0x7ab74 FMOD::ChannelI::isVirtual(bool *)")
}

// 0x7aba0 — __ZN4FMOD8ChannelI21getAudibilityInternalEPfb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, float *, bool)
#[doc(alias = "FMOD::ChannelI::getAudibilityInternal(float *,bool)")]
pub fn stub_7aba0_wdog265() -> ! {
    todo!("0x7aba0 FMOD::ChannelI::getAudibilityInternal(float *,bool)")
}

// 0x7ad00 — __ZN4FMOD8ChannelI13getAudibilityEPf [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, float *)
#[doc(alias = "FMOD::ChannelI::getAudibility(float *)")]
pub fn stub_7ad00_wdog266() -> ! {
    todo!("0x7ad00 FMOD::ChannelI::getAudibility(float *)")
}

// 0x7ad08 — __ZN4FMOD8ChannelI15getCurrentSoundEPPNS_6SoundIE [watchdog]
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelI::getCurrentSound(FMOD::SoundI **)")]
pub fn stub_7ad08_wdog267() -> ! {
    todo!("0x7ad08 FMOD::ChannelI::getCurrentSound(FMOD::SoundI **)")
}

// 0x7ad44 — __ZN4FMOD8ChannelI13getCurrentDSPEPPNS_4DSPIE [watchdog]
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "FMOD::ChannelI::getCurrentDSP(FMOD::DSPI **)")]
pub fn stub_7ad44_wdog268() -> ! {
    todo!("0x7ad44 FMOD::ChannelI::getCurrentDSP(FMOD::DSPI **)")
}

// 0x7ad70 — __ZN4FMOD8ChannelI11setCallbackEPF11FMOD_RESULTP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS5_E [watchdog]
// type: int __fastcall(int result, int)
#[doc(alias = "FMOD::ChannelI::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")]
pub fn stub_7ad70_wdog269() -> ! {
    todo!("0x7ad70 FMOD::ChannelI::setCallback(FMOD_RESULT (*)(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *))")
}

// 0x7ad88 — __ZN4FMOD8ChannelI11getPositionEPjj [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelI::getPosition(unsigned int *,unsigned int)")]
pub fn stub_7ad88_wdog270() -> ! {
    todo!("0x7ad88 FMOD::ChannelI::getPosition(unsigned int *,unsigned int)")
}

// 0x7adb0 — __ZN4FMOD8ChannelI16updateSyncPointsEb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::updateSyncPoints(bool)")]
pub fn stub_7adb0_wdog271() -> ! {
    todo!("0x7adb0 FMOD::ChannelI::updateSyncPoints(bool)")
}

// 0x7b1f8 — __ZN4FMOD8ChannelI12setFrequencyEf [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, float)
#[doc(alias = "FMOD::ChannelI::setFrequency(float)")]
pub fn stub_7b1f8_wdog272() -> ! {
    todo!("0x7b1f8 FMOD::ChannelI::setFrequency(float)")
}

// 0x7b31c — __ZN4FMOD8ChannelI10getDSPHeadEPPNS_4DSPIE [watchdog]
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::getDSPHead(FMOD::DSPI **)")]
pub fn stub_7b31c_wdog273() -> ! {
    todo!("0x7b31c FMOD::ChannelI::getDSPHead(FMOD::DSPI **)")
}

// 0x7b344 — __ZN4FMOD8ChannelI7getModeEPj [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *)
#[doc(alias = "FMOD::ChannelI::getMode(unsigned int *)")]
pub fn stub_7b344_wdog274() -> ! {
    todo!("0x7b344 FMOD::ChannelI::getMode(unsigned int *)")
}

// 0x7b36c — __ZN4FMOD8ChannelI12setLoopCountEi [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, int)
#[doc(alias = "FMOD::ChannelI::setLoopCount(int)")]
pub fn stub_7b36c_wdog275() -> ! {
    todo!("0x7b36c FMOD::ChannelI::setLoopCount(int)")
}

// 0x7b40c — __ZN4FMOD8ChannelI12getLoopCountEPi [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, int *)
#[doc(alias = "FMOD::ChannelI::getLoopCount(int *)")]
pub fn stub_7b40c_wdog276() -> ! {
    todo!("0x7b40c FMOD::ChannelI::getLoopCount(int *)")
}

// 0x7b434 — __ZN4FMOD8ChannelI11setUserDataEPv [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, void *)
#[doc(alias = "FMOD::ChannelI::setUserData(void *)")]
pub fn stub_7b434_wdog277() -> ! {
    todo!("0x7b434 FMOD::ChannelI::setUserData(void *)")
}

// 0x7b440 — __ZN4FMOD8ChannelI11getUserDataEPPv [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, void **)
#[doc(alias = "FMOD::ChannelI::getUserData(void **)")]
pub fn stub_7b440_wdog278() -> ! {
    todo!("0x7b440 FMOD::ChannelI::getUserData(void **)")
}

// 0x7b458 — __ZN4FMOD8ChannelI17getMemoryUsedImplEPNS_13MemoryTrackerE [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::ChannelI::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_7b458_wdog279() -> ! {
    todo!("0x7b458 FMOD::ChannelI::getMemoryUsedImpl(FMOD::MemoryTracker *)")
}

// 0x7b47c — __ZN4FMOD8ChannelI6addDSPEPNS_4DSPIEPPNS_14DSPConnectionIE [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, FMOD::DSPI *, FMOD::DSPConnectionI **)
#[doc(alias = "FMOD::ChannelI::addDSP(FMOD::DSPI *,FMOD::DSPConnectionI **)")]
pub fn stub_7b47c_wdog280() -> ! {
    todo!("0x7b47c FMOD::ChannelI::addDSP(FMOD::DSPI *,FMOD::DSPConnectionI **)")
}

// 0x7b4e8 — __ZN4FMOD8ChannelI16setSpeakerLevelsE12FMOD_SPEAKERPfib [watchdog]
// type: int __fastcall(int, unsigned int, int, int, char)
#[doc(alias = "FMOD::ChannelI::setSpeakerLevels(FMOD_SPEAKER,float *,int,bool)")]
pub fn stub_7b4e8_wdog281() -> ! {
    todo!("0x7b4e8 FMOD::ChannelI::setSpeakerLevels(FMOD_SPEAKER,float *,int,bool)")
}

// 0x7b79c — __ZN4FMOD8ChannelI21calculate3DReverbGainEPNS_7ReverbIEP11FMOD_VECTORPf [watchdog]
// type: int __fastcall(int, int, int, __int32 *)
#[doc(alias = "FMOD::ChannelI::calculate3DReverbGain(FMOD::ReverbI *,FMOD_VECTOR *,float *)")]
pub fn stub_7b79c_wdog282() -> ! {
    todo!("0x7b79c FMOD::ChannelI::calculate3DReverbGain(FMOD::ReverbI *,FMOD_VECTOR *,float *)")
}

// 0x7b860 — __ZN4FMOD8ChannelI5allocEPNS_6SoundIEb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, FMOD::SoundI *, bool)
#[doc(alias = "FMOD::ChannelI::alloc(FMOD::SoundI *,bool)")]
pub fn stub_7b860_wdog283() -> ! {
    todo!("0x7b860 FMOD::ChannelI::alloc(FMOD::SoundI *,bool)")
}

// 0x7bbc4 — __ZN4FMOD8ChannelI23calcVolumeAndPitchFor3DEv [watchdog]
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::calcVolumeAndPitchFor3D(void)")]
pub fn stub_7bbc4_wdog284() -> ! {
    todo!("0x7bbc4 FMOD::ChannelI::calcVolumeAndPitchFor3D(void)")
}

// 0x7c164 — __ZN4FMOD8ChannelI8validateEPNS_7ChannelEPPS0_ [watchdog]
// type: int __fastcall(unsigned int, _DWORD *, FMOD::SystemI **)
#[doc(alias = "FMOD::ChannelI::validate(FMOD::Channel *,FMOD::ChannelI**)")]
pub fn stub_7c164_wdog285() -> ! {
    todo!("0x7c164 FMOD::ChannelI::validate(FMOD::Channel *,FMOD::ChannelI**)")
}

// 0x7c224 — __ZN4FMOD8ChannelI9isPlayingEPb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, bool *)
#[doc(alias = "FMOD::ChannelI::isPlaying(bool *)")]
pub fn stub_7c224_wdog286() -> ! {
    todo!("0x7c224 FMOD::ChannelI::isPlaying(bool *)")
}

// 0x7c3d8 — __ZN4FMOD8ChannelI13getLoopPointsEPjjS1_j [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, unsigned int *, unsigned int, unsigned int *, unsigned int)
#[doc(alias = "FMOD::ChannelI::getLoopPoints(unsigned int *,unsigned int,unsigned int *,unsigned int)")]
pub fn stub_7c3d8_wdog287() -> ! {
    todo!("0x7c3d8 FMOD::ChannelI::getLoopPoints(unsigned int *,unsigned int,unsigned int *,unsigned int)")
}

// 0x7c784 — __ZN4FMOD8ChannelI14getChannelInfoEPNS_17FMOD_CHANNEL_INFOE [watchdog]
// type: int __fastcall(FMOD::ChannelI *, int)
#[doc(alias = "FMOD::ChannelI::getChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")]
pub fn stub_7c784_wdog288() -> ! {
    todo!("0x7c784 FMOD::ChannelI::getChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")
}

// 0x7c83c — __ZN4FMOD8ChannelI11setPositionEjj [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelI::setPosition(unsigned int,unsigned int)")]
pub fn stub_7c83c_wdog289() -> ! {
    todo!("0x7c83c FMOD::ChannelI::setPosition(unsigned int,unsigned int)")
}

// 0x7ce58 — __ZN4FMOD8ChannelI13setLoopPointsEjjjj [watchdog]
// type: int __fastcall(unsigned __int64 this, unsigned int, unsigned int, unsigned int)
#[doc(alias = "FMOD::ChannelI::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")]
pub fn stub_7ce58_wdog290() -> ! {
    todo!("0x7ce58 FMOD::ChannelI::setLoopPoints(unsigned int,unsigned int,unsigned int,unsigned int)")
}

// 0x7d208 — __ZN4FMOD8ChannelI14setChannelInfoEPNS_17FMOD_CHANNEL_INFOE [watchdog]
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::setChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")]
pub fn stub_7d208_wdog291() -> ! {
    todo!("0x7d208 FMOD::ChannelI::setChannelInfo(FMOD::FMOD_CHANNEL_INFO *)")
}

// 0x7d480 — __ZN4FMOD8ChannelI12forceVirtualEb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::forceVirtual(bool)")]
pub fn stub_7d480_wdog292() -> ! {
    todo!("0x7d480 FMOD::ChannelI::forceVirtual(bool)")
}

// 0x7d5fc — __ZN4FMOD8ChannelI14updatePositionEv [watchdog]
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::updatePosition(void)")]
pub fn stub_7d5fc_wdog293() -> ! {
    todo!("0x7d5fc FMOD::ChannelI::updatePosition(void)")
}

// 0x7d8c4 — __ZN4FMOD8ChannelI22set3DOcclusionInternalEffb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, float, float, bool)
#[doc(alias = "FMOD::ChannelI::set3DOcclusionInternal(float,float,bool)")]
pub fn stub_7d8c4_wdog294() -> ! {
    todo!("0x7d8c4 FMOD::ChannelI::set3DOcclusionInternal(float,float,bool)")
}

// 0x7d9b8 — __ZN4FMOD8ChannelI11setPriorityEi [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, unsigned int)
#[doc(alias = "FMOD::ChannelI::setPriority(int)")]
pub fn stub_7d9b8_wdog295() -> ! {
    todo!("0x7d9b8 FMOD::ChannelI::setPriority(int)")
}

// 0x7d9d0 — __ZN4FMOD8ChannelI9setVolumeEfb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, float, bool)
#[doc(alias = "FMOD::ChannelI::setVolume(float,bool)")]
pub fn stub_7d9d0_wdog296() -> ! {
    todo!("0x7d9d0 FMOD::ChannelI::setVolume(float,bool)")
}

// 0x7db84 — __ZN4FMOD8ChannelI7setMuteEb [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::setMute(bool)")]
pub fn stub_7db84_wdog297() -> ! {
    todo!("0x7db84 FMOD::ChannelI::setMute(bool)")
}

// 0x7dc98 — __ZN4FMOD8ChannelI11setDefaultsEv [watchdog]
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::setDefaults(void)")]
pub fn stub_7dc98_wdog298() -> ! {
    todo!("0x7dc98 FMOD::ChannelI::setDefaults(void)")
}

// 0x7df78 — __ZN4FMOD8ChannelI6updateEib [watchdog]
// type: int __fastcall(FMOD::ChannelI *this, unsigned int, bool)
#[doc(alias = "FMOD::ChannelI::update(int,bool)")]
pub fn stub_7df78_wdog299() -> ! {
    todo!("0x7df78 FMOD::ChannelI::update(int,bool)")
}

// 0x7e58c — __ZN4FMOD8ChannelI7setModeEj
// type: int __fastcall(FMOD::ChannelI *this, unsigned int)
#[doc(alias = "FMOD::ChannelI::setMode(unsigned int)")]
pub fn stub_7e58c() -> ! {
    todo!("0x7e58c FMOD::ChannelI::setMode(unsigned int)")
}

// 0x7e8f0 — __ZN4FMOD8ChannelI9setPausedEb
// type: int __fastcall(FMOD::ChannelI *this, bool)
#[doc(alias = "FMOD::ChannelI::setPaused(bool)")]
pub fn stub_7e8f0() -> ! {
    todo!("0x7e8f0 FMOD::ChannelI::setPaused(bool)")
}

// 0x7ea20 — __ZN4FMOD8ChannelI23setChannelGroupInternalEPNS_13ChannelGroupIEbb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelGroupI *, bool, bool)
#[doc(alias = "FMOD::ChannelI::setChannelGroupInternal(FMOD::ChannelGroupI *,bool,bool)")]
pub fn stub_7ea20() -> ! {
    todo!("0x7ea20 FMOD::ChannelI::setChannelGroupInternal(FMOD::ChannelGroupI *,bool,bool)")
}

// 0x7ecf8 — __ZN4FMOD8ChannelI15setChannelGroupEPNS_13ChannelGroupIE
// type: int __fastcall(FMOD::ChannelI *this, FMOD::ChannelGroupI *)
#[doc(alias = "FMOD::ChannelI::setChannelGroup(FMOD::ChannelGroupI *)")]
pub fn stub_7ecf8() -> ! {
    todo!("0x7ecf8 FMOD::ChannelI::setChannelGroup(FMOD::ChannelGroupI *)")
}

// 0x7ed04 — __ZN4FMOD8ChannelI6stopExEj
// type: int __fastcall(FMOD::ChannelI *this, char)
#[doc(alias = "FMOD::ChannelI::stopEx(unsigned int)")]
pub fn stub_7ed04() -> ! {
    todo!("0x7ed04 FMOD::ChannelI::stopEx(unsigned int)")
}

// 0x7f0f4 — __ZN4FMOD8ChannelI4stopEv
// type: int __fastcall(FMOD::ChannelI *this)
#[doc(alias = "FMOD::ChannelI::stop(void)")]
pub fn stub_7f0f4() -> ! {
    todo!("0x7f0f4 FMOD::ChannelI::stop(void)")
}

// 0x7f0fc — __ZN4FMOD8ChannelI4playEPNS_4DSPIEbbb
// type: int __fastcall(FMOD::ChannelI *this, FMOD::DSPI *, bool, char, bool)
#[doc(alias = "FMOD::ChannelI::play(FMOD::DSPI *,bool,bool,bool)")]
pub fn stub_7f0fc() -> ! {
    todo!("0x7f0fc FMOD::ChannelI::play(FMOD::DSPI *,bool,bool,bool)")
}

// 0x7f23c — __ZN4FMOD8ChannelI4playEPNS_6SoundIEbbb
// type: int __fastcall(FMOD::ChannelI *this, unsigned __int8 **, bool, bool, bool)
#[doc(alias = "FMOD::ChannelI::play(FMOD::SoundI *,bool,bool,bool)")]
pub fn stub_7f23c() -> ! {
    todo!("0x7f23c FMOD::ChannelI::play(FMOD::SoundI *,bool,bool,bool)")
}

// 0x7f4a0 — __ZN4FMOD8ChannelI13getMemoryUsedEPNS_13MemoryTrackerE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::ChannelI::getMemoryUsed(FMOD::MemoryTracker *)")]
pub fn stub_7f4a0() -> ! {
    todo!("0x7f4a0 FMOD::ChannelI::getMemoryUsed(FMOD::MemoryTracker *)")
}

// 0x7f4f8 — __ZN4FMOD11ChannelPoolC2Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::ChannelPool::ChannelPool(void)")]
pub fn stub_7f4f8() -> ! {
    todo!("0x7f4f8 FMOD::ChannelPool::ChannelPool(void)")
}

// 0x7f514 — __ZN4FMOD11ChannelPoolC1Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "FMOD::ChannelPool::ChannelPool(void)")]
pub fn stub_7f514() -> ! {
    todo!("0x7f514 FMOD::ChannelPool::ChannelPool(void)")
}

// 0x7f518 — __ZN4FMOD11ChannelPool15allocateChannelEPPNS_11ChannelRealEiiPib
// type: int __fastcall(FMOD::ChannelPool *this, FMOD::ChannelReal **, int, int, int *, bool)
#[doc(alias = "FMOD::ChannelPool::allocateChannel(FMOD::ChannelReal **,int,int,int *,bool)")]
pub fn stub_7f518() -> ! {
    todo!("0x7f518 FMOD::ChannelPool::allocateChannel(FMOD::ChannelReal **,int,int,int *,bool)")
}

// 0x7f744 — __ZN4FMOD11ChannelPool14getNumChannelsEPi
// type: int __fastcall(FMOD::ChannelPool *this, int *)
#[doc(alias = "FMOD::ChannelPool::getNumChannels(int *)")]
pub fn stub_7f744() -> ! {
    todo!("0x7f744 FMOD::ChannelPool::getNumChannels(int *)")
}

// 0x7f75c — __ZN4FMOD11ChannelPool15getChannelsUsedEPi
// type: int __fastcall(FMOD::ChannelPool *this, int *)
#[doc(alias = "FMOD::ChannelPool::getChannelsUsed(int *)")]
pub fn stub_7f75c() -> ! {
    todo!("0x7f75c FMOD::ChannelPool::getChannelsUsed(int *)")
}

// 0x7f774 — __ZN4FMOD11ChannelPool10setChannelEiPNS_11ChannelRealEPNS_4DSPIE
// type: int __fastcall(_DWORD *, unsigned int, int, int)
#[doc(alias = "FMOD::ChannelPool::setChannel(int,FMOD::ChannelReal *,FMOD::DSPI *)")]
pub fn stub_7f774() -> ! {
    todo!("0x7f774 FMOD::ChannelPool::setChannel(int,FMOD::ChannelReal *,FMOD::DSPI *)")
}

// 0x7f7e8 — __ZN4FMOD11ChannelPool7releaseEv
// type: int __fastcall(FMOD::ChannelPool *this)
#[doc(alias = "FMOD::ChannelPool::release(void)")]
pub fn stub_7f7e8() -> ! {
    todo!("0x7f7e8 FMOD::ChannelPool::release(void)")
}

// 0x7f898 — __ZN4FMOD11ChannelPool4initEPNS_7SystemIEPNS_6OutputEi
// type: int __fastcall(FMOD::ChannelPool *this, FMOD::SystemI *, FMOD::Output *, int)
#[doc(alias = "FMOD::ChannelPool::init(FMOD::SystemI *,FMOD::Output *,int)")]
pub fn stub_7f898() -> ! {
    todo!("0x7f898 FMOD::ChannelPool::init(FMOD::SystemI *,FMOD::Output *,int)")
}

// 0x7f924 — __ZN4FMOD5Codec9getLengthEPjj
// type: int __fastcall(FMOD::Codec *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::Codec::getLength(unsigned int *,unsigned int)")]
pub fn stub_7f924() -> ! {
    todo!("0x7f924 FMOD::Codec::getLength(unsigned int *,unsigned int)")
}

// 0x7f984 — __ZN4FMOD5Codec17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::Codec *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::Codec::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_7f984() -> ! {
    todo!("0x7f984 FMOD::Codec::getMemoryUsedImpl(FMOD::MemoryTracker *)")
}

// 0x7f9ec — __ZN4FMOD5Codec8metaDataE12FMOD_TAGTYPEPKcPvj16FMOD_TAGDATATYPEb
// type: int __fastcall(int, int, int, int, size_t, int, char)
#[doc(alias = "FMOD::Codec::metaData(FMOD_TAGTYPE,char const*,void *,unsigned int,FMOD_TAGDATATYPE,bool)")]
pub fn stub_7f9ec() -> ! {
    todo!("0x7f9ec FMOD::Codec::metaData(FMOD_TAGTYPE,char const*,void *,unsigned int,FMOD_TAGDATATYPE,bool)")
}

// 0x7facc — __ZN4FMOD5Codec11getPositionEPjj
// type: int __fastcall(FMOD::Codec *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::Codec::getPosition(unsigned int *,unsigned int)")]
pub fn stub_7facc() -> ! {
    todo!("0x7facc FMOD::Codec::getPosition(unsigned int *,unsigned int)")
}

// 0x7fb54 — __ZN4FMOD5Codec19getMetadataFromFileEv
// type: int __fastcall(FMOD::Codec *this)
#[doc(alias = "FMOD::Codec::getMetadataFromFile(void)")]
pub fn stub_7fb54() -> ! {
    todo!("0x7fb54 FMOD::Codec::getMetadataFromFile(void)")
}

// 0x7fc24 — __ZN4FMOD5Codec4readEPvjPj
// type: int __fastcall(FMOD::Codec *this, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::Codec::read(void *,unsigned int,unsigned int *)")]
pub fn stub_7fc24() -> ! {
    todo!("0x7fc24 FMOD::Codec::read(void *,unsigned int,unsigned int *)")
}

// 0x7fd9c — __ZN4FMOD5Codec7releaseEv
// type: int __fastcall(FMOD::Codec *this)
#[doc(alias = "FMOD::Codec::release(void)")]
pub fn stub_7fd9c() -> ! {
    todo!("0x7fd9c FMOD::Codec::release(void)")
}

// 0x7fe6c — __ZN4FMOD5Codec11setPositionEijj
// type: int __fastcall(FMOD::Codec *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::Codec::setPosition(int,unsigned int,unsigned int)")]
pub fn stub_7fe6c() -> ! {
    todo!("0x7fe6c FMOD::Codec::setPosition(int,unsigned int,unsigned int)")
}

// 0x80388 — __ZN4FMOD9CodecAIFF19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecAIFF *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecAIFF::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_80388() -> ! {
    todo!("0x80388 FMOD::CodecAIFF::setPositionInternal(int,unsigned int,unsigned int)")
}

// 0x804cc — __ZN4FMOD9CodecAIFF19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecAIFF *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecAIFF::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_804cc() -> ! {
    todo!("0x804cc FMOD::CodecAIFF::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")
}

// 0x804d8 — __ZN4FMOD9CodecAIFF12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecAIFF *this, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecAIFF::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_804d8() -> ! {
    todo!("0x804d8 FMOD::CodecAIFF::readInternal(void *,unsigned int,unsigned int *)")
}

// 0x806e4 — __ZN4FMOD9CodecAIFF12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecAIFF *, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecAIFF::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_806e4() -> ! {
    todo!("0x806e4 FMOD::CodecAIFF::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")
}

// 0x806f0 — __ZN4FMOD9CodecAIFF13closeInternalEv
// type: int __fastcall(FMOD::CodecAIFF *this)
#[doc(alias = "FMOD::CodecAIFF::closeInternal(void)")]
pub fn stub_806f0() -> ! {
    todo!("0x806f0 FMOD::CodecAIFF::closeInternal(void)")
}

// 0x80744 — __ZN4FMOD9CodecAIFF13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecAIFF *)
#[doc(alias = "FMOD::CodecAIFF::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_80744() -> ! {
    todo!("0x80744 FMOD::CodecAIFF::closeCallback(FMOD_CODEC_STATE *)")
}

// 0x80750 — __ZN4FMOD23ConvertFromIeeeExtendedEPh
// type: int __fastcall(FMOD *this, unsigned __int8 *)
#[doc(alias = "FMOD::ConvertFromIeeeExtended(unsigned char *)")]
pub fn stub_80750() -> ! {
    todo!("0x80750 FMOD::ConvertFromIeeeExtended(unsigned char *)")
}

// 0x80864 — __ZN4FMOD9CodecAIFF12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecAIFF::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_80864() -> ! {
    todo!("0x80864 FMOD::CodecAIFF::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x81068 — __ZN4FMOD9CodecAIFF12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecAIFF::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_81068() -> ! {
    todo!("0x81068 FMOD::CodecAIFF::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x81074 — __ZN4FMOD9CodecAIFF16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecAIFF *this)
#[doc(alias = "FMOD::CodecAIFF::getDescriptionEx(void)")]
pub fn stub_81074() -> ! {
    todo!("0x81074 FMOD::CodecAIFF::getDescriptionEx(void)")
}

// 0x8115c — __GLOBAL__I__ZN4FMOD9aiffcodecE
// type: int()
#[doc(alias = "global constructor keyed to FMOD::aiffcodec")]
pub fn stub_8115c() -> ! {
    todo!("0x8115c global constructor keyed toFMOD::aiffcodec")
}

// 0x81168 — __ZN4FMOD8CodecDLS19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecDLS *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecDLS::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_81168() -> ! {
    todo!("0x81168 FMOD::CodecDLS::setPositionInternal(int,unsigned int,unsigned int)")
}

// 0x8132c — __ZN4FMOD8CodecDLS19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecDLS *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecDLS::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_8132c() -> ! {
    todo!("0x8132c FMOD::CodecDLS::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")
}

// 0x81338 — __ZN4FMOD8CodecDLS12readInternalEPvjPj
// type: int __fastcall(FMOD::File **this, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecDLS::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_81338() -> ! {
    todo!("0x81338 FMOD::CodecDLS::readInternal(void *,unsigned int,unsigned int *)")
}

// 0x813e8 — __ZN4FMOD8CodecDLS12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::File **, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecDLS::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_813e8() -> ! {
    todo!("0x813e8 FMOD::CodecDLS::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")
}

// 0x813f4 — __ZN4FMOD8CodecDLS13closeInternalEv
// type: int __fastcall(FMOD::CodecDLS *this)
#[doc(alias = "FMOD::CodecDLS::closeInternal(void)")]
pub fn stub_813f4() -> ! {
    todo!("0x813f4 FMOD::CodecDLS::closeInternal(void)")
}

// 0x815e0 — __ZN4FMOD8CodecDLS13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecDLS *)
#[doc(alias = "FMOD::CodecDLS::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_815e0() -> ! {
    todo!("0x815e0 FMOD::CodecDLS::closeCallback(FMOD_CODEC_STATE *)")
}

// 0x815ec — __ZN4FMOD8CodecDLS16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecDLS *this)
#[doc(alias = "FMOD::CodecDLS::getDescriptionEx(void)")]
pub fn stub_815ec() -> ! {
    todo!("0x815ec FMOD::CodecDLS::getDescriptionEx(void)")
}

// 0x8168c — __ZN4FMOD8CodecDLS10parseChunkEPcj
// type: int __fastcall(FMOD::File **this, char *, unsigned int)
#[doc(alias = "FMOD::CodecDLS::parseChunk(char *,unsigned int)")]
pub fn stub_8168c() -> ! {
    todo!("0x8168c FMOD::CodecDLS::parseChunk(char *,unsigned int)")
}

// 0x82848 — __ZN4FMOD8CodecDLS12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecDLS::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_82848() -> ! {
    todo!("0x82848 FMOD::CodecDLS::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x82970 — __ZN4FMOD8CodecDLS12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecDLS::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_82970() -> ! {
    todo!("0x82970 FMOD::CodecDLS::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x829c8 — __GLOBAL__I__ZN4FMOD8dlscodecE
// type: int()
#[doc(alias = "global constructor keyed to FMOD::dlscodec")]
pub fn stub_829c8() -> ! {
    todo!("0x829c8 global constructor keyed toFMOD::dlscodec")
}

// 0x829d4 — __ZN4FMODL24FMOD_FLAC_LengthCallbackEPK19FLAC__StreamDecoderPyPv
// type: int __fastcall(int, _DWORD *, int)
#[doc(alias = "FMOD::FMOD_FLAC_LengthCallback(FLAC__StreamDecoder const*,unsigned long long *,void *)")]
pub fn stub_829d4() -> ! {
    todo!("0x829d4 FMOD::FMOD_FLAC_LengthCallback(FLAC__StreamDecoder const*,unsigned long long *,void *)")
}

// 0x82a1c — __ZN4FMODL23FMOD_FLAC_ErrorCallbackEPK19FLAC__StreamDecoder30FLAC__StreamDecoderErrorStatusPv
// type: void()
#[doc(alias = "FMOD::FMOD_FLAC_ErrorCallback(FLAC__StreamDecoder const*,FLAC__StreamDecoderErrorStatus,void *)")]
pub fn stub_82a1c() -> ! {
    todo!("0x82a1c FMOD::FMOD_FLAC_ErrorCallback(FLAC__StreamDecoder const*,FLAC__StreamDecoderErrorStatus,void *)")
}

// 0x82a20 — __ZN4FMOD9CodecFLAC19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecFLAC *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecFLAC::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_82a20() -> ! {
    todo!("0x82a20 FMOD::CodecFLAC::setPositionInternal(int,unsigned int,unsigned int)")
}

// 0x82a70 — __ZN4FMOD9CodecFLAC19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecFLAC *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecFLAC::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_82a70() -> ! {
    todo!("0x82a70 FMOD::CodecFLAC::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")
}

// 0x82a7c — __ZN4FMOD9CodecFLAC12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecFLAC *this, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecFLAC::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_82a7c() -> ! {
    todo!("0x82a7c FMOD::CodecFLAC::readInternal(void *,unsigned int,unsigned int *)")
}

// 0x82adc — __ZN4FMOD9CodecFLAC12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecFLAC *, void *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecFLAC::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_82adc() -> ! {
    todo!("0x82adc FMOD::CodecFLAC::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")
}

// 0x82ae8 — __ZN4FMOD9CodecFLAC13closeInternalEv
// type: int __fastcall(FMOD::CodecFLAC *this)
#[doc(alias = "FMOD::CodecFLAC::closeInternal(void)")]
pub fn stub_82ae8() -> ! {
    todo!("0x82ae8 FMOD::CodecFLAC::closeInternal(void)")
}

// 0x82ba4 — __ZN4FMOD9CodecFLAC13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecFLAC *)
#[doc(alias = "FMOD::CodecFLAC::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_82ba4() -> ! {
    todo!("0x82ba4 FMOD::CodecFLAC::closeCallback(FMOD_CODEC_STATE *)")
}

// 0x82bb0 — __ZN4FMODL22FMOD_FLAC_SeekCallbackEPK19FLAC__StreamDecoderyPv
// type: bool __fastcall(int, int, int, int)
#[doc(alias = "FMOD::FMOD_FLAC_SeekCallback(FLAC__StreamDecoder const*,unsigned long long,void *)")]
pub fn stub_82bb0() -> ! {
    todo!("0x82bb0 FMOD::FMOD_FLAC_SeekCallback(FLAC__StreamDecoder const*,unsigned long long,void *)")
}

// 0x82bd0 — __ZN4FMODL22FMOD_FLAC_ReadCallbackEPK19FLAC__StreamDecoderPhPmPv
// type: int __fastcall(int, void *, unsigned int *, int)
#[doc(alias = "FMOD::FMOD_FLAC_ReadCallback(FLAC__StreamDecoder const*,unsigned char *,unsigned long *,void *)")]
pub fn stub_82bd0() -> ! {
    todo!("0x82bd0 FMOD::FMOD_FLAC_ReadCallback(FLAC__StreamDecoder const*,unsigned char *,unsigned long *,void *)")
}

// 0x82c14 — __ZN4FMOD9CodecFLAC12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecFLAC::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_82c14() -> ! {
    todo!("0x82c14 FMOD::CodecFLAC::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x82f38 — __ZN4FMOD9CodecFLAC12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int)
#[doc(alias = "FMOD::CodecFLAC::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_82f38() -> ! {
    todo!("0x82f38 FMOD::CodecFLAC::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x82f44 — __ZN4FMODL23FMOD_FLAC_WriteCallbackEPK19FLAC__StreamDecoderPK11FLAC__FramePKPKiPv
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "FMOD::FMOD_FLAC_WriteCallback(FLAC__StreamDecoder const*,FLAC__Frame const*,int const* const*,void *)")]
pub fn stub_82f44() -> ! {
    todo!("0x82f44 FMOD::FMOD_FLAC_WriteCallback(FLAC__StreamDecoder const*,FLAC__Frame const*,int const* const*,void *)")
}

// 0x830e4 — __ZN4FMODL26FMOD_FLAC_MetadataCallbackEPK19FLAC__StreamDecoderPK20FLAC__StreamMetadataPv
// type: void __fastcall(int, _DWORD *, int)
#[doc(alias = "FMOD::FMOD_FLAC_MetadataCallback(FLAC__StreamDecoder const*,FLAC__StreamMetadata const*,void *)")]
pub fn stub_830e4() -> ! {
    todo!("0x830e4 FMOD::FMOD_FLAC_MetadataCallback(FLAC__StreamDecoder const*,FLAC__StreamMetadata const*,void *)")
}

// 0x83298 — __ZN4FMODL21FMOD_FLAC_EofCallbackEPK19FLAC__StreamDecoderPv
// type: bool __fastcall(int, int)
#[doc(alias = "FMOD::FMOD_FLAC_EofCallback(FLAC__StreamDecoder const*,void *)")]
pub fn stub_83298() -> ! {
    todo!("0x83298 FMOD::FMOD_FLAC_EofCallback(FLAC__StreamDecoder const*,void *)")
}

// 0x832e0 — __ZN4FMODL22FMOD_FLAC_TellCallbackEPK19FLAC__StreamDecoderPyPv
// type: int __fastcall(int, _DWORD *, int)
#[doc(alias = "FMOD::FMOD_FLAC_TellCallback(FLAC__StreamDecoder const*,unsigned long long *,void *)")]
pub fn stub_832e0() -> ! {
    todo!("0x832e0 FMOD::FMOD_FLAC_TellCallback(FLAC__StreamDecoder const*,unsigned long long *,void *)")
}

// 0x83320 — __ZN4FMOD9CodecFLAC16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecFLAC *this)
#[doc(alias = "FMOD::CodecFLAC::getDescriptionEx(void)")]
pub fn stub_83320() -> ! {
    todo!("0x83320 FMOD::CodecFLAC::getDescriptionEx(void)")
}

// 0x8340c — __GLOBAL__I__ZN4FMOD9flaccodecE
// type: int()
#[doc(alias = "global constructor keyed to FMOD::flaccodec")]
pub fn stub_8340c() -> ! {
    todo!("0x8340c global constructor keyed toFMOD::flaccodec")
}

// 0x83418 — __ZN4FMOD8CodecFSB16getNumSyncPointsEiPi
// type: int __fastcall(FMOD::CodecFSB *this, int, int *)
#[doc(alias = "FMOD::CodecFSB::getNumSyncPoints(int,int *)")]
pub fn stub_83418() -> ! {
    todo!("0x83418 FMOD::CodecFSB::getNumSyncPoints(int,int *)")
}

// 0x83434 — __ZN4FMOD8CodecFSB16getSyncPointDataEiiPPcPi
// type: int __fastcall(FMOD::CodecFSB *this, int, int, char **, int *)
#[doc(alias = "FMOD::CodecFSB::getSyncPointData(int,int,char **,int *)")]
pub fn stub_83434() -> ! {
    todo!("0x83434 FMOD::CodecFSB::getSyncPointData(int,int,char **,int *)")
}

// 0x834a0 — __ZN4FMOD8CodecFSB16canPointInternalEv
// type: int __fastcall(FMOD::CodecFSB *this)
#[doc(alias = "FMOD::CodecFSB::canPointInternal(void)")]
pub fn stub_834a0() -> ! {
    todo!("0x834a0 FMOD::CodecFSB::canPointInternal(void)")
}

// 0x834c8 — __ZN4FMOD8CodecFSB16canPointCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecFSB *)
#[doc(alias = "FMOD::CodecFSB::canPointCallback(FMOD_CODEC_STATE *)")]
pub fn stub_834c8() -> ! {
    todo!("0x834c8 FMOD::CodecFSB::canPointCallback(FMOD_CODEC_STATE *)")
}

// 0x834d4 — __ZN4FMOD8CodecFSB16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecFSB *this)
#[doc(alias = "FMOD::CodecFSB::getDescriptionEx(void)")]
pub fn stub_834d4() -> ! {
    todo!("0x834d4 FMOD::CodecFSB::getDescriptionEx(void)")
}

// 0x835d4 — __ZN4FMOD8CodecFSB17getMemoryUsedImplEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecFSB *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::CodecFSB::getMemoryUsedImpl(FMOD::MemoryTracker *)")]
pub fn stub_835d4() -> ! {
    todo!("0x835d4 FMOD::CodecFSB::getMemoryUsedImpl(FMOD::MemoryTracker *)")
}

// 0x83858 — __ZN4FMOD8CodecFSB21getMemoryUsedCallbackEP16FMOD_CODEC_STATEPNS_13MemoryTrackerE
// type: int __fastcall(FMOD::CodecFSB *this, FMOD::MemoryTracker *)
#[doc(alias = "FMOD::CodecFSB::getMemoryUsedCallback(FMOD_CODEC_STATE *,FMOD::MemoryTracker *)")]
pub fn stub_83858() -> ! {
    todo!("0x83858 FMOD::CodecFSB::getMemoryUsedCallback(FMOD_CODEC_STATE *,FMOD::MemoryTracker *)")
}

// 0x838b0 — __ZN4FMOD8CodecFSB13closeInternalEv
// type: int __fastcall(FMOD::CodecFSB *this)
#[doc(alias = "FMOD::CodecFSB::closeInternal(void)")]
pub fn stub_838b0() -> ! {
    todo!("0x838b0 FMOD::CodecFSB::closeInternal(void)")
}

// 0x83c50 — __ZN4FMOD8CodecFSB13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecFSB *)
#[doc(alias = "FMOD::CodecFSB::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_83c50() -> ! {
    todo!("0x83c50 FMOD::CodecFSB::closeCallback(FMOD_CODEC_STATE *)")
}

// 0x83c5c — __ZN4FMOD8CodecFSB13resetInternalEv
// type: int __fastcall(FMOD::CodecFSB *this)
#[doc(alias = "FMOD::CodecFSB::resetInternal(void)")]
pub fn stub_83c5c() -> ! {
    todo!("0x83c5c FMOD::CodecFSB::resetInternal(void)")
}

// 0x83ce0 — __ZN4FMOD8CodecFSB13resetCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecFSB *)
#[doc(alias = "FMOD::CodecFSB::resetCallback(FMOD_CODEC_STATE *)")]
pub fn stub_83ce0() -> ! {
    todo!("0x83ce0 FMOD::CodecFSB::resetCallback(FMOD_CODEC_STATE *)")
}

// 0x83cec — __ZN4FMOD8CodecFSB21getWaveFormatInternalEiP21FMOD_CODEC_WAVEFORMAT
// type: int __fastcall(int, int, int *__b)
#[doc(alias = "FMOD::CodecFSB::getWaveFormatInternal(int,FMOD_CODEC_WAVEFORMAT *)")]
pub fn stub_83cec() -> ! {
    todo!("0x83cec FMOD::CodecFSB::getWaveFormatInternal(int,FMOD_CODEC_WAVEFORMAT *)")
}

// 0x842c4 — __ZN4FMOD8CodecFSB21getWaveFormatCallbackEP16FMOD_CODEC_STATEiP21FMOD_CODEC_WAVEFORMAT
// type: int __fastcall(int, int, int *)
#[doc(alias = "FMOD::CodecFSB::getWaveFormatCallback(FMOD_CODEC_STATE *,int,FMOD_CODEC_WAVEFORMAT *)")]
pub fn stub_842c4() -> ! {
    todo!("0x842c4 FMOD::CodecFSB::getWaveFormatCallback(FMOD_CODEC_STATE *,int,FMOD_CODEC_WAVEFORMAT *)")
}

// 0x842d0 — __ZN4FMOD8CodecFSB19soundcreateInternalEiP10FMOD_SOUND
// type: int __fastcall(FMOD::CodecFSB *, int, FMOD::SoundI *)
#[doc(alias = "FMOD::CodecFSB::soundcreateInternal(int,FMOD_SOUND *)")]
pub fn stub_842d0() -> ! {
    todo!("0x842d0 FMOD::CodecFSB::soundcreateInternal(int,FMOD_SOUND *)")
}

// 0x84494 — __ZN4FMOD8CodecFSB19soundcreateCallbackEP16FMOD_CODEC_STATEiP10FMOD_SOUND
// type: int __fastcall(FMOD::CodecFSB *, int, FMOD::SoundI *)
#[doc(alias = "FMOD::CodecFSB::soundcreateCallback(FMOD_CODEC_STATE *,int,FMOD_SOUND *)")]
pub fn stub_84494() -> ! {
    todo!("0x84494 FMOD::CodecFSB::soundcreateCallback(FMOD_CODEC_STATE *,int,FMOD_SOUND *)")
}

// 0x844a0 — __ZN4FMOD8CodecFSB19getPositionInternalEPjj
// type: int __fastcall(FMOD::CodecFSB *this, unsigned int *, unsigned int)
#[doc(alias = "FMOD::CodecFSB::getPositionInternal(unsigned int *,unsigned int)")]
pub fn stub_844a0() -> ! {
    todo!("0x844a0 FMOD::CodecFSB::getPositionInternal(unsigned int *,unsigned int)")
}

// 0x84540 — __ZN4FMOD8CodecFSB19getPositionCallbackEP16FMOD_CODEC_STATEPjj
// type: int __fastcall(FMOD::CodecFSB *, unsigned int *, unsigned int)
#[doc(alias = "FMOD::CodecFSB::getPositionCallback(FMOD_CODEC_STATE *,unsigned int *,unsigned int)")]
pub fn stub_84540() -> ! {
    todo!("0x84540 FMOD::CodecFSB::getPositionCallback(FMOD_CODEC_STATE *,unsigned int *,unsigned int)")
}

// 0x8454c — __ZN4FMOD8CodecFSB12readInternalEPvjPj
// type: int __fastcall(FMOD::CodecFSB *this, int, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecFSB::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_8454c() -> ! {
    todo!("0x8454c FMOD::CodecFSB::readInternal(void *,unsigned int,unsigned int *)")
}

// 0x84ef4 — __ZN4FMOD8CodecFSB12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: int __fastcall(FMOD::CodecFSB *, int, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecFSB::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_84ef4() -> ! {
    todo!("0x84ef4 FMOD::CodecFSB::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")
}

// 0x84f00 — __ZN4FMOD8CodecFSB12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "FMOD::CodecFSB::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_84f00() -> ! {
    todo!("0x84f00 FMOD::CodecFSB::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x86654 — __ZN4FMOD8CodecFSB12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "FMOD::CodecFSB::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_86654() -> ! {
    todo!("0x86654 FMOD::CodecFSB::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x86660 — __ZN4FMOD8CodecFSB19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecFSB *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecFSB::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_86660() -> ! {
    todo!("0x86660 FMOD::CodecFSB::setPositionInternal(int,unsigned int,unsigned int)")
}

// 0x86aa0 — __ZN4FMOD8CodecFSB19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecFSB *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecFSB::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_86aa0() -> ! {
    todo!("0x86aa0 FMOD::CodecFSB::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")
}

// 0x86b10 — __GLOBAL__I__ZN4FMOD8fsbcodecE
// type: int()
#[doc(alias = "global constructor keyed to FMOD::fsbcodec")]
pub fn stub_86b10() -> ! {
    todo!("0x86b10 global constructor keyed toFMOD::fsbcodec")
}

// 0x86b1c — __ZN4FMOD7CodecIT8readBitsEhPj
// type: int __fastcall(FMOD::CodecIT *this, unsigned __int8, unsigned int *)
#[doc(alias = "FMOD::CodecIT::readBits(unsigned char,unsigned int *)")]
pub fn stub_86b1c() -> ! {
    todo!("0x86b1c FMOD::CodecIT::readBits(unsigned char,unsigned int *)")
}

// 0x86bcc — __ZN4FMOD14MusicChannelIT11volumeSlideEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::volumeSlide(void)")]
pub fn stub_86bcc() -> ! {
    todo!("0x86bcc FMOD::MusicChannelIT::volumeSlide(void)")
}

// 0x86c34 — __ZN4FMOD14MusicChannelIT8panSlideEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::panSlide(void)")]
pub fn stub_86c34() -> ! {
    todo!("0x86c34 FMOD::MusicChannelIT::panSlide(void)")
}

// 0x86c9c — __ZN4FMOD14MusicChannelIT10portamentoEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::portamento(void)")]
pub fn stub_86c9c() -> ! {
    todo!("0x86c9c FMOD::MusicChannelIT::portamento(void)")
}

// 0x86d60 — __ZN4FMOD14MusicChannelIT7vibratoEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::vibrato(void)")]
pub fn stub_86d60() -> ! {
    todo!("0x86d60 FMOD::MusicChannelIT::vibrato(void)")
}

// 0x86eb0 — __ZN4FMOD14MusicChannelIT11fineVibratoEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::fineVibrato(void)")]
pub fn stub_86eb0() -> ! {
    todo!("0x86eb0 FMOD::MusicChannelIT::fineVibrato(void)")
}

// 0x87000 — __ZN4FMOD14MusicChannelIT7tremoloEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::tremolo(void)")]
pub fn stub_87000() -> ! {
    todo!("0x87000 FMOD::MusicChannelIT::tremolo(void)")
}

// 0x8710c — __ZN4FMOD14MusicChannelIT9panbrelloEv
// type: int __fastcall(FMOD::MusicChannelIT *this)
#[doc(alias = "FMOD::MusicChannelIT::panbrello(void)")]
pub fn stub_8710c() -> ! {
    todo!("0x8710c FMOD::MusicChannelIT::panbrello(void)")
}

// 0x87238 — __ZN4FMOD7CodecIT15processEnvelopeEPNS_18MusicEnvelopeStateEPNS_19MusicVirtualChannelEiPNS_17MusicEnvelopeNodeEiiiiih
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int, char)
#[doc(alias = "FMOD::CodecIT::processEnvelope(FMOD::MusicEnvelopeState *,FMOD::MusicVirtualChannel *,int,FMOD::MusicEnvelopeNode *,int,int,int,int,int,unsigned char)")]
pub fn stub_87238() -> ! {
    todo!("0x87238 FMOD::CodecIT::processEnvelope(FMOD::MusicEnvelopeState *,FMOD::MusicVirtualChannel *,int,FMOD::MusicEnvelopeNode *,int,int,int,int,int,unsigned char)")
}

// 0x874a0 — __ZN4FMOD7CodecIT20processPitchEnvelopeEPNS_19MusicVirtualChannelEPNS_15MusicInstrumentEi
// type: int __fastcall(int, int, _BYTE *, int)
#[doc(alias = "FMOD::CodecIT::processPitchEnvelope(FMOD::MusicVirtualChannel *,FMOD::MusicInstrument *,int)")]
pub fn stub_874a0() -> ! {
    todo!("0x874a0 FMOD::CodecIT::processPitchEnvelope(FMOD::MusicVirtualChannel *,FMOD::MusicInstrument *,int)")
}

// 0x87bd8 — __ZN4FMOD7CodecIT13sampleVibratoEPNS_19MusicVirtualChannelE
// type: int __fastcall(int, int)
#[doc(alias = "FMOD::CodecIT::sampleVibrato(FMOD::MusicVirtualChannel *)")]
pub fn stub_87bd8() -> ! {
    todo!("0x87bd8 FMOD::CodecIT::sampleVibrato(FMOD::MusicVirtualChannel *)")
}

// 0x87cdc — __ZN4FMOD14MusicChannelIT17processVolumeByteEPNS_9MusicNoteEb
// type: int __fastcall(FMOD::MusicChannelIT *this, _BYTE *, char)
#[doc(alias = "FMOD::MusicChannelIT::processVolumeByte(FMOD::MusicNote *,bool)")]
pub fn stub_87cdc() -> ! {
    todo!("0x87cdc FMOD::MusicChannelIT::processVolumeByte(FMOD::MusicNote *,bool)")
}

// 0x87f7c — __ZN4FMOD7CodecIT13closeInternalEv
// type: int __fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::closeInternal(void)")]
pub fn stub_87f7c() -> ! {
    todo!("0x87f7c FMOD::CodecIT::closeInternal(void)")
}

// 0x883f0 — __ZN4FMOD7CodecIT13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecIT *)
#[doc(alias = "FMOD::CodecIT::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_883f0() -> ! {
    todo!("0x883f0 FMOD::CodecIT::closeCallback(FMOD_CODEC_STATE *)")
}

// 0x883fc — __ZN4FMOD7CodecIT9freeBlockEv
// type: int __fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::freeBlock(void)")]
pub fn stub_883fc() -> ! {
    todo!("0x883fc FMOD::CodecIT::freeBlock(void)")
}

// 0x88450 — __ZN4FMOD7CodecIT9unpackRowEv
// type: int __fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::unpackRow(void)")]
pub fn stub_88450() -> ! {
    todo!("0x88450 FMOD::CodecIT::unpackRow(void)")
}

// 0x88644 — __ZN4FMOD7CodecIT16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::getDescriptionEx(void)")]
pub fn stub_88644() -> ! {
    todo!("0x88644 FMOD::CodecIT::getDescriptionEx(void)")
}

// 0x8875c — __ZN4FMOD7CodecIT9readBlockEPPa
// type: int __fastcall(FMOD::CodecIT *this, unsigned __int8 **)
#[doc(alias = "FMOD::CodecIT::readBlock(signed char **)")]
pub fn stub_8875c() -> ! {
    todo!("0x8875c FMOD::CodecIT::readBlock(signed char **)")
}

// 0x88818 — __ZN4FMOD7CodecIT12decompress16EPPvS1_ibi
// type: int __fastcall(FMOD::CodecIT *this, unsigned __int8 **, _WORD *, int, bool, int)
#[doc(alias = "FMOD::CodecIT::decompress16(void **,void *,int,bool,int)")]
pub fn stub_88818() -> ! {
    todo!("0x88818 FMOD::CodecIT::decompress16(void **,void *,int,bool,int)")
}

// 0x88a34 — __ZN4FMOD7CodecIT11decompress8EPPvS1_ibi
// type: int __fastcall(FMOD::CodecIT *this, unsigned __int8 **, _BYTE *, int, bool, int)
#[doc(alias = "FMOD::CodecIT::decompress8(void **,void *,int,bool,int)")]
pub fn stub_88a34() -> ! {
    todo!("0x88a34 FMOD::CodecIT::decompress8(void **,void *,int,bool,int)")
}

// 0x88c44 — __ZN4FMOD7CodecIT4playEb
// type: int __fastcall(FMOD::CodecIT *this, bool)
#[doc(alias = "FMOD::CodecIT::play(bool)")]
pub fn stub_88c44() -> ! {
    todo!("0x88c44 FMOD::CodecIT::play(bool)")
}

// 0x88ccc — __ZN4FMOD7CodecIT9updateRowEb
// type: int __fastcall(FMOD::CodecIT *this, bool)
#[doc(alias = "FMOD::CodecIT::updateRow(bool)")]
pub fn stub_88ccc() -> ! {
    todo!("0x88ccc FMOD::CodecIT::updateRow(bool)")
}

// 0x8b660 — __ZN4FMOD7CodecIT6updateEb
// type: int __fastcall(FMOD::CodecIT *this, bool)
#[doc(alias = "FMOD::CodecIT::update(bool)")]
pub fn stub_8b660() -> ! {
    todo!("0x8b660 FMOD::CodecIT::update(bool)")
}

// 0x8b854 — __ZN4FMOD7CodecIT19setPositionInternalEijj
// type: int __fastcall(FMOD::CodecIT *this, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecIT::setPositionInternal(int,unsigned int,unsigned int)")]
pub fn stub_8b854() -> ! {
    todo!("0x8b854 FMOD::CodecIT::setPositionInternal(int,unsigned int,unsigned int)")
}

// 0x8b908 — __ZN4FMOD7CodecIT19setPositionCallbackEP16FMOD_CODEC_STATEijj
// type: int __fastcall(FMOD::CodecIT *, int, unsigned int, unsigned int)
#[doc(alias = "FMOD::CodecIT::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")]
pub fn stub_8b908() -> ! {
    todo!("0x8b908 FMOD::CodecIT::setPositionCallback(FMOD_CODEC_STATE *,int,unsigned int,unsigned int)")
}

// 0x8b914 — __ZN4FMOD7CodecIT15calculateLengthEv
// type: int __fastcall(FMOD::CodecIT *this)
#[doc(alias = "FMOD::CodecIT::calculateLength(void)")]
pub fn stub_8b914() -> ! {
    todo!("0x8b914 FMOD::CodecIT::calculateLength(void)")
}

// 0x8b978 — __ZN4FMOD7CodecIT12openInternalEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, _DWORD *)
#[doc(alias = "FMOD::CodecIT::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_8b978() -> ! {
    todo!("0x8b978 FMOD::CodecIT::openInternal(unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x8e7bc — __ZN4FMOD7CodecIT12openCallbackEP16FMOD_CODEC_STATEjP22FMOD_CREATESOUNDEXINFO
// type: int __fastcall(int, __int16, _DWORD *)
#[doc(alias = "FMOD::CodecIT::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")]
pub fn stub_8e7bc() -> ! {
    todo!("0x8e7bc FMOD::CodecIT::openCallback(FMOD_CODEC_STATE *,unsigned int,FMOD_CREATESOUNDEXINFO *)")
}

// 0x8e7c8 — __ZN4FMOD7CodecIT12readInternalEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecIT *this, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecIT::readInternal(void *,unsigned int,unsigned int *)")]
pub fn stub_8e7c8() -> ! {
    todo!("0x8e7c8 FMOD::CodecIT::readInternal(void *,unsigned int,unsigned int *)")
}

// 0x8ebc0 — __ZN4FMOD7CodecIT12readCallbackEP16FMOD_CODEC_STATEPvjPj
// type: unsigned int *__fastcall(FMOD::CodecIT *, char *, unsigned int, unsigned int *)
#[doc(alias = "FMOD::CodecIT::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")]
pub fn stub_8ebc0() -> ! {
    todo!("0x8ebc0 FMOD::CodecIT::readCallback(FMOD_CODEC_STATE *,void *,unsigned int,unsigned int *)")
}

// 0x8ec18 — __GLOBAL__I__ZN4FMOD7itcodecE
// type: int()
#[doc(alias = "global constructor keyed to FMOD::itcodec")]
pub fn stub_8ec18() -> ! {
    todo!("0x8ec18 global constructor keyed toFMOD::itcodec")
}

// 0x8ec24 — __ZN4FMOD19CodecMIDISubChannel15findArticulatorEii
// type: int __fastcall(FMOD::CodecMIDISubChannel *this, int, int)
#[doc(alias = "FMOD::CodecMIDISubChannel::findArticulator(int,int)")]
pub fn stub_8ec24() -> ! {
    todo!("0x8ec24 FMOD::CodecMIDISubChannel::findArticulator(int,int)")
}

// 0x8ec8c — __ZN4FMOD19CodecMIDISubChannel14articulateDestENS_14CONN_SRC_FLAGSEiPi
// type: int __fastcall(int, __int16, int, _DWORD *)
#[doc(alias = "FMOD::CodecMIDISubChannel::articulateDest(FMOD::CONN_SRC_FLAGS,int,int *)")]
pub fn stub_8ec8c() -> ! {
    todo!("0x8ec8c FMOD::CodecMIDISubChannel::articulateDest(FMOD::CONN_SRC_FLAGS,int,int *)")
}

// 0x8ef90 — __ZN4FMOD19CodecMIDISubChannel22getTimeCentsFromlScaleEi
// type: int __fastcall(FMOD::CodecMIDISubChannel *this, int)
#[doc(alias = "FMOD::CodecMIDISubChannel::getTimeCentsFromlScale(int)")]
pub fn stub_8ef90() -> ! {
    todo!("0x8ef90 FMOD::CodecMIDISubChannel::getTimeCentsFromlScale(int)")
}

// 0x8f00c — __ZN4FMOD16CodecMIDIChannel8getSoundEiPPNS_6SoundIEPPNS_18CodecDLSInstrumentEPiS7_S7_PbS7_S7_PPNS_19DLS_CONNECTIONBLOCKE
// type: int __fastcall(int, int, _DWORD *, _DWORD *, _DWORD *, _DWORD *, _DWORD *, int, _DWORD *, _DWORD *, _DWORD *)
#[doc(alias = "FMOD::CodecMIDIChannel::getSound(int,FMOD::SoundI **,FMOD::CodecDLSInstrument **,int *,int *,int *,bool *,int *,int *,FMOD::DLS_CONNECTIONBLOCK **)")]
pub fn stub_8f00c() -> ! {
    todo!("0x8f00c FMOD::CodecMIDIChannel::getSound(int,FMOD::SoundI **,FMOD::CodecDLSInstrument **,int *,int *,int *,bool *,int *,int *,FMOD::DLS_CONNECTIONBLOCK **)")
}

// 0x8f274 — __ZN4FMOD14CodecMIDITrack10readVarLenEPj
// type: int __fastcall(FMOD::CodecMIDITrack *this, unsigned int *)
#[doc(alias = "FMOD::CodecMIDITrack::readVarLen(unsigned int *)")]
pub fn stub_8f274() -> ! {
    todo!("0x8f274 FMOD::CodecMIDITrack::readVarLen(unsigned int *)")
}

// 0x8f2ec — __ZN4FMOD14CodecMIDITrack8readByteEPh
// type: int __fastcall(int this, unsigned __int8 *)
#[doc(alias = "FMOD::CodecMIDITrack::readByte(unsigned char *)")]
pub fn stub_8f2ec() -> ! {
    todo!("0x8f2ec FMOD::CodecMIDITrack::readByte(unsigned char *)")
}

// 0x8f320 — __ZN4FMOD9CodecMIDI27getMusicNumChannelsInternalEPi
// type: int __fastcall(FMOD::CodecMIDI *this, int *)
#[doc(alias = "FMOD::CodecMIDI::getMusicNumChannelsInternal(int *)")]
pub fn stub_8f320() -> ! {
    todo!("0x8f320 FMOD::CodecMIDI::getMusicNumChannelsInternal(int *)")
}

// 0x8f35c — __ZN4FMOD9CodecMIDI29setMusicChannelVolumeInternalEif
// type: int __fastcall(FMOD::CodecMIDI *this, unsigned int, float)
#[doc(alias = "FMOD::CodecMIDI::setMusicChannelVolumeInternal(int,float)")]
pub fn stub_8f35c() -> ! {
    todo!("0x8f35c FMOD::CodecMIDI::setMusicChannelVolumeInternal(int,float)")
}

// 0x8f3fc — __ZN4FMOD9CodecMIDI29getMusicChannelVolumeInternalEiPf
// type: int __fastcall(FMOD::CodecMIDI *this, unsigned int, float *)
#[doc(alias = "FMOD::CodecMIDI::getMusicChannelVolumeInternal(int,float *)")]
pub fn stub_8f3fc() -> ! {
    todo!("0x8f3fc FMOD::CodecMIDI::getMusicChannelVolumeInternal(int,float *)")
}

// 0x8f488 — __ZN4FMOD9CodecMIDI21setMusicSpeedInternalEf
// type: int __fastcall(FMOD::CodecMIDI *this, float)
#[doc(alias = "FMOD::CodecMIDI::setMusicSpeedInternal(float)")]
pub fn stub_8f488() -> ! {
    todo!("0x8f488 FMOD::CodecMIDI::setMusicSpeedInternal(float)")
}

// 0x8f528 — __ZN4FMOD9CodecMIDI21getMusicSpeedInternalEPf
// type: int __fastcall(FMOD::CodecMIDI *this, float *)
#[doc(alias = "FMOD::CodecMIDI::getMusicSpeedInternal(float *)")]
pub fn stub_8f528() -> ! {
    todo!("0x8f528 FMOD::CodecMIDI::getMusicSpeedInternal(float *)")
}

// 0x8f540 — __ZN4FMOD9CodecMIDI27getMusicNumChannelsCallbackEP16FMOD_CODEC_STATEPi
// type: int __fastcall(FMOD::CodecMIDI *, int *)
#[doc(alias = "FMOD::CodecMIDI::getMusicNumChannelsCallback(FMOD_CODEC_STATE *,int *)")]
pub fn stub_8f540() -> ! {
    todo!("0x8f540 FMOD::CodecMIDI::getMusicNumChannelsCallback(FMOD_CODEC_STATE *,int *)")
}

// 0x8f54c — __ZN4FMOD9CodecMIDI29setMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEif
// type: int __fastcall(FMOD::CodecMIDI *, unsigned int, float)
#[doc(alias = "FMOD::CodecMIDI::setMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float)")]
pub fn stub_8f54c() -> ! {
    todo!("0x8f54c FMOD::CodecMIDI::setMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float)")
}

// 0x8f558 — __ZN4FMOD9CodecMIDI29getMusicChannelVolumeCallbackEP16FMOD_CODEC_STATEiPf
// type: int __fastcall(FMOD::CodecMIDI *, unsigned int, float *)
#[doc(alias = "FMOD::CodecMIDI::getMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float *)")]
pub fn stub_8f558() -> ! {
    todo!("0x8f558 FMOD::CodecMIDI::getMusicChannelVolumeCallback(FMOD_CODEC_STATE *,int,float *)")
}

// 0x8f564 — __ZN4FMOD9CodecMIDI21setMusicSpeedCallbackEP16FMOD_CODEC_STATEf
// type: int __fastcall(FMOD::CodecMIDI *, float)
#[doc(alias = "FMOD::CodecMIDI::setMusicSpeedCallback(FMOD_CODEC_STATE *,float)")]
pub fn stub_8f564() -> ! {
    todo!("0x8f564 FMOD::CodecMIDI::setMusicSpeedCallback(FMOD_CODEC_STATE *,float)")
}

// 0x8f570 — __ZN4FMOD9CodecMIDI21getMusicSpeedCallbackEP16FMOD_CODEC_STATEPf
// type: int __fastcall(FMOD::CodecMIDI *, float *)
#[doc(alias = "FMOD::CodecMIDI::getMusicSpeedCallback(FMOD_CODEC_STATE *,float *)")]
pub fn stub_8f570() -> ! {
    todo!("0x8f570 FMOD::CodecMIDI::getMusicSpeedCallback(FMOD_CODEC_STATE *,float *)")
}

// 0x8f57c — __ZN4FMOD9CodecMIDI16getDescriptionExEv
// type: int *__fastcall(FMOD::CodecMIDI *this)
#[doc(alias = "FMOD::CodecMIDI::getDescriptionEx(void)")]
pub fn stub_8f57c() -> ! {
    todo!("0x8f57c FMOD::CodecMIDI::getDescriptionEx(void)")
}

// 0x8f674 — __ZN4FMOD9CodecMIDI13closeInternalEv
// type: int __fastcall(FMOD::CodecMIDI *this)
#[doc(alias = "FMOD::CodecMIDI::closeInternal(void)")]
pub fn stub_8f674() -> ! {
    todo!("0x8f674 FMOD::CodecMIDI::closeInternal(void)")
}

// 0x8f8d0 — __ZN4FMOD9CodecMIDI13closeCallbackEP16FMOD_CODEC_STATE
// type: int __fastcall(FMOD::CodecMIDI *)
#[doc(alias = "FMOD::CodecMIDI::closeCallback(FMOD_CODEC_STATE *)")]
pub fn stub_8f8d0() -> ! {
    todo!("0x8f8d0 FMOD::CodecMIDI::closeCallback(FMOD_CODEC_STATE *)")
}

// 0x8f8dc — __ZN4FMOD14CodecMIDITrack4readEPvi
// type: int __fastcall(FMOD::CodecMIDITrack *this, void *, size_t)
#[doc(alias = "FMOD::CodecMIDITrack::read(void *,int)")]
pub fn stub_8f8dc() -> ! {
    todo!("0x8f8dc FMOD::CodecMIDITrack::read(void *,int)")
}

// 0x8f944 — __ZN4FMOD14CodecMIDITrack6addTagEPKcib
// type: int __fastcall(FMOD::CodecMIDITrack *this, const char *, size_t, bool)
#[doc(alias = "FMOD::CodecMIDITrack::addTag(char const*,int,bool)")]
pub fn stub_8f944() -> ! {
    todo!("0x8f944 FMOD::CodecMIDITrack::addTag(char const*,int,bool)")
}

// 0x8fa30 — __ZN4FMOD19CodecMIDISubChannel17setUpArticulatorsEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
#[doc(alias = "FMOD::CodecMIDISubChannel::setUpArticulators(void)")]
pub fn stub_8fa30() -> ! {
    todo!("0x8fa30 FMOD::CodecMIDISubChannel::setUpArticulators(void)")
}

// 0x8ff60 — __ZN4FMOD19CodecMIDISubChannel9updatePanEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
#[doc(alias = "FMOD::CodecMIDISubChannel::updatePan(void)")]
pub fn stub_8ff60() -> ! {
    todo!("0x8ff60 FMOD::CodecMIDISubChannel::updatePan(void)")
}

// 0x8ffa4 — __ZN4FMOD19CodecMIDISubChannel11updatePitchEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
#[doc(alias = "FMOD::CodecMIDISubChannel::updatePitch(void)")]
pub fn stub_8ffa4() -> ! {
    todo!("0x8ffa4 FMOD::CodecMIDISubChannel::updatePitch(void)")
}

// 0x9034c — __ZN4FMOD19CodecMIDISubChannel4stopEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
#[doc(alias = "FMOD::CodecMIDISubChannel::stop(void)")]
pub fn stub_9034c() -> ! {
    todo!("0x9034c FMOD::CodecMIDISubChannel::stop(void)")
}

// 0x903bc — __ZN4FMOD9CodecMIDI4playEb
// type: int __fastcall(FMOD::CodecMIDI *this, bool)
#[doc(alias = "FMOD::CodecMIDI::play(bool)")]
pub fn stub_903bc() -> ! {
    todo!("0x903bc FMOD::CodecMIDI::play(bool)")
}

// 0x90584 — __ZN4FMOD19CodecMIDISubChannel12updateVolumeEv
// type: int __fastcall(FMOD::CodecMIDISubChannel *this)
#[doc(alias = "FMOD::CodecMIDISubChannel::updateVolume(void)")]
pub fn stub_90584() -> ! {
    todo!("0x90584 FMOD::CodecMIDISubChannel::updateVolume(void)")
}

// 0x90984 — __ZN4FMOD16CodecMIDIChannel6updateEv
// type: int __fastcall(FMOD::CodecMIDIChannel *this)
#[doc(alias = "FMOD::CodecMIDIChannel::update(void)")]
pub fn stub_90984() -> ! {
    todo!("0x90984 FMOD::CodecMIDIChannel::update(void)")
}

// 0x90a44 — __ZN4FMOD16CodecMIDIChannel7processEhbhb
// type: int __fastcall(FMOD::CodecMIDIChannel *this, unsigned __int8, bool, unsigned __int8, bool)
#[doc(alias = "FMOD::CodecMIDIChannel::process(unsigned char,bool,unsigned char,bool)")]
pub fn stub_90a44() -> ! {
    todo!("0x90a44 FMOD::CodecMIDIChannel::process(unsigned char,bool,unsigned char,bool)")
}

// 0x91454 — __ZN4FMOD14CodecMIDITrack7processEb
// type: int __fastcall(FMOD::CodecMIDITrack *this, bool)
#[doc(alias = "FMOD::CodecMIDITrack::process(bool)")]
pub fn stub_91454() -> ! {
    todo!("0x91454 FMOD::CodecMIDITrack::process(bool)")
}
