//! Auto-generated skeletons for rbx-network — filler global ascending EA-sorted
//! Filter: RakNet|RBX::Network|Replicator (case-insensitive) -> 4797 funcs, 4797 already stubbed (0 remaining before batch); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0xdf78..0x109b8 | existing 16430 -> 16530 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_138::{EnumDescModel, RenderSettingsItem};

/// Host stand-ins for the `rbx::implementation::typed_holder<T>` singletons
/// behind `placement_any<Region3>` (IDA 0xe43c/0xe97c) — same shape as the
/// 0xc95c family in `generated_139`: guarded `{typeinfo, destruct_func,
/// construct_func}` statics; the host keeps only the identity tag.
/// (`FrameRateManagerMode` reuses `generated_139::FRAME_RATE_MANAGER_HOLDER`.)
pub struct EnumValueHolder {
    pub type_name: &'static str,
}

/// was: `typed_holder<GraphicsMode>::singleton()::s` (IDA 0xe43c).
pub static GRAPHICS_MODE_HOLDER: EnumValueHolder = EnumValueHolder {
    type_name: "RBX::CRenderSettings::GraphicsMode",
};

/// was: `typed_holder<AASamples>::singleton()::s` (IDA 0xe97c).
pub static AA_SAMPLES_HOLDER: EnumValueHolder = EnumValueHolder {
    type_name: "RBX::CRenderSettings::AASamples",
};

// 0xdf78 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_df78(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0xdf78: decompile: same convertToItem asserts as 0xc9d8 — value>=0 (enumconverter.h:273) + value<size (:274), item else 0 (0xdf8c..0xe012). Host pairs search matches at any density ([INFERENCE] on table layout only).
    if value >= 0 {
        if let Some((v, _)) = desc.pairs.iter().find(|(val, _)| *val == value) {
            return *v;
        }
    }
    0
}

// 0xe044 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings20FrameRateManagerModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode const& rbx::any_cast<RBX::CRenderSettings::FrameRateManagerMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_e044(value: i32) -> i32 {
    // IDA 0xe044: same any_cast template as 0xcaa4 (holder typeinfo check; bad_cast on mismatch; value slot on hit). Host slots are checked upstream — the bad_cast arm is documented, not rebuilt.
    value
}

// 0xe134 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
pub fn stub_e134(desc: &EnumDescModel, name: &str, out: &mut i32) -> bool {
    // IDA 0xe134: same convertToValue<Name> template as 0xcc34 (twin RB-tree lower_bounds; hit stores node value + 1, else 0). Host by_name merges both trees.
    if let Some(&v) = desc.by_name.get(name) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xe1b0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_e1b0() {
    // IDA 0xe1b0: D2 — same EnumDesc D2 template as 0xccb0 (two _M_erase + base dtor); host tables drop with Rust ownership.
}

// 0xe24c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_e24c(desc: &EnumDescModel, value: i32, out: &mut String) {
    // IDA 0xe24c: same convertToString-value template as 0xc76c (asserts + bound-checked item string, empty when out of range). Host pairs search is the same observable mapping at any density.
    out.clear();
    if value >= 0 {
        if let Some((_, name)) = desc.pairs.iter().find(|(v, _)| *v == value) {
            out.push_str(name);
        }
    }
}

// 0xe3ec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12GraphicsModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::GraphicsMode>(RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_e3ec(slot: &mut i32, value: i32) {
    // IDA 0xe3ec: same placement_any op= template as 0xc90c (singleton fetch + holder-match fast path, else destroy/store/install). The holder tag folds into the GraphicsMode typed_holder static; the host carries the POD value directly.
    *slot = value;
}

// 0xe43c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::singleton(void)")]
pub fn stub_e43c() -> &'static EnumValueHolder {
    // IDA 0xe43c: decompile: guard-checked static init {typeinfo, destruct_func, construct_func→dword_12217C0} (0xe456..0xe496); returns &s (0xe4a6); returns the holder static.
    &GRAPHICS_MODE_HOLDER
}

// 0xe4a8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::construct_func(char const*,char *)")]
pub fn stub_e4a8(dst: &mut i32, src: &i32) {
    // IDA 0xe4a8: same construct_func template as 0xc9c8 (POD copy arm) — int-sized enum copied into the slot.
    *dst = *src;
}

// 0xe4b4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12GraphicsModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::GraphicsMode>::destruct_func(char *)")]
pub fn stub_e4b4() {
    // IDA 0xe4b4: same destruct_func template as 0xc9d4 (empty body) — int-sized enum needs no teardown.
}

