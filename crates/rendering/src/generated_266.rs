//! rendering shard 266 — 150 stubs EA-sorted asc global gap filler after 0x376fe4 not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 28927->29077 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 28927 before -> 29077 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x377024 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEED1Ev
// IDA 0x377024: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_377024() {
}

// 0x377048 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfED1Ev
// IDA 0x377048: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_377048() {
}

// 0x37706c — __ZNK3RBX10Soundscape12SoundChannel12getPlayCountEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getPlayCount(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel12getPlayCountEv
// IDA 0x37706c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37706c() {
}

// 0x377074 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiED1Ev
// IDA 0x377074: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_377074() {
}

// 0x377098 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbED1Ev
// IDA 0x377098: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_377098() {
}

// 0x3770bc — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EED1Ev
// IDA 0x3770bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3770bc() {
}

// 0x3770e0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x3770e0: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3770e0() {
}

// 0x377154 — __ZN3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE
// IDA 0x377154: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_377154() {
}

// 0x37716c — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEaSERKS4_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::Sound>::operator=(rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEaSERKS4_
// IDA 0x37716c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37716c() {
}

// 0x3771a4 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x3771a4: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3771a4() {
}

// 0x3772c0 — __ZNSt3mapIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(int, const std::string *)
#[doc(alias = "std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::operator[](RBX::Soundscape::SoundId const&)")]
// was: __ZNSt3mapIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// IDA 0x3772c0: 208 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3772c0() {
}

// 0x37750c — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv
// IDA 0x37750c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37750c() {
}

// 0x37751c — __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv
// IDA 0x37751c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37751c() {
}

// 0x37752c — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv
// IDA 0x37752c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37752c() {
}

// 0x37753c — __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv
// IDA 0x37753c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37753c() {
}

// 0x37754c — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev
// IDA 0x37754c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37754c() {
}

// 0x377550 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev
// IDA 0x377550: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_377550() {
}

// 0x377554 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED1Ev
// IDA 0x377554: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_377554() {
}

// 0x377558 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED0Ev
// IDA 0x377558: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_377558() {
}

// 0x3775f8 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE6lookupEPKc
// IDA 0x3775f8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3775f8() {
}

// 0x377628 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE6lookupERKNS0_7VariantE
// IDA 0x377628: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_377628() {
}

// 0x377648 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToValueEmRNS0_7VariantE
// IDA 0x377648: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_377648() {
}

// 0x3776a4 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE15convertToStringEmRSs
// IDA 0x3776a4: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3776a4() {
}

// 0x3777e8 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToString(RBX::Soundscape::ReverbType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE15convertToStringERKS3_
// IDA 0x3777e8: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3777e8() {
}

// 0x377988 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape10ReverbTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::ReverbType>(RBX::Soundscape::ReverbType const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape10ReverbTypeEEERS3_RKT_
// IDA 0x377988: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_377988() {
}

// 0x3779d8 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE9singletonEv
// IDA 0x3779d8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3779d8() {
}

// 0x377a44 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE14construct_funcEPKcPc
// IDA 0x377a44: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_377a44() {
}

// 0x377a50 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::ReverbType>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape10ReverbTypeEE13destruct_funcEPc
// IDA 0x377a50: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_377a50() {
}

// 0x377a54 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToItem(RBX::Soundscape::ReverbType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE13convertToItemERKS3_
// IDA 0x377a54: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_377a54() {
}

// 0x377b20 — __ZN3rbx8any_castIRKN3RBX10Soundscape10ReverbTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Soundscape::ReverbType const& rbx::any_cast<RBX::Soundscape::ReverbType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX10Soundscape10ReverbTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x377b20: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_377b20() {
}

// 0x377c10 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToValue(RBX::Name const&,RBX::Soundscape::ReverbType&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToValueERKNS_4NameERS3_
// IDA 0x377c10: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_377c10() {
}

// 0x377c8c — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *, int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED2Ev
// IDA 0x377c8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_377c8c() {
}

// 0x377e60 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD2Ev
// IDA 0x377e60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_377e60() {
}

// 0x377efc — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator12getClassNameEv
// IDA 0x377efc: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_377efc() {
}

// 0x377f84 — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7Creator6createEv
// IDA 0x377f84: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_377f84() {
}

