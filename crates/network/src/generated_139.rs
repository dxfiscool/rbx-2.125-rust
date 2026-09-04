//! Auto-generated skeletons for rbx-network — filler global ascending EA-sorted

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_138::{EnumDescModel, RenderSettingsItem};

// 0xb4d0 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
pub fn stub_b4d0() {
    // IDA 0xb4d0: vtable reset to off_1222248 (0xb4e8) + signature-item list
    // _M_clear (0xb4ec); the host holds no image heap.
}

// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
pub fn stub_b4f4(item: &RenderSettingsItem) -> u32 {
    // IDA 0xb4f4: return this[16] (0xb4f6).
    item.texture_cache_size
}

// 0xb4f8 — __ZNK3RBX15CRenderSettings16getMeshCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getMeshCacheSize(void)const")]
pub fn stub_b4f8(item: &RenderSettingsItem) -> u32 {
    // IDA 0xb4f8: return this[17] (0xb4fa).
    item.mesh_cache_size
}

// 0xb4fc — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
pub fn stub_b4fc() {
    // IDA 0xb4fc: base Instance C2 (0xb51e), vtable installs (0xb54e..0xb564)
    // and classDescriptor wiring (0xb584..); the host keeps no base-class
    // image state, so the free-function ctor folds to a no-op.
}

// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
pub fn stub_b740(xs: &mut Vec<(u16, u16)>, v: (u16, u16)) {
    // IDA 0xb740: fast-path append while finish != end (0xb74c..0xb75c),
    // _M_insert_aux on full (0xb766); Vec::push covers both arms.
    xs.push(v);
}

// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b76c(item: &mut RenderSettingsItem, desc: u32) {
    // IDA 0xb76c: guarded (0xb79c) next-slot walk firing each slot with the
    // descriptor (0xb7e6..0xb80a); the FLog::SignalPrints branch (0xb7ce..)
    // is diagnostics only. Host dispatch lives in emit_prop_changed.
    item.emit_prop_changed(desc);
}

// 0xb8b0 — __ZNK3RBX15CRenderSettings21getEagerBulkExecutionEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getEagerBulkExecution(void)const")]
pub fn stub_b8b0(item: &RenderSettingsItem) -> bool {
    // IDA 0xb8b0: return byte at +61 (0xb8b4).
    item.eager_bulk_execution
}

// 0xb8b8 — __ZN19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8b8() {
    // IDA 0xb8b8: thunk tail-calling the D2 dtor; Rust ownership covers it.
}

// 0xb8bc — __ZN19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8bc() {
    // IDA 0xb8bc: D2 dtor (0xb8c2) + operator delete (0xb8cc); the host
    // drops the value with Rust ownership instead.
}

// 0xb8d0 — __ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_b8d0() -> &'static str {
    // IDA 0xb8d0: static_getCreator (0xb8d4) + Creator::getClassName shim;
    // the factory yields the class name literal.
    "CRenderSettingsItem"
}

// 0xb8e0 — __ZThn32_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8e0() {
    // IDA 0xb8e0: Thn32 thunk: this -= 32 (0xb8e2), then the D1 dtor;
    // Rust ownership covers the teardown.
}

// 0xb8e8 — __ZThn32_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b8e8() {
    // IDA 0xb8e8: Thn32 thunk: this -= 32 (0xb8ea), D2 dtor (0xb8f2) +
    // operator delete (0xb8fc); the host drops with Rust ownership.
}

// 0xb900 — __ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE12getClassNameEv")]
pub fn stub_b900() -> &'static str {
    // IDA 0xb900: Thn32 getClassName: static_getCreator (0xb904) +
    // Creator::getClassName shim; same literal as 0xb8d0.
    "CRenderSettingsItem"
}

// 0xb910 — __ZThn36_N19CRenderSettingsItemD1Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b910() {
    // IDA 0xb910: Thn36 thunk: this -= 36 (0xb912), then the D1 dtor;
    // Rust ownership covers the teardown.
}

// 0xb918 — __ZThn36_N19CRenderSettingsItemD0Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "non-virtual thunk toCRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_b918() {
    // IDA 0xb918: Thn36 thunk: this -= 36 (0xb91a), D2 dtor (0xb922) +
    // operator delete (0xb92c); the host drops with Rust ownership.
}