// 0xe4b8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
pub fn stub_e4b8(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0xe4b8: same convertToItem template as 0xc9d8 (asserts + in-range item else 0). Host pairs search matches at any density ([INFERENCE] on table layout only).
    if value >= 0 {
        if let Some((v, _)) = desc.pairs.iter().find(|(val, _)| *val == value) {
            return *v;
        }
    }
    0
}

// 0xe584 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12GraphicsModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::GraphicsMode const& rbx::any_cast<RBX::CRenderSettings::GraphicsMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_e584(value: i32) -> i32 {
    // IDA 0xe584: same any_cast template as 0xcaa4 (holder typeinfo check; bad_cast on mismatch; value slot on hit). Host slots are checked upstream — the bad_cast arm is documented, not rebuilt.
    value
}

// 0xe674 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
pub fn stub_e674(desc: &EnumDescModel, name: &str, out: &mut i32) -> bool {
    // IDA 0xe674: same convertToValue<Name> template as 0xcc34 (twin RB-tree lower_bounds; hit stores node value + 1, else 0). Host by_name merges both trees.
    if let Some(&v) = desc.by_name.get(name) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xe6f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_e6f0() {
    // IDA 0xe6f0: D2 — same EnumDesc D2 template as 0xccb0 (two _M_erase + base dtor); host tables drop with Rust ownership.
}

// 0xe78c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_e78c(desc: &EnumDescModel, value: i32, out: &mut String) {
    // IDA 0xe78c: decompile: same convertToString asserts as 0xc76c — value>=0 (:262) + value<size (:263), empty when out of range (0xe7c8..). Host pairs search is the same observable mapping at any density.
    out.clear();
    if value >= 0 {
        if let Some((_, name)) = desc.pairs.iter().find(|(v, _)| *v == value) {
            out.push_str(name);
        }
    }
}

// 0xe92c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings9AASamplesEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AASamples>(RBX::CRenderSettings::AASamples const&)")]
pub fn stub_e92c(slot: &mut i32, value: i32) {
    // IDA 0xe92c: same placement_any op= template as 0xc90c (singleton fetch + holder-match fast path, else destroy/store/install). The holder tag folds into the AASamples typed_holder static; the host carries the POD value directly.
    *slot = value;
}

// 0xe97c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::singleton(void)")]
pub fn stub_e97c() -> &'static EnumValueHolder {
    // IDA 0xe97c: same singleton template as 0xc95c (guard-checked static {typeinfo, destruct, construct}); returns the holder static.
    &AA_SAMPLES_HOLDER
}

// 0xe9e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::construct_func(char const*,char *)")]
pub fn stub_e9e8(dst: &mut i32, src: &i32) {
    // IDA 0xe9e8: same construct_func template as 0xc9c8 (POD copy arm) — int-sized enum copied into the slot.
    *dst = *src;
}

// 0xe9f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings9AASamplesEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AASamples>::destruct_func(char *)")]
pub fn stub_e9f4() {
    // IDA 0xe9f4: same destruct_func template as 0xc9d4 (empty body) — int-sized enum needs no teardown.
}

// 0xe9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
pub fn stub_e9f8(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0xe9f8: same convertToItem template as 0xc9d8 (asserts + in-range item else 0). Host pairs search matches at any density ([INFERENCE] on table layout only).
    if value >= 0 {
        if let Some((v, _)) = desc.pairs.iter().find(|(val, _)| *val == value) {
            return *v;
        }
    }
    0
}

// 0xeac4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings9AASamplesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::AASamples const& rbx::any_cast<RBX::CRenderSettings::AASamples const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_eac4(value: i32) -> i32 {
    // IDA 0xeac4: same any_cast template as 0xcaa4 (holder typeinfo check; bad_cast on mismatch; value slot on hit). Host slots are checked upstream — the bad_cast arm is documented, not rebuilt.
    value
}

// 0xebb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
pub fn stub_ebb4(desc: &EnumDescModel, name: &str, out: &mut i32) -> bool {
    // IDA 0xebb4: same convertToValue<Name> template as 0xcc34 (twin RB-tree lower_bounds; hit stores node value + 1, else 0). Host by_name merges both trees.
    if let Some(&v) = desc.by_name.get(name) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xec30 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_ec30() {
    // IDA 0xec30: D2 — same EnumDesc D2 template as 0xccb0 (two _M_erase + base dtor); host tables drop with Rust ownership.
}

/// was: `RBX::Name` handle for `sRenderSettings` behind `Name::declare` /
/// `doDeclare` / `callDoDeclare` (IDA 0xf1d8/0xf1dc) and
/// `Creator::getClassName` (IDA 0xedfc). The index is assigned by the live
/// name table; 0 is the null name (`[INFERENCE]` — the binary returns the
/// table slot, same carrier shape as core's `StatsName`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderSettingsName {
    pub index: u32,
}