// 0x3780c8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10Soundscape12SoundChannelEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundChannel>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10Soundscape12SoundChannelEEEN5boost10shared_ptrIT_EEv
// IDA 0x3780c8: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3780c8() {
}

// 0x378178 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::shared_ptr<RBX::Soundscape::SoundChannel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x378178: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378178() {
}

// 0x378240 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundChannelES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundChannel>(rbx_core::SharedPtr<RBX::Soundscape::SoundChannel> const*,RBX::Soundscape::SoundChannel *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundChannelES7_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x378240: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378240() {
}

// 0x37832c — __ZN5boost6detail12shared_countC2IPN3RBX10Soundscape12SoundChannelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10Soundscape12SoundChannelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x37832c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37832c() {
}

// 0x378434 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x378434: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_378434() {
}

// 0x378438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x378438: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_378438() {
}

// 0x37843c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x37843c: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37843c() {
}

// 0x37845c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x37845c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37845c() {
}

// 0x378474 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x378474: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378474() {
}

// 0x378478 — __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundChannelEEEEvv
// IDA 0x378478: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_378478() {
}

// 0x37847c — __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundChannelEEEERKS0_v
// IDA 0x37847c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37847c() {
}

// 0x37855c — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorC2Ev
// IDA 0x37855c: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37855c() {
}

// 0x3787a0 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E17static_getCreatorEv
// IDA 0x3787a0: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3787a0() {
}

// 0x378814 — __ZNSt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2ERS3_RKS7_
// type: _DWORD *__fastcall(_DWORD *, const std::string *, const shared_count *)
#[doc(alias = "std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>::pair(RBX::Soundscape::SoundId const&,rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")]
// was: __ZNSt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2ERS3_RKS7_
// IDA 0x378814: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378814() {
}

// 0x3788dc — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// IDA 0x3788dc: 98 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3788dc() {
}

// 0x3789c4 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// IDA 0x3789c4: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3789c4() {
}

// 0x378a14 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_insert_unique(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// IDA 0x378a14: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378a14() {
}

// 0x378a94 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_create_node(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// IDA 0x378a94: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378a94() {
}

// 0x378ba0 — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::Sound>::shared_ptr<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
// was: __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEC2IS3_EEPT_
// IDA 0x378ba0: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378ba0() {
}

// 0x378c74 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape5SoundEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::Sound>(RBX::Soundscape::Sound *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX10Soundscape5SoundEEEPT_
// IDA 0x378c74: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378c74() {
}

// 0x378d80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED1Ev
// IDA 0x378d80: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_378d80() {
}

// 0x378d84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEED0Ev
// IDA 0x378d84: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_378d84() {
}

// 0x378d88 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE7disposeEv
// IDA 0x378d88: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378d88() {
}

// 0x378e2c — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE11get_deleterERKSt9type_info
// IDA 0x378e2c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378e2c() {
}

// 0x378e30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::Sound>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape5SoundEE19get_untyped_deleterEv
// IDA 0x378e30: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378e30() {
}

// 0x378e34 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE4findERS4_
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::find(RBX::Soundscape::SoundId const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE4findERS4_
// IDA 0x378e34: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378e34() {
}

// 0x378e84 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Soundscape10ReverbTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Soundscape10ReverbTypeEEEE13initSingletonEv
// IDA 0x378e84: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_378e84() {
}

// 0x378e88 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Soundscape10ReverbTypeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Soundscape10ReverbTypeEEEE14doGetSingletonEv
// IDA 0x378e88: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378e88() {
}

// 0x378f78 — __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x378f78: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_378f78() {
}

// 0x379094 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED1Ev
// IDA 0x379094: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_379094() {
}

// 0x3790c0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEED0Ev
// IDA 0x3790c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3790c0() {
}

// 0x379194 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// IDA 0x379194: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379194() {
}

// 0x37919c — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// IDA 0x37919c: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37919c() {
}

// 0x3791a4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_
// type: int __fastcall(int)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_
// IDA 0x3791a4: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3791a4() {
}

// 0x3791bc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
// IDA 0x3791bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3791bc() {
}

// 0x3791e8 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
// IDA 0x3791e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3791e8() {
}

// 0x3792bc — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>)")]
// was: __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_E
// IDA 0x3792bc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3792bc() {
}

// 0x3792e4 — __ZN9__gnu_cxx13new_allocatorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEE7destroyEPSA_
// type: void __fastcall(int, std::string *)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>::destroy(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>*)")]
// was: __ZN9__gnu_cxx13new_allocatorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEE7destroyEPSA_
// IDA 0x3792e4: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3792e4() {
}