// 0xb930 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_b930() {
    // IDA 0xb930: thunk tail-calling the Creator D2 dtor; the host keeps
    // no factory image state.
}

// 0xb934 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_b934() {
    // IDA 0xb934: thunk tail-calling the AASamples EnumDesc D2 dtor;
    // the host table drops with Rust ownership.
}

// 0xb938 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
pub fn stub_b938() {
    // IDA 0xb938: EnumDesc dtor (0xb93e) + operator delete; the host table
    // drops with Rust ownership instead.
}

// 0xb94c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")]
pub fn stub_b94c(desc: &EnumDescModel, name: &str) -> Option<i32> {
    // IDA 0xb94c: Name::lookup (0xb958) + convertToValue (0xb966) +
    // convertToItem (0xb972); null (0) on failure (0xb968/0xb978).
    desc.lookup(name)
}

// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_b97c(value: i32) -> i32 {
    // IDA 0xb97c: any_cast of the Variant payload (0xb98e) + convertToItem
    // (0xb998) with no failure arm; the host carries the value directly.
    value
}

// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_b99c(desc: &EnumDescModel, index: usize, out: &mut i32) -> bool {
    // IDA 0xb99c: if count (+40) > index, out = legacy[index] (base +144,
    // 0xb9a4..0xb9b6) and return 1, else return 0.
    if let Some(&v) = desc.legacy.get(index) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
pub fn stub_b9f8(desc: &EnumDescModel, index: usize, out: &mut String) -> bool {
    // IDA 0xb9f8: if count (+40) > index, item string via legacy[index]
    // (0xba4c..0xba5c) assigned out (0xba66..0xba72), return 1 (0xba7a);
    // else return 0 (0xbaaa).
    if let Some((_, name)) = desc.pairs.get(index) {
        out.clear();
        out.push_str(name);
        true
    } else {
        false
    }
}

// 0xbb3c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_bb3c() {
    // IDA 0xbb3c: thunk tail-calling the GraphicsMode EnumDesc D2 dtor;
    // the host table drops with Rust ownership.
}

// 0xbb40 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
pub fn stub_bb40() {
    // IDA 0xbb40: EnumDesc dtor (0xbb46) + operator delete; the host table
    // drops with Rust ownership instead.
}

// 0xbb54 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")]
pub fn stub_bb54(desc: &EnumDescModel, name: &str) -> Option<i32> {
    // IDA 0xbb54: Name::lookup (0xbb60) + convertToValue (0xbb6e) +
    // convertToItem (0xbb7a); null (0) on failure (0xbb70/0xbb80).
    desc.lookup(name)
}

// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_bb84(value: i32) -> i32 {
    // IDA 0xbb84: any_cast of the Variant payload (0xbb96) + convertToItem
    // (0xbba0) with no failure arm; the host carries the value directly.
    value
}

// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_bba4(desc: &EnumDescModel, index: usize, out: &mut i32) -> bool {
    // IDA 0xbba4: same template as 0xb99c — if count (+40) > index,
    // out = legacy[index] (base +144, 0xbbac..0xbbbe) and return 1,
    // else return 0 (disasm-grounded; decompiler refused this EA).
    if let Some(&v) = desc.legacy.get(index) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_bc00(desc: &EnumDescModel, index: usize, out: &mut String) -> bool {
    // IDA 0xbc00: same convertToString template as 0xb9f8 — bound-checked
    // item string assigned out, 1 on success, 0 when out of range.
    if let Some((_, name)) = desc.pairs.get(index) {
        out.clear();
        out.push_str(name);
        true
    } else {
        false
    }
}

// 0xbd44 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_bd44() {
    // IDA 0xbd44: thunk tail-calling the FrameRateManagerMode EnumDesc D2
    // dtor; the host table drops with Rust ownership.
}

// 0xbd48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
pub fn stub_bd48() {
    // IDA 0xbd48: EnumDesc dtor (0xbd4e) + operator delete; the host table
    // drops with Rust ownership instead.
}

// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
pub fn stub_bd5c(desc: &EnumDescModel, name: &str) -> Option<i32> {
    // IDA 0xbd5c: Name::lookup (0xbd68) + convertToValue (0xbd76) +
    // convertToItem (0xbd82); null (0) on failure (0xbd78/0xbd88).
    desc.lookup(name)
}

// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_bd8c(value: i32) -> i32 {
    // IDA 0xbd8c: any_cast of the Variant payload (0xbd9e) + convertToItem
    // (0xbda8) with no failure arm; the host carries the value directly.
    value
}

// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_bdac(desc: &EnumDescModel, index: usize, out: &mut i32) -> bool {
    // IDA 0xbdac: same convertToValue template as 0xb99c/0xbba4 — bound
    // check on count (+40), out = legacy[index], 1/0
    // (disasm-grounded; decompiler refused this EA).
    if let Some(&v) = desc.legacy.get(index) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_be08(desc: &EnumDescModel, index: usize, out: &mut String) -> bool {
    // IDA 0xbe08: same convertToString template as 0xb9f8 — bound-checked
    // item string assigned out, 1 on success, 0 when out of range.
    if let Some((_, name)) = desc.pairs.get(index) {
        out.clear();
        out.push_str(name);
        true
    } else {
        false
    }
}

// 0xbf4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_bf4c() {
    // IDA 0xbf4c: thunk tail-calling the AntialiasingMode EnumDesc D2 dtor;
    // the host table drops with Rust ownership.
}

// 0xbf50 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_bf50() {
    // IDA 0xbf50: EnumDesc dtor (0xbf56) + operator delete; the host table
    // drops with Rust ownership instead.
}

// 0xbf64 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")]
pub fn stub_bf64(desc: &EnumDescModel, name: &str) -> Option<i32> {
    // IDA 0xbf64: Name::lookup (0xbf70) + convertToValue (0xbf7e) +
    // convertToItem (0xbf8a); null (0) on failure (0xbf80/0xbf90).
    desc.lookup(name)
}

// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_bf94(value: i32) -> i32 {
    // IDA 0xbf94: any_cast of the Variant payload (0xbfa6) + convertToItem
    // (0xbfb0) with no failure arm; the host carries the value directly.
    value
}

// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_bfb4(desc: &EnumDescModel, index: usize, out: &mut i32) -> bool {
    // IDA 0xbfb4: same convertToValue template as 0xb99c/0xbba4 — bound
    // check on count (+40), out = legacy[index], 1/0
    // (disasm-grounded; decompiler refused this EA).
    if let Some(&v) = desc.legacy.get(index) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c010(desc: &EnumDescModel, index: usize, out: &mut String) -> bool {
    // IDA 0xc010: same convertToString template as 0xb9f8 — bound-checked
    // item string assigned out, 1 on success, 0 when out of range.
    if let Some((_, name)) = desc.pairs.get(index) {
        out.clear();
        out.push_str(name);
        true
    } else {
        false
    }
}

// 0xc154 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_c154() {
    // IDA 0xc154: thunk tail-calling the ShadowMode EnumDesc D2 dtor;
    // the host table drops with Rust ownership.
}

// 0xc158 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_c158() {
    // IDA 0xc158: EnumDesc dtor (0xc15e) + operator delete; the host table
    // drops with Rust ownership instead.
}

// 0xc16c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(char const*)const")]
pub fn stub_c16c(desc: &EnumDescModel, name: &str) -> Option<i32> {
    // IDA 0xc16c: Name::lookup (0xc178) + convertToValue (0xc186) +
    // convertToItem (0xc192); null (0) on failure (0xc188/0xc198).
    desc.lookup(name)
}

// 0xc19c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_c19c(value: i32) -> i32 {
    // IDA 0xc19c: any_cast of the Variant payload (0xc1ae) + convertToItem
    // (0xc1b8) with no failure arm; the host carries the value directly.
    value
}

// 0xc1bc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_c1bc(desc: &EnumDescModel, index: usize, out: &mut i32) -> bool {
    // IDA 0xc1bc: same convertToValue template as 0xb99c/0xbba4 — bound
    // check on count (+40), out = legacy[index], 1/0
    // (disasm-grounded; decompiler refused this EA).
    if let Some(&v) = desc.legacy.get(index) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xc218 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c218(desc: &EnumDescModel, index: usize, out: &mut String) -> bool {
    // IDA 0xc218: same convertToString template as 0xb9f8 — bound-checked
    // item string assigned out, 1 on success, 0 when out of range.
    if let Some((_, name)) = desc.pairs.get(index) {
        out.clear();
        out.push_str(name);
        true
    } else {
        false
    }
}