static RENDER_SETTINGS_NAME_CELL: std::sync::LazyLock<RenderSettingsName> =
    std::sync::LazyLock::new(|| RenderSettingsName { index: 1 });

// 0xeccc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_eccc() {
    // IDA 0xeccc: Creator D2 — vtable reset to the Creator vtable (0xed1c), isConstructed assert shape, base dtors under SjLj guard; creator lifetime is engine-side, so the host keeps a faithful no-op shell.
    }

// 0xedfc — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_edfc() -> RenderSettingsName {
    // IDA 0xedfc: getClassName — wasConstructed() assert (isConstructed == 0x29A, 0xee10..0xee5c), boost::call_once(declare flag, callDoDeclare<sRenderSettings>) (0xee60..0xee78), tail-call doDeclare (0xee80). Returns the interned class name.
    stub_f1dc()}

// 0xee84 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv
// type: int __fastcall(int *)
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7Creator6createEv")]
pub fn stub_ee84() -> SharedPtr<RenderSettingsItem> {
    // IDA 0xee84: Creator::create — wasConstructed() assert (Object.h:231, 0xee98..0xeee2), Creatable::create (0xeeec), out = (px ? px + 32 : 0) with shared count (0xeef2..0xeefe); the +32 is the enable_shared_from_this owner offset, folded into Arc.
    stub_ef04()}

// 0xef04 — __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
pub fn stub_ef04() -> SharedPtr<RenderSettingsItem> {
    // IDA 0xef04: Creatable::create<CRenderSettingsItem> — operator new(0xC4) (0xef38), CRenderSettingsItem ctor (0xef5c), shared_ptr ctor with Creatable::Deleter (0xef6a). Arc::new is all three.
        SharedPtr::new(RenderSettingsItem::default())}

// 0xefb4 — __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_efb4(value: RenderSettingsItem) -> SharedPtr<RenderSettingsItem> {
    // IDA 0xefb4: shared_ptr(px, deleter) — *a1 = px (0xefba), shared_count ctor (0xefc0), _internal_accept_owner when px != 0 (0xefd0); px is always non-null from 0xef04, so Arc::new covers all three arms.
        SharedPtr::new(value)}

// 0xefd8 — __ZNK5boost6detail15sp_counted_base9use_countEv
// type: int __fastcall(boost::detail::sp_counted_base *this)
#[doc(alias = "boost::detail::sp_counted_base::use_count(void)const")]
pub fn stub_efd8(shared: &SharedPtr<RenderSettingsItem>) -> i32 {
    // IDA 0xefd8: use_count — spinlock_pool<1> slot lock (0xf01a..0xf020), load use_count_ (0xf032), unlock (0xf058..0xf078). Arc strong_count is the same observable.
        SharedPtr::strong_count(shared) as i32}

// 0xf098 — __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f098() {
    // IDA 0xf098: shared_count(px, deleter) — *a1 = 0, operator new(0x14) counted block with use = weak = 1 (0xf0fa..0xf0fe), vtable + px install (0xf104..0xf10c). The Arc control block is built at SharedPtr::new, so the host keeps a no-op shell.
    }

// 0xf198 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_f198() {
    // IDA 0xf198: sp_counted_impl_pd D1 — empty body; drop covers it.
    }

// 0xf19c — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_f19c() {
    // IDA 0xf19c: dispose — Instance::predelete(px) (0xf1a4), then the vtable+8 delete when px != 0 (0xf1aa..0xf1b8); disposal folds into Arc drop at the live owner.
    }

// 0xf1bc — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_f1bc() -> bool {
    // IDA 0xf1bc: get_deleter — typeinfo-name compare against "N3RBX9CreatableINS_8InstanceEE7DeleterE" (0xf1ce) gates the a1+16 slot return (0xf1c0..0xf1d2). This instantiation always carries the Creatable deleter, so the gate always passes; the host stores no tagged deleter.
        true}

// 0xf1d4 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_f1d4() -> bool {
    // IDA 0xf1d4: get_untyped_deleter — unconditional a1+16 slot return (0xf1d6); same always-present Creatable deleter as 0xf1bc.
        true}

// 0xf1d8 — __ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZ15sRenderSettingsEEEvv")]
pub fn stub_f1d8() -> RenderSettingsName {
    // IDA 0xf1d8: callDoDeclare<sRenderSettings> — thunk straight into doDeclare (decompile: single shim tail-call).
    stub_f1dc()}

