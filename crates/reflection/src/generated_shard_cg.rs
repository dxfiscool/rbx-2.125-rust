// Auto-generated shard CG — next 100 RBX::Reflection stubs — EA-sorted ascending 0xf48914..0xf49d54 (remaining 1785) — starts 0xf48914
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total, 15508->15608 covered, 1685 remaining)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr (was boost::shared_ptr)
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;

// 0xf48914 — j___ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev")]
pub fn stub_f48914() {
    // IDA 0xf48914: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf48924 — j___ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev")]
pub fn stub_f48924() {
    // IDA 0xf48924: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf48934 — j___ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED2Ev")]
pub fn stub_f48934() {
    // IDA 0xf48934: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf48944 — j___ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED2Ev")]
pub fn stub_f48944() {
    // IDA 0xf48944: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf48964 — j___ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::EventDesc(rbx::signal<void ()(bool)> RBX::TextBox::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_7TextBoxEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_f48964() -> ! {
    todo!("0xf48964 RBX::Reflection::EventDesc<RBX::TextBox,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::TextBox::*>::EventDesc(rbx::signal<void ()(bool)> RBX::TextBox::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf48974 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::XAlignment> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv")]
pub fn stub_f48974() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf48974: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_aa::stub_0x7d8544)
}

// 0xf48984 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::YAlignment> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv")]
pub fn stub_f48984() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf48984: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_aa::stub_0x7d8720)
}

// 0xf48994 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::Font> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv")]
pub fn stub_f48994() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf48994: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_aa::stub_0x7d833c)
}

// 0xf489a4 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TextService::FontSize> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE14doGetSingletonEv")]
pub fn stub_f489a4() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf489a4: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_aa::stub_0x7d80c0)
}