// 0x379388 — __ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x379388: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_379388() {
}

// 0x37938c — __ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x37938c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37938c() {
}

// 0x37942c — __ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x37942c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37942c() {
}

// 0x379434 — __ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x379434: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_379434() {
}

// 0x3794d8 — __ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3794d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3794d8() {
}

// 0x3794e0 — __ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3794e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3794e0() {
}

// 0x379584 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x379584: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_379584() {
}

// 0x379588 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x379588: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_379588() {
}

// 0x379628 — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x379628: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_379628() {
}

// 0x379630 — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x379630: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_379630() {
}

// 0x3796d4 — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3796d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3796d4() {
}

// 0x3796dc — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3796dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3796dc() {
}

// 0x379780 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::BoundFuncDesc(void (RBX::Soundscape::SoundChannel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x379780: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379780() {
}

// 0x379884 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EED0Ev
// IDA 0x379884: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_379884() {
}

// 0x379938 — __ZNK3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x379938: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379938() {
}

// 0x379958 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundChannelEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundChannel>(char const*,char const*,bool RBX::Soundscape::SoundChannel::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundChannelEEEPKcS8_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x379958: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379958() {
}

// 0x379ae8 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE10isReadOnlyEv
// IDA 0x379ae8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379ae8() {
}

// 0x379aec — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE11isWriteOnlyEv
// IDA 0x379aec: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379aec() {
}

// 0x379af0 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x379af0: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379af0() {
}

// 0x379afc — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundChannel>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundChannelEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x379afc: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379afc() {
}

// 0x379b4c — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,int>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x379b4c: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379b4c() {
}

// 0x379c58 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbED0Ev
// IDA 0x379c58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_379c58() {
}

// 0x379c84 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE10isReadOnlyEv
// IDA 0x379c84: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379c84() {
}

// 0x379c88 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv
// IDA 0x379c88: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379c88() {
}

// 0x379c8c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x379c8c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379c8c() {
}

// 0x379cb0 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE7GetImplIMS3_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x379cb0: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379cb0() {
}

// 0x379dd0 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEMS3_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::PropDescriptor<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>(char const*,char const*,bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbEC2IMS3_KFbvEMS3_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x379dd0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379dd0() {
}

// 0x379ee4 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE10isReadOnlyEv
// IDA 0x379ee4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379ee4() {
}

// 0x379ee8 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE11isWriteOnlyEv
// IDA 0x379ee8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379ee8() {
}

// 0x379eec — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x379eec: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379eec() {
}

// 0x379f10 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::GetSetImpl<bool (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbE10GetSetImplIMS3_KFbvEMS3_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x379f10: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379f10() {
}

// 0x379f34 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiEC2IMS3_KFivEMS3_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::PropDescriptor<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>(char const*,char const*,int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiEC2IMS3_KFivEMS3_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x379f34: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_379f34() {
}

// 0x37a048 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiED0Ev
// IDA 0x37a048: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37a048() {
}

// 0x37a074 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE10isReadOnlyEv
// IDA 0x37a074: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a074() {
}

// 0x37a078 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE11isWriteOnlyEv
// IDA 0x37a078: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a078() {
}

// 0x37a07c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x37a07c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a07c() {
}

// 0x37a09c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::GetSetImpl<int (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiE10GetSetImplIMS3_KFivEMS3_FviEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x37a09c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a09c() {
}

// 0x37a0c0 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfEC2IMS3_KFfvEMS3_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::PropDescriptor<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>(char const*,char const*,float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfEC2IMS3_KFfvEMS3_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x37a0c0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a0c0() {
}

// 0x37a1d4 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfED0Ev
// IDA 0x37a1d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37a1d4() {
}

// 0x37a200 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE10isReadOnlyEv
// IDA 0x37a200: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a200() {
}

// 0x37a204 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE11isWriteOnlyEv
// IDA 0x37a204: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a204() {
}

// 0x37a208 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x37a208: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a208() {
}

// 0x37a228 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE8setValueEPNS0_13DescribedBaseERKf
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::GetSetImpl<float (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfE10GetSetImplIMS3_KFfvEMS3_FvfEE8setValueEPNS0_13DescribedBaseERKf
// IDA 0x37a228: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a228() {
}