// 0xf1dc — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")]
pub fn stub_f1dc() -> RenderSettingsName {
    // IDA 0xf1dc: doDeclare<sRenderSettings> — guarded once-init (__cxa_guard_acquire 0xf238, Name::declare(sRenderSettings) 0xf25e, release 0xf262), returns the static (0xf290). LazyLock is that guard.
    *RENDER_SETTINGS_NAME_CELL}

/// was: `FactoryProduct<CRenderSettingsItem,...>::creatorPrivate` — the
/// Creator singleton returned by `static_getCreator` (IDA 0xf500/0xf572).
/// Construction/registration (IDA 0xf2bc) lives engine-side; the host keeps
/// only the identity handle.
pub struct RenderSettingsCreator;

static RENDER_SETTINGS_CREATOR_PRIVATE: RenderSettingsCreator = RenderSettingsCreator;

// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_f2bc() {
    // IDA 0xf2bc: Creator C2 — vtable install (0xf2f2), call_once Name declare (0xf2f4) + doDeclare (0xf30a), creators-map insert with find/dup asserts (Object.h:244/:245, 0xf316..0xf468), isConstructed = 666 (0xf422); the factory registry lives engine-side — faithful no-op shell.
    }

// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_f500() -> &'static RenderSettingsCreator {
    // IDA 0xf500: static_getCreator — Creator::wasConstructed() assert (Object.h:282, 0xf510..0xf562); returns &creatorPrivate (0xf572).
        &RENDER_SETTINGS_CREATOR_PRIVATE}

// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
pub fn stub_f574() {
    // IDA 0xf574: signal::next — slot addref (0xf5c4..0xf5ce), static-mutex lock (0xf5f8..0xf608), slot op= advance (0xf61c), unlock + release (0xf638..); the next-slot cursor is emit-loop state folded into Signal::fire (cf. stub_b76c / RenderSettingsItem::emit_prop_changed). was: boost::intrusive_ptr<slot> -> rbx_core::SharedPtr.
    }

// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
pub fn stub_f6dc() -> bool {
    // IDA 0xf6dc: signal::on_error — &slot_exception_handler (0xf6f0); the nonnull gate (0xf6f6..0xf6fc) would invoke it (0xf6fe), else returns &handler (0xf702). The host installs no handler, so the gate never passes.
        false}

// 0xf704 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
pub fn stub_f704(xs: &mut Vec<(u16, u16)>, index: usize, v: (u16, u16)) {
    // IDA 0xf704: vector::insert_aux — finish bump (0xf72c), copy_backward shift (0xf734), value store (0xf738); Vec::insert covers both arms (the fast path is inlined at the caller, cf. 0xb740).
        xs.insert(index, v);}

// 0xf7e8 — __ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")]
pub fn stub_f7e8(n: usize) -> Vec<(u16, u16)> {
    // IDA 0xf7e8: _Vector_base::_M_allocate — __throw_bad_alloc when n >= 0x40000000 (0xf7f0..0xf7f2), else operator new(4 * n); with_capacity is the uninit-storage carrier (__cxa_throw -> panic!).
        if n >= 0x4000_0000 {
        panic!("bad_alloc");
    }
    Vec::with_capacity(n)}

// 0xf800 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")]
pub fn stub_f800(xs: &mut [(u16, u16)], first: usize, last: usize, result: usize) -> usize {
    // IDA 0xf800: copy_backward word-at-a-time loop (0xf800..0xf832), returns the adjusted result end (0xf834..0xf83a); copy_within is the overlapping-backward carrier (pairs are 4-byte words, same unit).
        let n = last - first;
    xs.copy_within(first..last, result - n);
    result - n}

// 0xf83c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_f83c() {
    // IDA 0xf83c: GlobalAdvancedSettingsItem D1 — four vtable installs (0xf85c..0xf86e), sing = 0 (0xf872), Instance dtor (0xf878); teardown folds into Rust drop.
    }

// 0xf87c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_f87c() {
    // IDA 0xf87c: GlobalAdvancedSettingsItem D0 — inlined D1 body (disasm 0xf87c..0xf892: this-save, vtable + sing refs, same stores as 0xf83c) + operator delete; drops with Rust ownership.
    }

// 0xf8c8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_f8c8() {
    // IDA 0xf8c8: Thn32 D1 — same teardown as 0xf83c with adjusted this (decompile 0xf8e8..0xf906: vtable stores at this-8/-5, Instance dtor at this-4, sing = 0 at 0xf900); drop covers it.
    }