// 0xc35c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_c35c() {
    // IDA 0xc35c: thunk tail-calling the QualityLevel EnumDesc D2 dtor;
    // the host table drops with Rust ownership.
}

// 0xc360 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_c360() {
    // IDA 0xc360: EnumDesc dtor + operator delete; the host table drops
    // with Rust ownership instead.
}

// 0xc374 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(char const*)const")]
pub fn stub_c374(desc: &EnumDescModel, name: &str) -> Option<i32> {
    // IDA 0xc374: Name::lookup + convertToValue + convertToItem; null (0)
    // on failure — same lookup template as 0xb94c/0xbb54/0xbd5c.
    desc.lookup(name)
}

// 0xc3a4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_c3a4(value: i32) -> i32 {
    // IDA 0xc3a4: any_cast of the Variant payload + convertToItem with no
    // failure arm — same lookup template as 0xb97c/0xbb84/0xbd8c.
    value
}

/// Host stand-in for the `rbx::implementation::typed_holder<T>` singletons
/// behind `placement_any<Region3>` (IDA 0xc95c/0xcf3c/0xd47c/0xd9bc/0xdefc):
/// each image static is `{typeinfo, destruct_func, construct_func(ptr)}`
/// guarded by `__cxa_guard_acquire`; the host keeps only the identity tag —
/// values travel as plain `i32` slots, so construction/destruction fold away.
pub struct EnumValueHolder {
    pub type_name: &'static str,
}

/// was: `typed_holder<ResolutionPreset>::singleton()::s` (IDA 0xc95c).
pub static RESOLUTION_PRESET_HOLDER: EnumValueHolder = EnumValueHolder {
    type_name: "RBX::CRenderSettings::ResolutionPreset",
};

/// was: `typed_holder<QualityLevel>::singleton()::s` (IDA 0xcf3c).
pub static QUALITY_LEVEL_HOLDER: EnumValueHolder = EnumValueHolder {
    type_name: "RBX::CRenderSettings::QualityLevel",
};

/// was: `typed_holder<ShadowMode>::singleton()::s` (IDA 0xd47c).
pub static SHADOW_MODE_HOLDER: EnumValueHolder = EnumValueHolder {
    type_name: "RBX::CRenderSettings::ShadowMode",
};

/// was: `typed_holder<AntialiasingMode>::singleton()::s` (IDA 0xd9bc).
pub static ANTIALIASING_MODE_HOLDER: EnumValueHolder = EnumValueHolder {
    type_name: "RBX::CRenderSettings::AntialiasingMode",
};

/// was: `typed_holder<FrameRateManagerMode>::singleton()::s` (IDA 0xdefc).
pub static FRAME_RATE_MANAGER_HOLDER: EnumValueHolder = EnumValueHolder {
    type_name: "RBX::CRenderSettings::FrameRateManagerMode",
};

// 0xc3c4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_c3c4(desc: &EnumDescModel, index: usize, out: &mut i32) -> bool {
    // IDA 0xc3c4: disasm 0xc3cc..0xc3de: LDR count +40 / CMP index / ITTTT HI gated legacy pair load; out = legacy[index], 1 on hit else 0.
    if let Some(&v) = desc.legacy.get(index) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xc420 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c420(desc: &EnumDescModel, index: usize, out: &mut String) -> bool {
    // IDA 0xc420: same convertToString template as 0xb9f8 — bound-checked item string assigned out, 1 on success, 0 when out of range.
    if let Some((_, name)) = desc.pairs.get(index) {
        out.clear();
        out.push_str(name);
        true
    } else {
        false
    }
}

// 0xc564 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
pub fn stub_c564() {
    // IDA 0xc564: thunk tail-calling the ResolutionPreset EnumDesc D2 dtor (0xccb0); the host table drops with Rust ownership (disasm: single B.W — pure thunk).
}

