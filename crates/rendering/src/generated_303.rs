//! rendering shard 303 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 32840->32940 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 32840 before -> 32940 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x43d0e0 (lowest remaining 0x438dbc..0x43d0e0, next lowest 0x43d0f4)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x438dbc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueEmRNS0_7VariantE
pub fn stub_438dbc() -> ! {
    todo!("0x438dbc RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x438df0 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringEmRSs
pub fn stub_438df0() -> ! {
    todo!("0x438df0 RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToString(unsigned long,std::string &)const")
}

// 0x438f34 — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED1Ev
pub fn stub_438f34() -> ! {
    todo!("0x438f34 RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::~EnumDesc()")
}

// 0x438f38 — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED0Ev
pub fn stub_438f38() -> ! {
    todo!("0x438f38 RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::~EnumDesc()")
}

// 0x438fd8 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE6lookupEPKc
pub fn stub_438fd8() -> ! {
    todo!("0x438fd8 RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::lookup(char const*)const")
}

// 0x439008 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE6lookupERKNS0_7VariantE
pub fn stub_439008() -> ! {
    todo!("0x439008 RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x439028 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueEmRNS0_7VariantE
pub fn stub_439028() -> ! {
    todo!("0x439028 RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x43905c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE15convertToStringEmRSs
pub fn stub_43905c() -> ! {
    todo!("0x43905c RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToString(unsigned long,std::string &)const")
}

// 0x4391a0 — __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEED1Ev
pub fn stub_4391a0() -> ! {
    todo!("0x4391a0 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::~EnumDesc()")
}

// 0x4391a4 — __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEED0Ev
pub fn stub_4391a4() -> ! {
    todo!("0x4391a4 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::~EnumDesc()")
}

// 0x439244 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE6lookupEPKc
pub fn stub_439244() -> ! {
    todo!("0x439244 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::lookup(char const*)const")
}

// 0x439274 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE6lookupERKNS0_7VariantE
pub fn stub_439274() -> ! {
    todo!("0x439274 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x439294 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE14convertToValueEmRNS0_7VariantE
pub fn stub_439294() -> ! {
    todo!("0x439294 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4392c8 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE15convertToStringEmRSs
pub fn stub_4392c8() -> ! {
    todo!("0x4392c8 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToString(unsigned long,std::string &)const")
}

// 0x43940c — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToString(RBX::Instance::SaveFilter const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE15convertToStringERKS3_
pub fn stub_43940c() -> ! {
    todo!("0x43940c RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToString(RBX::Instance::SaveFilter const&)const")
}

// 0x4395ac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Instance10SaveFilterEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Instance::SaveFilter>(RBX::Instance::SaveFilter const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Instance10SaveFilterEEERS3_RKT_
pub fn stub_4395ac() -> ! {
    todo!("0x4395ac rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Instance::SaveFilter>(RBX::Instance::SaveFilter const&)")
}

// 0x4395fc — __ZN3rbx14implementation12typed_holderIN3RBX8Instance10SaveFilterEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Instance::SaveFilter>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX8Instance10SaveFilterEE9singletonEv
pub fn stub_4395fc() -> ! {
    todo!("0x4395fc rbx::implementation::typed_holder<RBX::Instance::SaveFilter>::singleton(void)")
}

// 0x439668 — __ZN3rbx14implementation12typed_holderIN3RBX8Instance10SaveFilterEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Instance::SaveFilter>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX8Instance10SaveFilterEE14construct_funcEPKcPc
pub fn stub_439668() -> ! {
    todo!("0x439668 rbx::implementation::typed_holder<RBX::Instance::SaveFilter>::construct_func(char const*,char *)")
}

// 0x439674 — __ZN3rbx14implementation12typed_holderIN3RBX8Instance10SaveFilterEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Instance::SaveFilter>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX8Instance10SaveFilterEE13destruct_funcEPc
pub fn stub_439674() -> ! {
    todo!("0x439674 rbx::implementation::typed_holder<RBX::Instance::SaveFilter>::destruct_func(char *)")
}

// 0x439678 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToItem(RBX::Instance::SaveFilter const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE13convertToItemERKS3_
pub fn stub_439678() -> ! {
    todo!("0x439678 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToItem(RBX::Instance::SaveFilter const&)const")
}

// 0x439744 — __ZN3rbx8any_castIRKN3RBX8Instance10SaveFilterENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Instance::SaveFilter const& rbx::any_cast<RBX::Instance::SaveFilter const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX8Instance10SaveFilterENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_439744() -> ! {
    todo!("0x439744 RBX::Instance::SaveFilter const& rbx::any_cast<RBX::Instance::SaveFilter const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x439834 — __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToValue(RBX::Name const&,RBX::Instance::SaveFilter&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE14convertToValueERKNS_4NameERS3_
pub fn stub_439834() -> ! {
    todo!("0x439834 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::convertToValue(RBX::Name const&,RBX::Instance::SaveFilter&)const")
}

// 0x4398b0 — __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEED2Ev
pub fn stub_4398b0() -> ! {
    todo!("0x4398b0 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::~EnumDesc()")
}

// 0x439a84 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToString(RBX::DataModel::GearType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE15convertToStringERKS3_
pub fn stub_439a84() -> ! {
    todo!("0x439a84 RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToString(RBX::DataModel::GearType const&)const")
}

// 0x439c24 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel8GearTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::GearType>(RBX::DataModel::GearType const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel8GearTypeEEERS3_RKT_
pub fn stub_439c24() -> ! {
    todo!("0x439c24 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::GearType>(RBX::DataModel::GearType const&)")
}

// 0x439c74 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearType>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE9singletonEv
pub fn stub_439c74() -> ! {
    todo!("0x439c74 rbx::implementation::typed_holder<RBX::DataModel::GearType>::singleton(void)")
}

// 0x439ce0 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearType>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE14construct_funcEPKcPc
pub fn stub_439ce0() -> ! {
    todo!("0x439ce0 rbx::implementation::typed_holder<RBX::DataModel::GearType>::construct_func(char const*,char *)")
}

// 0x439cec — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearType>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE13destruct_funcEPc
pub fn stub_439cec() -> ! {
    todo!("0x439cec rbx::implementation::typed_holder<RBX::DataModel::GearType>::destruct_func(char *)")
}

// 0x439cf0 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToItem(RBX::DataModel::GearType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE13convertToItemERKS3_
pub fn stub_439cf0() -> ! {
    todo!("0x439cf0 RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToItem(RBX::DataModel::GearType const&)const")
}

// 0x439dbc — __ZN3rbx8any_castIRKN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::DataModel::GearType const& rbx::any_cast<RBX::DataModel::GearType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_439dbc() -> ! {
    todo!("0x439dbc RBX::DataModel::GearType const& rbx::any_cast<RBX::DataModel::GearType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x439eac — __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToValue(RBX::Name const&,RBX::DataModel::GearType&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE14convertToValueERKNS_4NameERS3_
pub fn stub_439eac() -> ! {
    todo!("0x439eac RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::convertToValue(RBX::Name const&,RBX::DataModel::GearType&)const")
}

// 0x439f28 — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEED2Ev
pub fn stub_439f28() -> ! {
    todo!("0x439f28 RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::~EnumDesc()")
}

// 0x43a0fc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToString(RBX::DataModel::GearGenreSetting const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE15convertToStringERKS3_
pub fn stub_43a0fc() -> ! {
    todo!("0x43a0fc RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToString(RBX::DataModel::GearGenreSetting const&)const")
}

// 0x43a29c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel16GearGenreSettingEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::GearGenreSetting>(RBX::DataModel::GearGenreSetting const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel16GearGenreSettingEEERS3_RKT_
pub fn stub_43a29c() -> ! {
    todo!("0x43a29c rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::GearGenreSetting>(RBX::DataModel::GearGenreSetting const&)")
}

// 0x43a2ec — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE9singletonEv
pub fn stub_43a2ec() -> ! {
    todo!("0x43a2ec rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::singleton(void)")
}

// 0x43a358 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE14construct_funcEPKcPc
pub fn stub_43a358() -> ! {
    todo!("0x43a358 rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::construct_func(char const*,char *)")
}

// 0x43a364 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE13destruct_funcEPc
pub fn stub_43a364() -> ! {
    todo!("0x43a364 rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::destruct_func(char *)")
}

// 0x43a368 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToItem(RBX::DataModel::GearGenreSetting const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE13convertToItemERKS3_
pub fn stub_43a368() -> ! {
    todo!("0x43a368 RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToItem(RBX::DataModel::GearGenreSetting const&)const")
}

// 0x43a434 — __ZN3rbx8any_castIRKN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::DataModel::GearGenreSetting const& rbx::any_cast<RBX::DataModel::GearGenreSetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_43a434() -> ! {
    todo!("0x43a434 RBX::DataModel::GearGenreSetting const& rbx::any_cast<RBX::DataModel::GearGenreSetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x43a524 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToValue(RBX::Name const&,RBX::DataModel::GearGenreSetting&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToValueERKNS_4NameERS3_
pub fn stub_43a524() -> ! {
    todo!("0x43a524 RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToValue(RBX::Name const&,RBX::DataModel::GearGenreSetting&)const")
}

// 0x43a5a0 — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED2Ev
pub fn stub_43a5a0() -> ! {
    todo!("0x43a5a0 RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")
}

// 0x43a774 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToString(RBX::DataModel::Genre const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringERKS3_
pub fn stub_43a774() -> ! {
    todo!("0x43a774 RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToString(RBX::DataModel::Genre const&)const")
}

// 0x43a914 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel5GenreEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::Genre>(RBX::DataModel::Genre const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel5GenreEEERS3_RKT_
pub fn stub_43a914() -> ! {
    todo!("0x43a914 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::Genre>(RBX::DataModel::Genre const&)")
}

// 0x43a964 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE9singletonEv
pub fn stub_43a964() -> ! {
    todo!("0x43a964 rbx::implementation::typed_holder<RBX::DataModel::Genre>::singleton(void)")
}

// 0x43a9d0 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE14construct_funcEPKcPc
pub fn stub_43a9d0() -> ! {
    todo!("0x43a9d0 rbx::implementation::typed_holder<RBX::DataModel::Genre>::construct_func(char const*,char *)")
}

// 0x43a9dc — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE13destruct_funcEPc
pub fn stub_43a9dc() -> ! {
    todo!("0x43a9dc rbx::implementation::typed_holder<RBX::DataModel::Genre>::destruct_func(char *)")
}

// 0x43a9e0 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToItem(RBX::DataModel::Genre const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE13convertToItemERKS3_
pub fn stub_43a9e0() -> ! {
    todo!("0x43a9e0 RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToItem(RBX::DataModel::Genre const&)const")
}

// 0x43aaac — __ZN3rbx8any_castIRKN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::DataModel::Genre const& rbx::any_cast<RBX::DataModel::Genre const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_43aaac() -> ! {
    todo!("0x43aaac RBX::DataModel::Genre const& rbx::any_cast<RBX::DataModel::Genre const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x43ab9c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToValue(RBX::Name const&,RBX::DataModel::Genre&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueERKNS_4NameERS3_
pub fn stub_43ab9c() -> ! {
    todo!("0x43ab9c RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToValue(RBX::Name const&,RBX::DataModel::Genre&)const")
}

// 0x43ac18 — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED2Ev
pub fn stub_43ac18() -> ! {
    todo!("0x43ac18 RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::~EnumDesc()")
}

// 0x43adec — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToString(RBX::DataModel::CreatorType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringERKS3_
pub fn stub_43adec() -> ! {
    todo!("0x43adec RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToString(RBX::DataModel::CreatorType const&)const")
}

// 0x43af8c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel11CreatorTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::CreatorType>(RBX::DataModel::CreatorType const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel11CreatorTypeEEERS3_RKT_
pub fn stub_43af8c() -> ! {
    todo!("0x43af8c rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::CreatorType>(RBX::DataModel::CreatorType const&)")
}

// 0x43afdc — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::CreatorType>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE9singletonEv
pub fn stub_43afdc() -> ! {
    todo!("0x43afdc rbx::implementation::typed_holder<RBX::DataModel::CreatorType>::singleton(void)")
}

// 0x43b048 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::CreatorType>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE14construct_funcEPKcPc
pub fn stub_43b048() -> ! {
    todo!("0x43b048 rbx::implementation::typed_holder<RBX::DataModel::CreatorType>::construct_func(char const*,char *)")
}

// 0x43b054 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::CreatorType>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE13destruct_funcEPc
pub fn stub_43b054() -> ! {
    todo!("0x43b054 rbx::implementation::typed_holder<RBX::DataModel::CreatorType>::destruct_func(char *)")
}

// 0x43b058 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToItem(RBX::DataModel::CreatorType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE13convertToItemERKS3_
pub fn stub_43b058() -> ! {
    todo!("0x43b058 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToItem(RBX::DataModel::CreatorType const&)const")
}

// 0x43b124 — __ZN3rbx8any_castIRKN3RBX9DataModel11CreatorTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::DataModel::CreatorType const& rbx::any_cast<RBX::DataModel::CreatorType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX9DataModel11CreatorTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_43b124() -> ! {
    todo!("0x43b124 RBX::DataModel::CreatorType const& rbx::any_cast<RBX::DataModel::CreatorType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x43b214 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToValue(RBX::Name const&,RBX::DataModel::CreatorType&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToValueERKNS_4NameERS3_
pub fn stub_43b214() -> ! {
    todo!("0x43b214 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToValue(RBX::Name const&,RBX::DataModel::CreatorType&)const")
}

// 0x43b290 — __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEED2Ev
pub fn stub_43b290() -> ! {
    todo!("0x43b290 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::~EnumDesc()")
}

// 0x43b464 — __ZN3RBX4Name13callDoDeclareILZNS_10sDataModelEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sDataModelEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sDataModelEEEEvv
pub fn stub_43b464() -> ! {
    todo!("0x43b464 __ZN3RBX4Name13callDoDeclareILZNS_10sDataModelEEEEvv")
}

// 0x43b468 — __ZN3RBX4Name9doDeclareILZNS_10sDataModelEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sDataModelEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sDataModelEEEERKS0_v
pub fn stub_43b468() -> ! {
    todo!("0x43b468 __ZN3RBX4Name9doDeclareILZNS_10sDataModelEEEERKS0_v")
}

// 0x43b548 — __ZNK3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7Creator6createEv
pub fn stub_43b548() -> ! {
    todo!("0x43b548 __ZNK3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7Creator6createEv")
}

// 0x43b690 — __ZN5boost6detail12shared_countC2IPN3RBX11LocalScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX11LocalScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_43b690() -> ! {
    todo!("0x43b690 boost::detail::shared_count::shared_count<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x43b798 — __ZN3RBX4Name13callDoDeclareILZNS_15sCoreGuiServiceEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sCoreGuiServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_15sCoreGuiServiceEEEEvv
pub fn stub_43b798() -> ! {
    todo!("0x43b798 __ZN3RBX4Name13callDoDeclareILZNS_15sCoreGuiServiceEEEEvv")
}

// 0x43b7a0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14CoreGuiServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::CoreGuiService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_14CoreGuiServiceEEEvv
pub fn stub_43b7a0() -> ! {
    todo!("0x43b7a0 void RBX::ServiceProvider::callDoGetClassIndex<RBX::CoreGuiService>(void)")
}

// 0x43b7a4 — __ZN3rbx7signals6signalIFvRKSsEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string const&)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvRKSsEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
pub fn stub_43b7a4() -> ! {
    todo!("0x43b7a4 rbx::signals::signal<void ()(std::string const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string const&)>::slot> &)")
}

// 0x43b904 — __ZN3rbx7signals6signalIFvRKSsEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvRKSsEE8on_errorERSt9exception
pub fn stub_43b904() -> ! {
    todo!("0x43b904 rbx::signals::signal<void ()(std::string const&)>::on_error(std::exception &)")
}

// 0x43b92c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKSsEE4slotEEaSERKS9_
// type: int *__fastcall(int *, _DWORD *)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string const&)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKSsEE4slotEEaSERKS9_
pub fn stub_43b92c() -> ! {
    todo!("0x43b92c boost::intrusive_ptr<rbx::signals::signal<void ()(std::string const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string const&)>::slot> const&)")
}

// 0x43b950 — __ZN3rbx7signals6signalIFvRKSsEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKSsEE22safe_static_init_mutexEv
pub fn stub_43b950() -> ! {
    todo!("0x43b950 rbx::signals::signal<void ()(std::string const&)>::safe_static_init_mutex(void)")
}

// 0x43b954 — __ZN3rbx7signals6signalIFvRKSsEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKSsEE24safe_static_do_get_mutexEv
pub fn stub_43b954() -> ! {
    todo!("0x43b954 rbx::signals::signal<void ()(std::string const&)>::safe_static_do_get_mutex(void)")
}

// 0x43ba50 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4nextERNS2_13intrusive_ptrINSC_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4nextERNS2_13intrusive_ptrINSC_4slotEEE
pub fn stub_43ba50() -> ! {
    todo!("0x43ba50 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot> &)")
}

// 0x43bbb0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
pub fn stub_43bbb0() -> ! {
    todo!("0x43bbb0 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")
}

// 0x43bbd8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotEEaSERKSF_
// type: int *__fastcall(int *, _DWORD *)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotEEaSERKSF_
pub fn stub_43bbd8() -> ! {
    todo!("0x43bbd8 boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot> const&)")
}

// 0x43bbfc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE22safe_static_init_mutexEv
pub fn stub_43bbfc() -> ! {
    todo!("0x43bbfc rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::safe_static_init_mutex(void)")
}

// 0x43bc00 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE24safe_static_do_get_mutexEv
pub fn stub_43bc00() -> ! {
    todo!("0x43bc00 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::safe_static_do_get_mutex(void)")
}

// 0x43bcf8 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEENS2_IPdEEEclIPFvS5_NS_10shared_ptrIKNS3_13TaskScheduler3JobEEERSsdSB_ENS0_5list1IRSJ_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int, sp_counted_base **, int *, int, int, int), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::operator()<void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&> &,int)")]
// was: __ZN5boost3_bi5list5INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEENS2_IPdEEEclIPFvS5_NS_10shared_ptrIKNS3_13TaskScheduler3JobEEERSsdSB_ENS0_5list1IRSJ_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_43bcf8() -> ! {
    todo!("0x43bcf8 void boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::operator()<void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *) &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&> &,int)")
}

// 0x43bde0 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEENS2_IPdEEEC2ES6_S8_S9_SA_SC_
// type: int __fastcall(int, int, std::string *, int, int, int)
#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::list5(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>)")]
// was: __ZN5boost3_bi5list5INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEENS2_IPdEEEC2ES6_S8_S9_SA_SC_
pub fn stub_43bde0() -> ! {
    todo!("0x43bde0 boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::list5(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>)")
}

// 0x43bf10 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEENS2_IPdEEEC2ES6_S8_S9_SA_SC_
// type: int __fastcall(int, int, std::string *, int, int, int)
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::storage5(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>)")]
// was: __ZN5boost3_bi8storage5INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEENS2_IPdEEEC2ES6_S8_S9_SA_SC_
pub fn stub_43bf10() -> ! {
    todo!("0x43bf10 boost::_bi::storage5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>::storage5(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>)")
}

// 0x43c044 — __ZN5boost3_bi8storage4INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEEEC2ES6_S8_S9_SA_
// type: _DWORD *__fastcall(_DWORD *, int, std::string *, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>>::storage4(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>)")]
// was: __ZN5boost3_bi8storage4INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_ISsEENS2_IdEEEC2ES6_S8_S9_SA_
pub fn stub_43c044() -> ! {
    todo!("0x43c044 boost::_bi::storage4<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>>::storage4(boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>)")
}

// 0x43c178 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptContextEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptContext>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptContextEEEmv
pub fn stub_43c178() -> ! {
    todo!("0x43c178 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptContext>(void)")
}

// 0x43c250 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_IPSt6vectorINS3_10Reflection7VariantESaISB_EEEEEclIPFvS5_NS_10shared_ptrIKNS3_13TaskScheduler3JobEEESE_ENS0_5list1IRSM_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int, sp_counted_base **, int), const shared_count **)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *>>::operator()<void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX9DataModelEEENS_3argILi1EEENS2_IPSt6vectorINS3_10Reflection7VariantESaISB_EEEEEclIPFvS5_NS_10shared_ptrIKNS3_13TaskScheduler3JobEEESE_ENS0_5list1IRSM_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_43c250() -> ! {
    todo!("0x43c250 void boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *>>::operator()<void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *),boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *) &,boost::_bi::list1<boost::shared_ptr<RBX::TaskScheduler::Job const>&> &,int)")
}

// 0x43c330 — __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKfEERNS_12basic_formatIT_T0_T1_EESD_T2_
// type: int __fastcall(int, int)
#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,float const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,float const&)")]
// was: __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKfEERNS_12basic_formatIT_T0_T1_EESD_T2_
pub fn stub_43c330() -> ! {
    todo!("0x43c330 boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,float const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,float const&)")
}

// 0x43c38c — __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE5clearEv
// type: int *__fastcall(int *)
#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::clear(void)")]
// was: __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE5clearEv
pub fn stub_43c38c() -> ! {
    todo!("0x43c38c boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::clear(void)")
}

// 0x43c450 — __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKfEEvRNS_12basic_formatIT_T0_T1_EET2_
// type: void __fastcall(__int64 *, int)
#[doc(alias = "void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,float const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,float const&)")]
// was: __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKfEEvRNS_12basic_formatIT_T0_T1_EET2_
pub fn stub_43c450() -> ! {
    todo!("0x43c450 void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,float const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,float const&)")
}

// 0x43c570 — __ZN5boost15throw_exceptionINS_2io13too_many_argsEEEvRKT_
// type: void __fastcall __noreturn(int)
#[doc(alias = "void boost::throw_exception<boost::io::too_many_args>(boost::io::too_many_args const&)")]
// was: __ZN5boost15throw_exceptionINS_2io13too_many_argsEEEvRKT_
pub fn stub_43c570() -> ! {
    todo!("0x43c570 void boost::throw_exception<boost::io::too_many_args>(boost::io::too_many_args const&)")
}

// 0x43c660 — __ZN5boost2io13too_many_argsD1Ev
// type: void __fastcall(std::exception *this)
#[doc(alias = "boost::io::too_many_args::~too_many_args()")]
// was: __ZN5boost2io13too_many_argsD1Ev
pub fn stub_43c660() -> ! {
    todo!("0x43c660 boost::io::too_many_args::~too_many_args()")
}

// 0x43c664 — __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKfEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale
// type: void __fastcall(float *, int, std::string *, int, int)
#[doc(alias = "void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,float const&>(float const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")]
// was: __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKfEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale
pub fn stub_43c664() -> ! {
    todo!("0x43c664 void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,float const&>(float const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")
}

// 0x43cb88 — __ZN5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEEC1EPNS0_18basic_altstringbufIcS3_S4_EE
// type: _DWORD *__fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::basic_oaltstringstream(boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *)")]
// was: __ZN5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEEC1EPNS0_18basic_altstringbufIcS3_S4_EE
pub fn stub_43cb88() -> ! {
    todo!("0x43cb88 boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::basic_oaltstringstream(boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *)")
}

// 0x43ccd8 — __ZNK5boost2io6detail19stream_format_stateIcSt11char_traitsIcEE8apply_onERSt9basic_iosIcS4_EPSt6locale
// type: void __fastcall(int *, int, int)
#[doc(alias = "boost::io::detail::stream_format_state<char,std::char_traits<char>>::apply_on(std::basic_ios<char,std::char_traits<char>> &,std::locale *)const")]
// was: __ZNK5boost2io6detail19stream_format_stateIcSt11char_traitsIcEE8apply_onERSt9basic_iosIcS4_EPSt6locale
pub fn stub_43ccd8() -> ! {
    todo!("0x43ccd8 boost::io::detail::stream_format_state<char,std::char_traits<char>>::apply_on(std::basic_ios<char,std::char_traits<char>> &,std::locale *)const")
}

// 0x43cd68 — __ZN5boost2io6detail6mk_strIcSt11char_traitsIcESaIcEEEvRSbIT_T0_T1_EPKS6_NS9_9size_typeEiS6_St13_Ios_FmtflagsS6_b
// type: int __fastcall(std::string *, const char *, unsigned int, signed int, int, int, char, int)
#[doc(alias = "void boost::io::detail::mk_str<char,std::char_traits<char>,std::allocator<char>>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> &,char const*,std::basic_string<char,std::char_traits<char>,std::allocator<char>>::size_type,int,char,std::_Ios_Fmtflags,char,bool)")]
// was: __ZN5boost2io6detail6mk_strIcSt11char_traitsIcESaIcEEEvRSbIT_T0_T1_EPKS6_NS9_9size_typeEiS6_St13_Ios_FmtflagsS6_b
pub fn stub_43cd68() -> ! {
    todo!("0x43cd68 void boost::io::detail::mk_str<char,std::char_traits<char>,std::allocator<char>>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> &,char const*,std::basic_string<char,std::char_traits<char>,std::allocator<char>>::size_type,int,char,std::_Ios_Fmtflags,char,bool)")
}

// 0x43ce40 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE12clear_bufferEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::clear_buffer(void)")]
// was: __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE12clear_bufferEv
pub fn stub_43ce40() -> ! {
    todo!("0x43ce40 boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::clear_buffer(void)")
}

// 0x43cf00 — __ZN5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_oaltstringstream()")]
// was: __ZN5boost2io22basic_oaltstringstreamIcSt11char_traitsIcESaIcEED0Ev
pub fn stub_43cf00() -> ! {
    todo!("0x43cf00 boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_oaltstringstream()")
}

// 0x43cfd4 — __ZNK5boost15optional_detail13optional_baseISt6localeE14is_initializedEv
// type: int __fastcall(unsigned __int8 *)
#[doc(alias = "boost::optional_detail::optional_base<std::locale>::is_initialized(void)const")]
// was: __ZNK5boost15optional_detail13optional_baseISt6localeE14is_initializedEv
pub fn stub_43cfd4() -> ! {
    todo!("0x43cfd4 boost::optional_detail::optional_base<std::locale>::is_initialized(void)const")
}

// 0x43cfd8 — __ZN5boost6detail12shared_countC2IPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS3_22basic_oaltstringstreamIcS6_S7_E5No_OpEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>(boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op)")]
// was: __ZN5boost6detail12shared_countC2IPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS3_22basic_oaltstringstreamIcS6_S7_E5No_OpEEET_T0_
pub fn stub_43cfd8() -> ! {
    todo!("0x43cfd8 boost::detail::shared_count::shared_count<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>(boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op)")
}

// 0x43d0b8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEED1Ev
pub fn stub_43d0b8() -> ! {
    todo!("0x43d0b8 boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::~sp_counted_impl_pd()")
}

// 0x43d0bc — __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEED0Ev
pub fn stub_43d0bc() -> ! {
    todo!("0x43d0bc boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::~sp_counted_impl_pd()")
}

// 0x43d0c0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE7disposeEv
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE7disposeEv
pub fn stub_43d0c0() -> ! {
    todo!("0x43d0c0 boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::dispose(void)")
}

// 0x43d0c4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE11get_deleterERKSt9type_info
pub fn stub_43d0c4() -> ! {
    todo!("0x43d0c4 boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::get_deleter(std::type_info const&)")
}

// 0x43d0dc — __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_2io18basic_altstringbufIcSt11char_traitsIcESaIcEEENS2_22basic_oaltstringstreamIcS5_S6_E5No_OpEE19get_untyped_deleterEv
pub fn stub_43d0dc() -> ! {
    todo!("0x43d0dc boost::detail::sp_counted_impl_pd<boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>> *,boost::io::basic_oaltstringstream<char,std::char_traits<char>,std::allocator<char>>::No_Op>::get_untyped_deleter(void)")
}

// 0x43d0e0 — __ZN5boost2io13too_many_argsD0Ev
// type: void __fastcall(std::exception *this)
#[doc(alias = "boost::io::too_many_args::~too_many_args()")]
// was: __ZN5boost2io13too_many_argsD0Ev
pub fn stub_43d0e0() -> ! {
    todo!("0x43d0e0 boost::io::too_many_args::~too_many_args()")
}