// 0xf90c — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_f90c() {
    // IDA 0xf90c: Thn32 D0 — adjusted-this D1 body + operator delete (disasm 0xf90c..0xf922 head mirrors the D1 stores); drops with Rust ownership.
    }

// 0xf964 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_f964() {
    // IDA 0xf964: Thn36 D1 — same teardown stores with adjusted this (disasm 0xf964..0xf976: vtable + sing refs); drop covers it.
    }

// 0xf9a8 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_f9a8() {
    // IDA 0xf9a8: Thn36 D0 — adjusted-this D1 body + operator delete (disasm 0xf9a8..0xf9be head mirrors the D1 stores); drops with Rust ownership.
    }

/// was: `Described<CRenderSettingsItem>::classDescriptor()::describedClassDescriptor`
/// (IDA 0xfa00) — the once-init `ClassDescriptor("RenderSettings")` with the
/// `Instance` base; `__cxa_atexit` teardown stays engine-side, the host keeps
/// the once handle (`LazyLock` is the `__cxa_guard` pair).
pub struct RenderSettingsClass {
    pub name: &'static str,
}

static RENDER_SETTINGS_CLASS_CELL: std::sync::LazyLock<RenderSettingsClass> =
    std::sync::LazyLock::new(|| RenderSettingsClass { name: "RenderSettings" });

// 0xfa00 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_fa00() -> &'static RenderSettingsClass {
    // IDA 0xfa00: Described<CRenderSettingsItem>::classDescriptor — guarded once-init (0xfa5c): Instance base descriptor (0xfa68), ClassDescriptor("RenderSettings") (0xfaa0), atexit dtor (0xfabe); the descriptor registry lives engine-side, the host keeps the once handle.
        &RENDER_SETTINGS_CLASS_CELL}

// 0xfb1c — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_fb1c() {
    // IDA 0xfb1c: Described<CRenderSettingsItem> D1 — vtable/base teardown engine-side; host drops with Rust ownership.
    }

// 0xfb20 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_fb20() {
    // IDA 0xfb20: Described<CRenderSettingsItem> D0 — D1 body + operator delete; drops with Rust ownership.
    }

// 0xfb34 — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_fb34() {
    // IDA 0xfb34: Thn32 Described D1 — same teardown with adjusted this (cf. the GASI thn32 D1 at 0xf8c8); drop covers it.
    }

// 0xfb3c — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_fb3c() {
    // IDA 0xfb3c: Thn32 Described D0 — adjusted-this D1 body + operator delete (cf. 0xf90c); drops with Rust ownership.
    }

// 0xfb54 — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_fb54() {
    // IDA 0xfb54: Thn36 Described D1 — same teardown with adjusted this (cf. 0xf964); drop covers it.
    }

// 0xfb5c — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_fb5c() {
    // IDA 0xfb5c: Thn36 Described D0 — adjusted-this D1 body + operator delete (cf. 0xf9a8); drops with Rust ownership.
    }

// 0xfb74 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_fb74() {
    // IDA 0xfb74: PropDescriptor<CRenderSettingsItem,int> C2 — classDescriptor ensure (0xfb9c), operator new(0x14) GetSetImpl holder with getter/setter pair (0xfba2..0xfbd8), TypedPropertyDescriptor attach (0xfc1a), vtable install (0xfc38); the descriptor heap lives engine-side — faithful no-op shell.
    }

// 0xfc88 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
pub fn stub_fc88() {
    // IDA 0xfc88: PropDescriptor D0 — vtable reset + holder delete + operator delete; the descriptor heap lives engine-side, drops with Rust ownership.
    }

// 0xfcb4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const")]
pub fn stub_fcb4() -> bool {
    // IDA 0xfcb4: GetSetImpl::isReadOnly — returns 0 (0xfcb6); the property has both accessors.
        false}

// 0xfcb8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")]
pub fn stub_fcb8() -> bool {
    // IDA 0xfcb8: GetSetImpl::isWriteOnly — returns 0 (0xfcba); same dual-accessor note as 0xfcb4.
        false}

// 0xfcbc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_fcbc(get: impl Fn() -> u32) -> u32 {
    // IDA 0xfcbc: GetSetImpl::getValue — Described-36 item adjust when obj != 0 (0xfcbe..0xfcca), member-pointer decode (0xfcc0..0xfce4), getter invoke; the `unsigned (CRenderSettings::*)() const` travels as a closure (boost::function -> Box<dyn Fn>).
        get()}

// 0xfce8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_fce8(set: impl Fn(u32), value: u32) {
    // IDA 0xfce8: GetSetImpl::setValue — same adjust/decode (0xfcee..0xfd04), setter invoke with the value; the `void (CRenderSettingsItem::*)(unsigned)` travels as a closure.
        set(value);}