// 0xc568 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
pub fn stub_c568() {
    // IDA 0xc568: EnumDesc dtor + operator delete; the host table drops with Rust ownership instead. disasm: D2 call (0xc56e) + operator delete (0xc578).
}

// 0xc57c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const")]
pub fn stub_c57c(desc: &EnumDescModel, name: &str) -> Option<i32> {
    // IDA 0xc57c: disasm: Name::lookup (0xc588) + convertToValue<Name> (0xc592) + convertToItem; null on failure.
    desc.lookup(name)
}

// 0xc5ac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_c5ac(value: i32) -> i32 {
    // IDA 0xc5ac: disasm: any_cast (0xc5b6) + convertToItem (0xc5c2), no failure arm — the host carries the value directly.
    value
}

// 0xc5cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE
// type: 
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_c5cc(desc: &EnumDescModel, index: usize, out: &mut i32) -> bool {
    // IDA 0xc5cc: disasm 0xc5d4..0xc5e6: same LDR +40 / CMP / ITTTT HI gated pair load as 0xc3c4; out = legacy[index], 1 on hit else 0.
    if let Some(&v) = desc.legacy.get(index) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xc628 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const")]
pub fn stub_c628(desc: &EnumDescModel, index: usize, out: &mut String) -> bool {
    // IDA 0xc628: same convertToString template as 0xb9f8 — bound-checked item string assigned out, 1 on success, 0 when out of range.
    if let Some((_, name)) = desc.pairs.get(index) {
        out.clear();
        out.push_str(name);
        true
    } else {
        false
    }
}

// 0xc76c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")]
pub fn stub_c76c(desc: &EnumDescModel, value: i32, out: &mut String) {
    // IDA 0xc76c: decompile: ReleaseAssert value>=0 (enumconverter.h:262) + value<enumToItem.size() (:263); empty string when negative/oversized, else enumToItem[value] (0xc876..0xc896). Host pairs search is the same observable mapping at any density.
    out.clear();
    if value >= 0 {
        if let Some((_, name)) = desc.pairs.iter().find(|(v, _)| *v == value) {
            out.push_str(name);
        }
    }
}

// 0xc90c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16ResolutionPresetEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ResolutionPreset>(RBX::CRenderSettings::ResolutionPreset const&)")]
pub fn stub_c90c(slot: &mut i32, value: i32) {
    // IDA 0xc90c: decompile: singleton() (0xc918); holder match stores value only (0xc944), else destroy-old (0xc930..0xc93c) + store + install holder (0xc94e..0xc950). The holder tag folds into the ResolutionPreset typed_holder static; the host carries the POD value directly.
    *slot = value;
}

// 0xc95c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::singleton(void)")]
pub fn stub_c95c() -> &'static EnumValueHolder {
    // IDA 0xc95c: decompile: guard-checked static init {typeinfo, destruct_func, construct_func} (0xc976..0xc9b6); returns &s (0xc9c6); returns the holder static.
    &RESOLUTION_PRESET_HOLDER
}

// 0xc9c8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::construct_func(char const*,char *)")]
pub fn stub_c9c8(dst: &mut i32, src: &i32) {
    // IDA 0xc9c8: decompile: POD construct arm copying the int-sized enum (0xc9ca..0xc9d0) — int-sized enum copied into the slot.
    *dst = *src;
}

// 0xc9d4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16ResolutionPresetEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ResolutionPreset>::destruct_func(char *)")]
pub fn stub_c9d4() {
    // IDA 0xc9d4: decompile: empty body — no teardown (114 chars) — int-sized enum needs no teardown.
}

// 0xc9d8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")]
pub fn stub_c9d8(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0xc9d8: decompile: ReleaseAssert value>=0 (:273) + value<size (:274); in range returns enumToItem[value] (0xc984..0xc99c), else 0. Host pairs search matches at any density ([INFERENCE] on table layout only).
    if value >= 0 {
        if let Some((v, _)) = desc.pairs.iter().find(|(val, _)| *val == value) {
            return *v;
        }
    }
    0
}