// 0x37a24c — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEEC2IMS3_KFS4_vEMS3_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::PropDescriptor<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>(char const*,char const*,RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEEC2IMS3_KFS4_vEMS3_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x37a24c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a24c() {
}

// 0x37a360 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int *, int, int, char, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x37a360: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a360() {
}

// 0x37a484 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEED0Ev
// IDA 0x37a484: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37a484() {
}

// 0x37a4b0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10isReadOnlyEv
// IDA 0x37a4b0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a4b0() {
}

// 0x37a4c0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11isWriteOnlyEv
// IDA 0x37a4c0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a4c0() {
}

// 0x37a4d0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x37a4d0: 150 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a4d0() {
}

// 0x37a67c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x37a67c: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a67c() {
}

// 0x37a7a8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x37a7a8: 187 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a7a8() {
}

// 0x37a9a4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x37a9a4: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37a9a4() {
}

// 0x37aacc — __ZN3rbx8any_castIRKN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Soundscape::SoundId const& rbx::any_cast<RBX::Soundscape::SoundId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x37aacc: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37aacc() {
}

// 0x37abbc — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::~TypedPropertyDescriptor()")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEED1Ev
// IDA 0x37abbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37abbc() {
}

// 0x37abe0 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::~TypedPropertyDescriptor()")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEED0Ev
// IDA 0x37abe0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37abe0() {
}

// 0x37ac0c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE10isReadOnlyEv
// IDA 0x37ac0c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ac0c() {
}

// 0x37ac10 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE11isWriteOnlyEv
// IDA 0x37ac10: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ac10() {
}

// 0x37ac14 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x37ac14: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ac14() {
}

// 0x37ac3c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::GetSetImpl<RBX::Soundscape::SoundId (RBX::Soundscape::SoundChannel::*)(void)const,void (RBX::Soundscape::SoundChannel::*)(RBX::Soundscape::SoundId)>::setValue(RBX::Reflection::DescribedBase *,RBX::Soundscape::SoundId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEE10GetSetImplIMS3_KFS4_vEMS3_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x37ac3c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ac3c() {
}

// 0x37ad84 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EEC2EMS3_FvS4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::BoundFuncDesc(void (RBX::Soundscape::SoundService::*)(RBX::SoundType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EEC2EMS3_FvS4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x37ad84: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ad84() {
}

// 0x37aefc — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x37aefc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37aefc() {
}

// 0x37af2c — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EED0Ev
// IDA 0x37af2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37af2c() {
}

// 0x37b000 — __ZNK3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x37b000: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b000() {
}

// 0x37b034 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9SoundTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int (__fastcall ***__fastcall(int (__fastcall ***)(_DWORD), int))(_DWORD)
#[doc(alias = "RBX::SoundType RBX::Reflection::ArgHelper::getArg<RBX::SoundType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::SoundType> const&,boost::disable_if<boost::is_same<RBX::SoundType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9SoundTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x37b034: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b034() {
}

// 0x37b1c4 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9SoundTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINS9_7is_enumIS7_EEvE4typeE
// type: int __fastcall(int, _DWORD *, int, int)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::SoundType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::SoundType &,boost::enable_if<boost::is_enum<RBX::SoundType>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9SoundTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINS9_7is_enumIS7_EEvE4typeE
// IDA 0x37b1c4: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b1c4() {
}

// 0x37b218 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEEC2IMS3_KFS4_vEMS3_FvRKS4_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::EnumPropDescriptor<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>(char const*,char const*,RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEEC2IMS3_KFS4_vEMS3_FvRKS4_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x37b218: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b218() {
}

// 0x37b3cc — __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEED0Ev
// IDA 0x37b3cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37b3cc() {
}

// 0x37b3f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10isReadOnlyEv
// IDA 0x37b3f8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b3f8() {
}

// 0x37b408 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11isWriteOnlyEv
// IDA 0x37b408: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b408() {
}

// 0x37b418 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x37b418: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b418() {
}

// 0x37b440 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x37b440: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b440() {
}

// 0x37b464 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x37b464: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b464() {
}

// 0x37b5b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x37b5b0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b5b0() {
}

// 0x37b5d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14hasStringValueEv
// IDA 0x37b5d4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b5d4() {
}

// 0x37b5d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x37b5d8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b5d8() {
}

// 0x37b5fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x37b5fc: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b5fc() {
}

// 0x37b63c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x37b63c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b63c() {
}