// 0xfd0c — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_fd0c() {
    // IDA 0xfd0c: BoundFuncDesc<CRenderSettingsItem,int(),0> C2 — classDescriptor ensure (0xfd32), FunctionDescriptor attach (0xfd52), vtable + member-fn pair + int return-type singleton (0xfd6e..0xfda2); the descriptor heap lives engine-side — faithful no-op shell.
    }

// 0xfe04 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
pub fn stub_fe04() {
    // IDA 0xfe04: BoundFuncDesc D0 — vtable reset + signature-item list _M_clear (cf. the D1 at 0xb4d0) + operator delete; drops with Rust ownership.
    }

// 0xfe30 — __ZNK3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_fe30(call: impl Fn() -> i32) -> i32 {
    // IDA 0xfe30: BoundFuncDesc::execute — this-36 item adjust when obj != 0 (0xfe38..0xfe3a), then Call0Helper::call with the stored member pair; dispatch folds into the closure.
        call()}

// 0xfe54 — __ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE
// type: int __fastcall(int, int (__fastcall *)(_DWORD), int, _DWORD *)
#[doc(alias = "RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_fe54(get: impl Fn() -> i32) -> i32 {
    // IDA 0xfe54: Call0Helper::call — member dispatch with virtual-adjust arm (0xfe5a..0xfe68), invoke (0xfe6c), int result wrapped into the placement_any Variant (0xfe72..0xfe80); the host carries the int directly.
        get()}

// 0xfe84 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_fe84() {
    // IDA 0xfe84: EnumPropDescriptor<CRenderSettingsItem,ResolutionPreset> C2 — classDescriptor ensure (0xfea8), EnumDesc singleton call_once + doGet (0xfec8..0xfecc), PropertyDescriptor attach; the descriptor heap lives engine-side — faithful no-op shell.
    }

/// Host carrier for the `XmlElement`/`XmlNameValuePair` value read by
/// `EnumPropDescriptor::readValue` (IDA 0x102cc): the image threads
/// `XmlElement` heaps through `getValue` overloads; the host passes the
/// already-extracted payload — nil, an int cell, a text cell, or anything
/// else (which readValue ignores past the diagnostics gate).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmlPropValue {
    Nil,
    Int(i32),
    Text(String),
    Other,
}

// 0x10038 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
pub fn stub_10038() {
    // IDA 0x10038: EnumPropDescriptor D0 — vtable reset (0x1004c), impl-holder delete when non-null (0x1004e..0x10054), operator delete; the descriptor heap lives engine-side, drops with Rust ownership.
    }

// 0x10064 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const")]
pub fn stub_10064() -> bool {
    // IDA 0x10064: EnumPropDescriptor::isReadOnly — forwards to the GetSetImpl slot (+44, 0x10070); the int impl answers 0 (cf. 0xfcb4).
        false}

// 0x10074 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const")]
pub fn stub_10074() -> bool {
    // IDA 0x10074: EnumPropDescriptor::isWriteOnly — forwards to the GetSetImpl slot (+44, 0x10080); the int impl answers 0 (cf. 0xfcb8).
        false}

// 0x10084 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_10084(a: i32, b: i32) -> bool {
    // IDA 0x10084: EnumPropDescriptor::equalValues — get(a2) vs get(a3) through the GetSetImpl slot (0x10094/0x100aa); the host compares carried values directly.
        a == b}

// 0x100ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_100ac(get: impl Fn() -> i32) -> i32 {
    // IDA 0x100ac: EnumPropDescriptor::getVariant — get via vtable+68 (0x100ba), int wrapped into the placement_any Variant (0x100c0..0x100ce); the host carries the int, the Variant wrap folds.
        get()}

// 0x100d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_100d0(value: i32, set: impl Fn(i32)) {
    // IDA 0x100d0: EnumPropDescriptor::setVariant — int-holder fast path via any_cast<int> (0x1014e..0x101cc), else Variant::convert<int> (0x1017c..0x1018e), then the +72 setter (0x101da); the host carries the int, extraction folds.
        set(value);}

// 0x10220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_10220(get: impl Fn() -> i32, set: impl Fn(i32)) {
    // IDA 0x10220: EnumPropDescriptor::copyValue — get through the impl slot (0x10232), set into dst (0x10242).
        let v = get();
    set(v);}

// 0x10244 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::hasStringValue(void)const")]
pub fn stub_10244() -> bool {
    // IDA 0x10244: EnumPropDescriptor::hasStringValue — returns 1 (0x10246).
        true}