// 0xcaa4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16ResolutionPresetENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::ResolutionPreset const& rbx::any_cast<RBX::CRenderSettings::ResolutionPreset const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_caa4(value: i32) -> i32 {
    // IDA 0xcaa4: decompile: holder typeinfo check (0xcace..0xcb10); mismatch throws std::bad_cast (0xcb62..0xcb86); hit returns value slot a1+1 (0xcb4a). Host slots are checked upstream — the bad_cast arm is documented, not rebuilt.
    value
}

// 0xcb94 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::~refcount_ptr()")]
pub fn stub_cb94() {
    // IDA 0xcb94: decompile: releases the error_info_container ref when count hits 1 via vtable+16 (0xcbf2..0xcbf6). was: boost::exception_detail::refcount_ptr → anyhow/thiserror; the host stores no exception payload, so release folds into drop.
}

// 0xcc34 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")]
pub fn stub_cc34(desc: &EnumDescModel, name: &str, out: &mut i32) -> bool {
    // IDA 0xcc34: decompile: twin RB-tree lower_bounds over the name→value maps (0xcc4a..0xcc9e); hit stores node value + 1 (0xccaa..0xccac), miss 0. Host by_name merges both trees.
    if let Some(&v) = desc.by_name.get(name) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xccb0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
pub fn stub_ccb0() {
    // IDA 0xccb0: D2 — decompile: _M_erase over both name→value maps (0xcd34/0xcd3e) + EnumDescriptor base dtor (0xcd48); host tables drop with Rust ownership.
}

// 0xcd4c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_cd4c(desc: &EnumDescModel, value: i32, out: &mut String) {
    // IDA 0xcd4c: same convertToString-value template as 0xc76c (asserts + bound-checked item string, empty when out of range). Host pairs search is the same observable mapping at any density.
    out.clear();
    if value >= 0 {
        if let Some((_, name)) = desc.pairs.iter().find(|(v, _)| *v == value) {
            out.push_str(name);
        }
    }
}

// 0xceec — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings12QualityLevelEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::QualityLevel>(RBX::CRenderSettings::QualityLevel const&)")]
pub fn stub_ceec(slot: &mut i32, value: i32) {
    // IDA 0xceec: same placement_any op= template as 0xc90c (singleton fetch + holder-match fast path, else destroy/store/install). The holder tag folds into the QualityLevel typed_holder static; the host carries the POD value directly.
    *slot = value;
}

// 0xcf3c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::singleton(void)")]
pub fn stub_cf3c() -> &'static EnumValueHolder {
    // IDA 0xcf3c: same singleton template as 0xc95c (guard-checked static {typeinfo, destruct, construct}); returns the holder static.
    &QUALITY_LEVEL_HOLDER
}

// 0xcfa8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::construct_func(char const*,char *)")]
pub fn stub_cfa8(dst: &mut i32, src: &i32) {
    // IDA 0xcfa8: same construct_func template as 0xc9c8 (POD copy arm) — int-sized enum copied into the slot.
    *dst = *src;
}

// 0xcfb4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings12QualityLevelEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::QualityLevel>::destruct_func(char *)")]
pub fn stub_cfb4() {
    // IDA 0xcfb4: same destruct_func template as 0xc9d4 (empty body) — int-sized enum needs no teardown.
}

// 0xcfb8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")]
pub fn stub_cfb8(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0xcfb8: same convertToItem template as 0xc9d8 (asserts + in-range item else 0). Host pairs search matches at any density ([INFERENCE] on table layout only).
    if value >= 0 {
        if let Some((v, _)) = desc.pairs.iter().find(|(val, _)| *val == value) {
            return *v;
        }
    }
    0
}

// 0xd084 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings12QualityLevelENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::QualityLevel const& rbx::any_cast<RBX::CRenderSettings::QualityLevel const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_d084(value: i32) -> i32 {
    // IDA 0xd084: same any_cast template as 0xcaa4 (holder typeinfo check; bad_cast on mismatch; value slot on hit). Host slots are checked upstream — the bad_cast arm is documented, not rebuilt.
    value
}