// 0xf48d14 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f48d14() -> ! {
    todo!("0xf48d14 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf48d24 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f48d24() -> ! {
    todo!("0xf48d24 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf48d34 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f48d34() -> ! {
    todo!("0xf48d34 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf48d44 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7TextBoxENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f48d44() -> ! {
    todo!("0xf48d44 RBX::Reflection::EnumPropDescriptor<RBX::TextBox,RBX::TextService::FontSize>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf48d54 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToItem(RBX::TextService::XAlignment const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE13convertToItemERKS3_")]
pub fn stub_f48d54(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf48d54: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf48d64 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToIndex(RBX::TextService::XAlignment)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToIndexES3_")]
pub fn stub_f48d64(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf48d64: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf48d74 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToValue(RBX::Name const&,RBX::TextService::XAlignment&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f48d74(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf48d74: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf48d84 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::XAlignment>::convertToString(RBX::TextService::XAlignment const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE15convertToStringERKS3_")]
pub fn stub_f48d84(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf48d84: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf48d94 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToItem(RBX::TextService::YAlignment const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE13convertToItemERKS3_")]
pub fn stub_f48d94(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf48d94: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf48da4 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToIndex(RBX::TextService::YAlignment)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToIndexES3_")]
pub fn stub_f48da4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf48da4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf48db4 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToValue(RBX::Name const&,RBX::TextService::YAlignment&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f48db4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf48db4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf48dc4 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::YAlignment>::convertToString(RBX::TextService::YAlignment const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE15convertToStringERKS3_")]
pub fn stub_f48dc4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf48dc4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf48dd4 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToItem(RBX::TextService::Font const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE13convertToItemERKS3_")]
pub fn stub_f48dd4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf48dd4: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf48de4 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToIndex(RBX::TextService::Font)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_")]
pub fn stub_f48de4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf48de4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf48df4 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToValue(RBX::Name const&,RBX::TextService::Font&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f48df4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf48df4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf48e04 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::Font>::convertToString(RBX::TextService::Font const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE15convertToStringERKS3_")]
pub fn stub_f48e04(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf48e04: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf48e14 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToItem(RBX::TextService::FontSize const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE13convertToItemERKS3_")]
pub fn stub_f48e14(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf48e14: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf48e24 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToIndex(RBX::TextService::FontSize)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToIndexES3_")]
pub fn stub_f48e24(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf48e24: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf48e34 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToValue(RBX::Name const&,RBX::TextService::FontSize&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f48e34(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf48e34: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf48e44 — j___ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TextService::FontSize>::convertToString(RBX::TextService::FontSize const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE15convertToStringERKS3_")]
pub fn stub_f48e44(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf48e44: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf48e84 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextBox,RBX::TextBox>(rbx_core::SharedPtr<RBX::TextBox> const*,RBX::TextBox *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextBoxES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f48e84() {
    // IDA 0xf48e84: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf48ed4 — j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48ed4() -> ! {
    todo!("0xf48ed4 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48ee4 — j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EEC2IMS2_KFS4_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EEC2IMS2_KFS4_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48ee4() -> ! {
    todo!("0xf48ee4 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48ef4 — j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48ef4() -> ! {
    todo!("0xf48ef4 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48f04 — j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonESsEC2IMNS_12GuiTextMixinEKFSsvEMS2_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,std::string>::PropDescriptor<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(std::string)>(char const*,char const*,std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonESsEC2IMNS_12GuiTextMixinEKFSsvEMS2_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48f04() -> ! {
    todo!("0xf48f04 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,std::string>::PropDescriptor<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(std::string)>(char const*,char const*,std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48f14 — j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48f14() -> ! {
    todo!("0xf48f14 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48f24 — j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,bool (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48f24() -> ! {
    todo!("0xf48f24 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,bool (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48f34 — j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48f34() -> ! {
    todo!("0xf48f34 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48f44 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48f44() -> ! {
    todo!("0xf48f44 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48f54 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48f54() -> ! {
    todo!("0xf48f54 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48f64 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48f64() -> ! {
    todo!("0xf48f64 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48f74 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::EnumPropDescriptor<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::FontSize)>(char const*,char const*,RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::FontSize),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f48f74() -> ! {
    todo!("0xf48f74 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::EnumPropDescriptor<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::FontSize)>(char const*,char const*,RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::FontSize),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf48ff4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f48ff4() -> ! {
    todo!("0xf48ff4 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf49004 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f49004() -> ! {
    todo!("0xf49004 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf49014 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f49014() -> ! {
    todo!("0xf49014 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf49024 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f49024() -> ! {
    todo!("0xf49024 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf49044 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13GuiTextButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiTextButton,RBX::GuiTextButton>(rbx_core::SharedPtr<RBX::GuiTextButton> const*,RBX::GuiTextButton *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13GuiTextButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f49044() {
    // IDA 0xf49044: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf49054 — j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextLabel,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f49054() -> ! {
    todo!("0xf49054 RBX::Reflection::PropDescriptor<RBX::TextLabel,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf49064 — j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEN3G3D7Vector2EEC2IMS2_KFS4_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextLabel,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::TextLabel::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::TextLabel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEN3G3D7Vector2EEC2IMS2_KFS4_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f49064() -> ! {
    todo!("0xf49064 RBX::Reflection::PropDescriptor<RBX::TextLabel,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::TextLabel::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::TextLabel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf49074 — j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextLabel,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f49074() -> ! {
    todo!("0xf49074 RBX::Reflection::PropDescriptor<RBX::TextLabel,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf49084 — j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelESsEC2IMNS_12GuiTextMixinEKFSsvEMS2_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextLabel,std::string>::PropDescriptor<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(std::string)>(char const*,char const*,std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelESsEC2IMNS_12GuiTextMixinEKFSsvEMS2_FvSsEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f49084() -> ! {
    todo!("0xf49084 RBX::Reflection::PropDescriptor<RBX::TextLabel,std::string>::PropDescriptor<std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(std::string)>(char const*,char const*,std::string (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf49094 — j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextLabel,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f49094() -> ! {
    todo!("0xf49094 RBX::Reflection::PropDescriptor<RBX::TextLabel,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf490a4 — j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextLabel,bool>::PropDescriptor<bool (RBX::TextLabel::*)(void)const,int>(char const*,char const*,bool (RBX::TextLabel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f490a4() -> ! {
    todo!("0xf490a4 RBX::Reflection::PropDescriptor<RBX::TextLabel,bool>::PropDescriptor<bool (RBX::TextLabel::*)(void)const,int>(char const*,char const*,bool (RBX::TextLabel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf490b4 — j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextLabel,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f490b4() -> ! {
    todo!("0xf490b4 RBX::Reflection::PropDescriptor<RBX::TextLabel,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf490c4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f490c4() -> ! {
    todo!("0xf490c4 RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf490d4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f490d4() -> ! {
    todo!("0xf490d4 RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf490e4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f490e4() -> ! {
    todo!("0xf490e4 RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf490f4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService8FontSizeEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::FontSize>::EnumPropDescriptor<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::FontSize)>(char const*,char const*,RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::FontSize),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService8FontSizeEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f490f4() -> ! {
    todo!("0xf490f4 RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::FontSize>::EnumPropDescriptor<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::FontSize)>(char const*,char const*,RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::TextLabel::*)(RBX::TextService::FontSize),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf49144 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f49144() -> ! {
    todo!("0xf49144 RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf49154 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f49154() -> ! {
    todo!("0xf49154 RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf49164 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f49164() -> ! {
    todo!("0xf49164 RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf49174 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::FontSize>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9TextLabelENS_11TextService8FontSizeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f49174() -> ! {
    todo!("0xf49174 RBX::Reflection::EnumPropDescriptor<RBX::TextLabel,RBX::TextService::FontSize>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf491c4 — j___ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f491c4() -> ! {
    todo!("0xf491c4 RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf491d4 — j___ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EEC2IMS2_KFKS4_vEMS2_FvRS7_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::PropDescriptor<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EEC2IMS2_KFKS4_vEMS2_FvRS7_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f491d4() -> ! {
    todo!("0xf491d4 RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::PropDescriptor<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf491e4 — j___ZN3RBX10Reflection14PropDescriptorINS_4ToolESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::PropDescriptor<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>(char const*,char const*,std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_4ToolESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f491e4() -> ! {
    todo!("0xf491e4 RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::PropDescriptor<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>(char const*,char const*,std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf491f4 — j___ZN3RBX10Reflection14PropDescriptorINS_4ToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::PropDescriptor<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>(char const*,char const*,bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_4ToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f491f4() -> ! {
    todo!("0xf491f4 RBX::Reflection::PropDescriptor<RBX::Tool,bool>::PropDescriptor<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>(char const*,char const*,bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf49204 — j___ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<0,RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::replicateEvent(RBX::Reflection::EventSource *)")]
#[doc(alias = "j___ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE")]
pub fn stub_f49204() -> ! {
    todo!("0xf49204 RBX::Reflection::RemoteEventDescImpl<0,RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::replicateEvent(RBX::Reflection::EventSource *)")
}

// 0xf49214 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4ToolEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Tool>(char const*,char const*,bool RBX::Tool::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4ToolEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f49214() -> ! {
    todo!("0xf49214 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Tool>(char const*,char const*,bool RBX::Tool::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf49224 — j___ZN3RBX10Reflection9EventDescINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_EC2ES9_PKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Tool,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::EventDesc(RBX::Tool::special_equipped_signal RBX::Tool::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_EC2ES9_PKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_f49224() -> ! {
    todo!("0xf49224 RBX::Reflection::EventDesc<RBX::Tool,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::EventDesc(RBX::Tool::special_equipped_signal RBX::Tool::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf494a4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4ToolES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Tool,RBX::Tool>(rbx_core::SharedPtr<RBX::Tool> const*,RBX::Tool *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4ToolES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f494a4() {
    // IDA 0xf494a4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf494b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MouseES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Mouse,RBX::Mouse>(rbx_core::SharedPtr<RBX::Mouse> const*,RBX::Mouse *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MouseES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f494b4() {
    // IDA 0xf494b4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf496d4 — j___ZN3RBX10Reflection11Call1HelperINS_10ControllerEMS2_FbNS2_6ButtonEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Controller,bool (RBX::Controller::*)(RBX::Controller::Button),RBX::Controller::Button,bool>::call(RBX::Controller*,bool (RBX::Controller::*)(RBX::Controller::Button),RBX::Reflection::Variant &,RBX::Controller::Button const&)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call1HelperINS_10ControllerEMS2_FbNS2_6ButtonEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_")]
pub fn stub_f496d4() -> ! {
    todo!("0xf496d4 RBX::Reflection::Call1Helper<RBX::Controller,bool (RBX::Controller::*)(RBX::Controller::Button),RBX::Controller::Button,bool>::call(RBX::Controller*,bool (RBX::Controller::*)(RBX::Controller::Button),RBX::Reflection::Variant &,RBX::Controller::Button const&)")
}

// 0xf496e4 — j___ZN3RBX10Reflection11Call2HelperINS_10ControllerEMS2_FvNS2_6ButtonESsES3_SsvE4callEPS2_S5_RNS0_7VariantERKS3_RKSs
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Controller,void (RBX::Controller::*)(RBX::Controller::Button,std::string),RBX::Controller::Button,std::string,void>::call(RBX::Controller*,void (RBX::Controller::*)(RBX::Controller::Button,std::string),RBX::Reflection::Variant &,RBX::Controller::Button const&,std::string const&)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call2HelperINS_10ControllerEMS2_FvNS2_6ButtonESsES3_SsvE4callEPS2_S5_RNS0_7VariantERKS3_RKSs")]
pub fn stub_f496e4() -> ! {
    todo!("0xf496e4 RBX::Reflection::Call2Helper<RBX::Controller,void (RBX::Controller::*)(RBX::Controller::Button,std::string),RBX::Controller::Button,std::string,void>::call(RBX::Controller*,void (RBX::Controller::*)(RBX::Controller::Button,std::string),RBX::Reflection::Variant &,RBX::Controller::Button const&,std::string const&)")
}

// 0xf496f4 — j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFbNS2_6ButtonEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Controller,bool ()(RBX::Controller::Button),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFbNS2_6ButtonEELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_f496f4() -> ! {
    todo!("0xf496f4 RBX::Reflection::BoundFuncDesc<RBX::Controller,bool ()(RBX::Controller::Button),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf49704 — j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFbNS2_6ButtonEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Controller,bool ()(RBX::Controller::Button),1>::BoundFuncDesc(bool (RBX::Controller::*)(RBX::Controller::Button),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFbNS2_6ButtonEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_f49704() -> ! {
    todo!("0xf49704 RBX::Reflection::BoundFuncDesc<RBX::Controller,bool ()(RBX::Controller::Button),1>::BoundFuncDesc(bool (RBX::Controller::*)(RBX::Controller::Button),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf49714 — j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Controller,void ()(RBX::Controller::Button),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonEELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_f49714() -> ! {
    todo!("0xf49714 RBX::Reflection::BoundFuncDesc<RBX::Controller,void ()(RBX::Controller::Button),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf49724 — j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Controller,void ()(RBX::Controller::Button),1>::BoundFuncDesc(void (RBX::Controller::*)(RBX::Controller::Button),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_f49724() -> ! {
    todo!("0xf49724 RBX::Reflection::BoundFuncDesc<RBX::Controller,void ()(RBX::Controller::Button),1>::BoundFuncDesc(void (RBX::Controller::*)(RBX::Controller::Button),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf49734 — j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonESsELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Controller,void ()(RBX::Controller::Button,std::string),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonESsELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_")]
pub fn stub_f49734() -> ! {
    todo!("0xf49734 RBX::Reflection::BoundFuncDesc<RBX::Controller,void ()(RBX::Controller::Button,std::string),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0xf49744 — j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonESsELi2EEC2EMS2_FvS3_SsEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Controller,void ()(RBX::Controller::Button,std::string),2>::BoundFuncDesc(void (RBX::Controller::*)(RBX::Controller::Button,std::string),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_10ControllerEFvNS2_6ButtonESsELi2EEC2EMS2_FvS3_SsEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_f49744() -> ! {
    todo!("0xf49744 RBX::Reflection::BoundFuncDesc<RBX::Controller,void ()(RBX::Controller::Button,std::string),2>::BoundFuncDesc(void (RBX::Controller::*)(RBX::Controller::Button,std::string),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf49754 — j___ZN3RBX10Reflection7Variant14genericConvertINS_10Controller6ButtonEEERT_v
#[doc(alias = "RBX::Controller::Button & RBX::Reflection::Variant::genericConvert<RBX::Controller::Button>(void)")]
#[doc(alias = "j___ZN3RBX10Reflection7Variant14genericConvertINS_10Controller6ButtonEEERT_v")]
pub fn stub_f49754() -> ! {
    todo!("0xf49754 RBX::Controller::Button & RBX::Reflection::Variant::genericConvert<RBX::Controller::Button>(void)")
}

// 0xf49764 — j___ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::addPair(RBX::Controller::Button,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEE7addPairES3_PKc")]
pub fn stub_f49764(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf49764: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf49774 — j___ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED2Ev")]
pub fn stub_f49774() {
    // IDA 0xf49774: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf49784 — j___ZN3RBX10Reflection9ArgHelper6getArgINS_10Controller6ButtonELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Controller::Button RBX::Reflection::ArgHelper::getArg<RBX::Controller::Button,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Controller::Button> const&,boost::disable_if<boost::is_same<RBX::Controller::Button,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgINS_10Controller6ButtonELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_f49784() -> ! {
    todo!("0xf49784 RBX::Controller::Button RBX::Reflection::ArgHelper::getArg<RBX::Controller::Button,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Controller::Button> const&,boost::disable_if<boost::is_same<RBX::Controller::Button,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf49794 — j___ZN3RBX10Reflection9ArgHelper6getArgISsLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgISsLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_f49794() -> ! {
    todo!("0xf49794 std::string RBX::Reflection::ArgHelper::getArg<std::string,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf497a4 — j___ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_10Controller6ButtonEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::Controller::Button>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Controller::Button &,boost::enable_if<boost::is_enum<RBX::Controller::Button>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_10Controller6ButtonEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_f497a4() -> ! {
    todo!("0xf497a4 bool RBX::Reflection::ArgHelper::try_enum<1,RBX::Controller::Button>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Controller::Button &,boost::enable_if<boost::is_enum<RBX::Controller::Button>,void>::type *)")
}

// 0xf49804 — j___ZN3RBX10Reflection9EventDescINS_10ControllerEFvNS2_6ButtonEEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Controller,void ()(RBX::Controller::Button),rbx::signal<void ()(RBX::Controller::Button)>,rbx::signal<void ()(RBX::Controller::Button)> RBX::Controller::*>::EventDesc(rbx::signal<void ()(RBX::Controller::Button)> RBX::Controller::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_10ControllerEFvNS2_6ButtonEEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_f49804() -> ! {
    todo!("0xf49804 RBX::Reflection::EventDesc<RBX::Controller,void ()(RBX::Controller::Button),rbx::signal<void ()(RBX::Controller::Button)>,rbx::signal<void ()(RBX::Controller::Button)> RBX::Controller::*>::EventDesc(rbx::signal<void ()(RBX::Controller::Button)> RBX::Controller::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf49814 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Controller6ButtonEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Controller::Button> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Controller6ButtonEEEE14doGetSingletonEv")]
pub fn stub_f49814() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf49814: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_m::stub_0x6907f4)
}

// 0xf49b14 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_10Controller6ButtonEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::Controller::Button>(RBX::Controller::Button &)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_10Controller6ButtonEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_")]
pub fn stub_f49b14() -> ! {
    todo!("0xf49b14 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::Controller::Button>(RBX::Controller::Button &)")
}

// 0xf49b24 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_10Controller6ButtonENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Controller::Button const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
#[doc(alias = "j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_10Controller6ButtonENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_")]
pub fn stub_f49b24() -> ! {
    todo!("0xf49b24 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Controller::Button const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")
}

// 0xf49b64 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10Controller6ButtonEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10Controller6ButtonEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_f49b64() -> ! {
    todo!("0xf49b64 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf49bb4 — j___ZN5boost9function1IvN3RBX10Controller6ButtonEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::Controller::Button>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
#[doc(alias = "j___ZN5boost9function1IvN3RBX10Controller6ButtonEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_f49bb4() -> ! {
    todo!("0xf49bb4 void boost::function1<void,RBX::Controller::Button>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}

// 0xf49c94 — j___ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::convertToItem(RBX::Controller::Button const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE13convertToItemERKS3_")]
pub fn stub_f49c94(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf49c94: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf49ca4 — j___ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::convertToValue(RBX::Name const&,RBX::Controller::Button&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_f49ca4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf49ca4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf49cb4 — j___ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Controller::Button>::convertToString(RBX::Controller::Button const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10Controller6ButtonEE15convertToStringERKS3_")]
pub fn stub_f49cb4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf49cb4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf49cf4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17GameBasicSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GameBasicSettings,RBX::GameBasicSettings>(rbx_core::SharedPtr<RBX::GameBasicSettings> const*,RBX::GameBasicSettings *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17GameBasicSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f49cf4() {
    // IDA 0xf49cf4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf49d04 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17VehicleControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::VehicleController,RBX::VehicleController>(rbx_core::SharedPtr<RBX::VehicleController> const*,RBX::VehicleController *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17VehicleControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f49d04() {
    // IDA 0xf49d04: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf49d14 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18HumanoidControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HumanoidController,RBX::HumanoidController>(rbx_core::SharedPtr<RBX::HumanoidController> const*,RBX::HumanoidController *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18HumanoidControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f49d14() {
    // IDA 0xf49d14: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf49d24 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19ButtonBindingWidgetES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ButtonBindingWidget,RBX::ButtonBindingWidget>(rbx_core::SharedPtr<RBX::ButtonBindingWidget> const*,RBX::ButtonBindingWidget *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19ButtonBindingWidgetES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f49d24() {
    // IDA 0xf49d24: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf49d34 — j___ZNK5boost6detail8function13basic_vtable1IvN3RBX10Controller6ButtonEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::Controller::Button>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvN3RBX10Controller6ButtonEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_f49d34() -> ! {
    todo!("0xf49d34 void boost::detail::function::basic_vtable1<void,RBX::Controller::Button>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf49d44 — j___ZNK5boost6detail8function13basic_vtable1IvN3RBX10Controller6ButtonEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Controller::Button>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvN3RBX10Controller6ButtonEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_f49d44() -> ! {
    todo!("0xf49d44 bool boost::detail::function::basic_vtable1<void,RBX::Controller::Button>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0xf49d54 — j___ZNK5boost6detail8function13basic_vtable1IvN3RBX10Controller6ButtonEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Controller::Button>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvN3RBX10Controller6ButtonEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_f49d54() -> ! {
    todo!("0xf49d54 bool boost::detail::function::basic_vtable1<void,RBX::Controller::Button>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Controller::Button const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}