// 0x10248 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_10248(desc: &EnumDescModel, get: impl Fn() -> i32, out: &mut String) {
    // IDA 0x10248: EnumPropDescriptor::getStringValue — get through the impl slot (0x1025a), EnumDesc::convertToString(value) (0x1026a); delegates to the 0xc76c port.
        crate::generated_139::stub_c76c(desc, get(), out);}

// 0x1026c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_1026c(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x1026c: EnumPropDescriptor::setStringValue — Name::lookup (0x1027e) + EnumDesc::convertToValue (0x1028c), set on hit (0x102a2), 1/0; delegates to the 0xcc34 port.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x102ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_102ac(get: impl Fn() -> i32) -> (u32, i32) {
    // IDA 0x102ac: EnumPropDescriptor::writeValue — get through the impl slot (0x102ba), pair clearValue (0x102c0), kind tag 5 + value store (0x102c6..0x102ca), returns 5; the host has no XmlElement heap, so the (kind, value) outputs travel as a pair.
        (5, get())}

// 0x102cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_102cc(desc: &EnumDescModel, value: XmlPropValue, set: impl Fn(i32)) {
    // IDA 0x102cc: EnumPropDescriptor::readValue — xsi:nil bails (0x102f0); int pair goes through setIntValue (0x10338..0x10348); string pair goes through Name::lookup + convertToValue + set (0x1037e..0x103b2) with the empty-text +64-with-0 fallback (0x103d4..0x10486); unconvertible text hits the diagnostics-gated ReleaseAssert(false) (Reflection.h:359, 0x103fc..0x104aa), documented here, not rebuilt.
        match value {
        XmlPropValue::Nil => {}
        XmlPropValue::Int(i) => {
            stub_10674(desc, i, &set);
        }
        XmlPropValue::Text(s) => {
            let mut v = 0;
            if crate::generated_139::stub_cc34(desc, &s, &mut v) {
                set(v);
            } else if s.is_empty() {
                set(0);
            }
        }
        XmlPropValue::Other => {}
    }}

// 0x1050c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1050c(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x1050c: EnumPropDescriptor::getIndexValue — get through the impl slot (0x1051c), EnumDesc::convertToIndex (0x1050e+); delegates to the 0x10604 port.
        stub_10604(desc, get())}

// 0x10528 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_10528(desc: &EnumDescModel, index: usize, set: impl Fn(i32)) -> bool {
    // IDA 0x10528: EnumPropDescriptor::setIndexValue — count(+40) > index gates legacy[index] (base +144, 0x1053a..0x10544), set on hit (0x1054e), 1/0; same table shape as convertToValue.
        if let Some(&v) = desc.legacy.get(index) {
        set(v);
        true
    } else {
        false
    }}

// 0x1055c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1055c(get: impl Fn() -> i32) -> i32 {
    // IDA 0x1055c: EnumPropDescriptor::getEnumValue — get through the impl slot (+8); the host carries the value.
        get()}

// 0x10564 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_10564(desc: &EnumDescModel, value: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x10564: EnumPropDescriptor::setEnumValue — find_if equalValue over the items (0x1058e), set on hit (0x10596..), 1/0; the host searches pairs.
        if desc.pairs.iter().any(|(v, _)| *v == value) {
        set(value);
        true
    } else {
        false
    }}

// 0x105b0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_105b0(desc: &EnumDescModel, get: impl Fn() -> i32) -> i32 {
    // IDA 0x105b0: EnumPropDescriptor::getEnumItem — get through the impl slot (0x105c2), EnumDesc::convertToItem (0x105ce); delegates to the 0xc9d8 port.
        crate::generated_139::stub_c9d8(desc, get())}

// 0x105d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_105d0(desc: &EnumDescModel, name: &str, set: impl Fn(i32)) -> bool {
    // IDA 0x105d0: EnumPropDescriptor::setStringValue(Name) — EnumDesc::convertToValue<Name> (0x105e6), set on hit (0x105fc), 1/0; same port as 0x1026c with the Name already interned.
        let mut v = 0;
    if crate::generated_139::stub_cc34(desc, name, &mut v) {
        set(v);
        true
    } else {
        false
    }}

// 0x10604 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToIndex(RBX::CRenderSettings::ResolutionPreset)const")]
pub fn stub_10604(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0x10604: EnumDesc::convertToIndex — assert value>=0 (enumconverter.h:350, 0x10618..0x1064e); in-range index-vector[value] (base +156, 0x1065e..0x1066e), else -1 (0x10666..0x10672). Host pairs-position search is the same mapping on dense tables ([INFERENCE] on table layout only).
        desc.pairs.iter().position(|(v, _)| *v == value).map(|p| p as i32).unwrap_or(-1)}