// 0xd174 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")]
pub fn stub_d174(desc: &EnumDescModel, name: &str, out: &mut i32) -> bool {
    // IDA 0xd174: same convertToValue<Name> template as 0xcc34 (twin RB-tree lower_bounds; hit stores node value + 1, else 0). Host by_name merges both trees.
    if let Some(&v) = desc.by_name.get(name) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xd1f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
pub fn stub_d1f0() {
    // IDA 0xd1f0: D2 — same D2 template as 0xccb0 (two _M_erase + base dtor); host tables drop with Rust ownership.
}

// 0xd28c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_d28c(desc: &EnumDescModel, value: i32, out: &mut String) {
    // IDA 0xd28c: same convertToString-value template as 0xc76c (asserts + bound-checked item string, empty when out of range). Host pairs search is the same observable mapping at any density.
    out.clear();
    if value >= 0 {
        if let Some((_, name)) = desc.pairs.iter().find(|(v, _)| *v == value) {
            out.push_str(name);
        }
    }
}

// 0xd42c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings10ShadowModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::ShadowMode>(RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_d42c(slot: &mut i32, value: i32) {
    // IDA 0xd42c: same placement_any op= template as 0xc90c (singleton fetch + holder-match fast path, else destroy/store/install). The holder tag folds into the ShadowMode typed_holder static; the host carries the POD value directly.
    *slot = value;
}

// 0xd47c — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::singleton(void)")]
pub fn stub_d47c() -> &'static EnumValueHolder {
    // IDA 0xd47c: same singleton template as 0xc95c (guard-checked static {typeinfo, destruct, construct}); returns the holder static.
    &SHADOW_MODE_HOLDER
}

// 0xd4e8 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::construct_func(char const*,char *)")]
pub fn stub_d4e8(dst: &mut i32, src: &i32) {
    // IDA 0xd4e8: same construct_func template as 0xc9c8 (POD copy arm) — int-sized enum copied into the slot.
    *dst = *src;
}

// 0xd4f4 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings10ShadowModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::ShadowMode>::destruct_func(char *)")]
pub fn stub_d4f4() {
    // IDA 0xd4f4: same destruct_func template as 0xc9d4 (empty body) — int-sized enum needs no teardown.
}

// 0xd4f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")]
pub fn stub_d4f8(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0xd4f8: same convertToItem template as 0xc9d8 (asserts + in-range item else 0). Host pairs search matches at any density ([INFERENCE] on table layout only).
    if value >= 0 {
        if let Some((v, _)) = desc.pairs.iter().find(|(val, _)| *val == value) {
            return *v;
        }
    }
    0
}

// 0xd5c4 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings10ShadowModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::ShadowMode const& rbx::any_cast<RBX::CRenderSettings::ShadowMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_d5c4(value: i32) -> i32 {
    // IDA 0xd5c4: same any_cast template as 0xcaa4 (holder typeinfo check; bad_cast on mismatch; value slot on hit). Host slots are checked upstream — the bad_cast arm is documented, not rebuilt.
    value
}

// 0xd6b4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")]
pub fn stub_d6b4(desc: &EnumDescModel, name: &str, out: &mut i32) -> bool {
    // IDA 0xd6b4: same convertToValue<Name> template as 0xcc34 (twin RB-tree lower_bounds; hit stores node value + 1, else 0). Host by_name merges both trees.
    if let Some(&v) = desc.by_name.get(name) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xd730 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
pub fn stub_d730() {
    // IDA 0xd730: D2 — same D2 template as 0xccb0 (two _M_erase + base dtor); host tables drop with Rust ownership.
}

// 0xd7cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_d7cc(desc: &EnumDescModel, value: i32, out: &mut String) {
    // IDA 0xd7cc: same convertToString-value template as 0xc76c (asserts + bound-checked item string, empty when out of range). Host pairs search is the same observable mapping at any density.
    out.clear();
    if value >= 0 {
        if let Some((_, name)) = desc.pairs.iter().find(|(v, _)| *v == value) {
            out.push_str(name);
        }
    }
}

// 0xd96c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings16AntialiasingModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::AntialiasingMode>(RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_d96c(slot: &mut i32, value: i32) {
    // IDA 0xd96c: same placement_any op= template as 0xc90c (singleton fetch + holder-match fast path, else destroy/store/install). The holder tag folds into the AntialiasingMode typed_holder static; the host carries the POD value directly.
    *slot = value;
}