// 0x10674 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_10674(desc: &EnumDescModel, index: i32, set: impl Fn(i32)) -> bool {
    // IDA 0x10674: EnumPropDescriptor::setIntValue — index>=0 and in the item table (0x1067e..0x10690), entry != -1 (0x1069c), set (0x106a8..0x106aa), 1; else 0. Host pairs carry the entries with the same -1 sentinel.
        if index >= 0 {
        if let Some(&(v, _)) = desc.pairs.get(index as usize) {
            if v != -1 {
                set(v);
                return true;
            }
        }
    }
    false}

// 0x106b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isReadOnly(void)const")]
pub fn stub_106b4() -> bool {
    // IDA 0x106b4: GetSetImpl<ResolutionPreset>::isReadOnly — returns 0 (0x106b6); same dual-accessor shape as 0xfcb4.
        false}

// 0x106b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isWriteOnly(void)const")]
pub fn stub_106b8() -> bool {
    // IDA 0x106b8: GetSetImpl<ResolutionPreset>::isWriteOnly — returns 0 (0x106ba); same dual-accessor shape as 0xfcb8.
        false}

// 0x106bc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_106bc(get: impl Fn() -> i32) -> i32 {
    // IDA 0x106bc: GetSetImpl<ResolutionPreset>::getValue — same member-getter dispatch as 0xfcbc (adjust 0x106be..0x106ca, decode 0x106c0..0x106e4, invoke); the getter travels as a closure.
        get()}

// 0x106e8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::ResolutionPreset const&)const")]
pub fn stub_106e8(set: impl Fn(i32), value: i32) {
    // IDA 0x106e8: GetSetImpl<ResolutionPreset>::setValue — same member-setter dispatch as 0xfce8 (0x106ee..0x10704, invoke with value); the setter travels as a closure.
        set(value);}

// 0x1070c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_1070c() {
    // IDA 0x1070c: PropDescriptor<CRenderSettingsItem,bool> C2 — same template as the int C2 at 0xfb74 (classDescriptor ensure 0x1071e+, GetSetImpl holder, TypedProperty attach, vtable); the descriptor heap lives engine-side — faithful no-op shell.
    }

// 0x10820 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
pub fn stub_10820() {
    // IDA 0x10820: PropDescriptor<bool> D0 — same teardown as 0xfc88 (vtable reset + holder delete + operator delete); drops with Rust ownership.
    }

// 0x1084c — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isReadOnly(void)const")]
pub fn stub_1084c() -> bool {
    // IDA 0x1084c: GetSetImpl<bool>::isReadOnly — same 0-return shape as 0xfcb4; the property has both accessors.
        false}

// 0x10850 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_10850() -> bool {
    // IDA 0x10850: GetSetImpl<bool>::isWriteOnly — same 0-return shape as 0xfcb8.
        false}

// 0x10854 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_10854(get: impl Fn() -> bool) -> bool {
    // IDA 0x10854: GetSetImpl<bool>::getValue — same member-getter dispatch as 0xfcbc; the `bool (CRenderSettingsItem::*)() const` travels as a closure.
        get()}

// 0x10878 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItembE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::GetSetImpl<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_10878(set: impl Fn(bool), value: bool) {
    // IDA 0x10878: GetSetImpl<bool>::setValue — same member-setter dispatch as 0xfce8; the `void (CRenderSettingsItem::*)(bool)` travels as a closure.
        set(value);}

// 0x1089c — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>(char const*,char const*,int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_1089c() {
    // IDA 0x1089c: PropDescriptor<CRenderSettingsItem,int> C2 — same template as 0xfb74 (classDescriptor ensure, GetSetImpl holder with the int getter/setter pair, TypedProperty attach, vtable); descriptor heap engine-side — faithful no-op shell.
    }

// 0x109b0 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isReadOnly(void)const")]
pub fn stub_109b0() -> bool {
    // IDA 0x109b0: GetSetImpl<int>::isReadOnly — same 0-return shape as 0xfcb4.
        false}

// 0x109b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::isWriteOnly(void)const")]
pub fn stub_109b4() -> bool {
    // IDA 0x109b4: GetSetImpl<int>::isWriteOnly — same 0-return shape as 0xfcb8.
        false}

// 0x109b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_109b8(get: impl Fn() -> i32) -> i32 {
    // IDA 0x109b8: GetSetImpl<int>::getValue — same member-getter dispatch as 0xfcbc; the `int (CRenderSettings::*)() const` travels as a closure.
        get()}