// 0xd9bc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::singleton(void)")]
pub fn stub_d9bc() -> &'static EnumValueHolder {
    // IDA 0xd9bc: same singleton template as 0xc95c (guard-checked static {typeinfo, destruct, construct}); returns the holder static.
    &ANTIALIASING_MODE_HOLDER
}

// 0xda28 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::construct_func(char const*,char *)")]
pub fn stub_da28(dst: &mut i32, src: &i32) {
    // IDA 0xda28: same construct_func template as 0xc9c8 (POD copy arm) — int-sized enum copied into the slot.
    *dst = *src;
}

// 0xda34 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings16AntialiasingModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::AntialiasingMode>::destruct_func(char *)")]
pub fn stub_da34() {
    // IDA 0xda34: same destruct_func template as 0xc9d4 (empty body) — int-sized enum needs no teardown.
}

// 0xda38 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")]
pub fn stub_da38(desc: &EnumDescModel, value: i32) -> i32 {
    // IDA 0xda38: same convertToItem template as 0xc9d8 (asserts + in-range item else 0). Host pairs search matches at any density ([INFERENCE] on table layout only).
    if value >= 0 {
        if let Some((v, _)) = desc.pairs.iter().find(|(val, _)| *val == value) {
            return *v;
        }
    }
    0
}

// 0xdb04 — __ZN3rbx8any_castIRKN3RBX15CRenderSettings16AntialiasingModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode const& rbx::any_cast<RBX::CRenderSettings::AntialiasingMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_db04(value: i32) -> i32 {
    // IDA 0xdb04: same any_cast template as 0xcaa4 (holder typeinfo check; bad_cast on mismatch; value slot on hit). Host slots are checked upstream — the bad_cast arm is documented, not rebuilt.
    value
}

// 0xdbf4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")]
pub fn stub_dbf4(desc: &EnumDescModel, name: &str, out: &mut i32) -> bool {
    // IDA 0xdbf4: same convertToValue<Name> template as 0xcc34 (twin RB-tree lower_bounds; hit stores node value + 1, else 0). Host by_name merges both trees.
    if let Some(&v) = desc.by_name.get(name) {
        *out = v;
        true
    } else {
        false
    }
}

// 0xdc70 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
pub fn stub_dc70() {
    // IDA 0xdc70: D2 — same D2 template as 0xccb0 (two _M_erase + base dtor); host tables drop with Rust ownership.
}

// 0xdd0c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
pub fn stub_dd0c(desc: &EnumDescModel, value: i32, out: &mut String) {
    // IDA 0xdd0c: same convertToString-value template as 0xc76c (asserts + bound-checked item string, empty when out of range). Host pairs search is the same observable mapping at any density.
    out.clear();
    if value >= 0 {
        if let Some((_, name)) = desc.pairs.iter().find(|(v, _)| *v == value) {
            out.push_str(name);
        }
    }
}

// 0xdeac — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15CRenderSettings20FrameRateManagerModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CRenderSettings::FrameRateManagerMode>(RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_deac(slot: &mut i32, value: i32) {
    // IDA 0xdeac: same placement_any op= template as 0xc90c (singleton fetch + holder-match fast path, else destroy/store/install). The holder tag folds into the FrameRateManagerMode typed_holder static; the host carries the POD value directly.
    *slot = value;
}

// 0xdefc — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::singleton(void)")]
pub fn stub_defc() -> &'static EnumValueHolder {
    // IDA 0xdefc: same singleton template as 0xc95c (guard-checked static {typeinfo, destruct, construct}); returns the holder static.
    &FRAME_RATE_MANAGER_HOLDER
}

// 0xdf68 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::construct_func(char const*,char *)")]
pub fn stub_df68(dst: &mut i32, src: &i32) {
    // IDA 0xdf68: same construct_func template as 0xc9c8 (POD copy arm) — int-sized enum copied into the slot.
    *dst = *src;
}

// 0xdf74 — __ZN3rbx14implementation12typed_holderIN3RBX15CRenderSettings20FrameRateManagerModeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CRenderSettings::FrameRateManagerMode>::destruct_func(char *)")]
pub fn stub_df74() {
    // IDA 0xdf74: same destruct_func template as 0xc9d4 (empty body) — int-sized enum needs no teardown.
}
