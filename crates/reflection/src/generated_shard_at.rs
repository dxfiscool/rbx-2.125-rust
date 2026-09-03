// Auto-generated shard AT — next 100 RBX::Reflection stubs — EA-sorted ascending 0x95a0e4..0x9730a0 (remaining 4290) — starts 0x95a0e4
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total, 11781 prior -> 11881 total)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr
#![allow(unused_imports)]
use rbx_core::SharedPtr;

// 0x95a0e4 — __ZN3RBX10Reflection7VariantaSINS_7Network6Player14MembershipTypeEEERS1_RKT_
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::Network::Player::MembershipType>(RBX::Network::Player::MembershipType const&)")]
#[doc(alias = "__ZN3RBX10Reflection7VariantaSINS_7Network6Player14MembershipTypeEEERS1_RKT_")]
pub fn stub_95a0e4() -> ! {
    todo!("0x95a0e4 RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::Network::Player::MembershipType>(RBX::Network::Player::MembershipType const&)")
}

// 0x95a2a8 — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player14MembershipTypeEE13convertToItemERKS4_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::convertToItem(RBX::Network::Player::MembershipType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network6Player14MembershipTypeEE13convertToItemERKS4_")]
pub fn stub_95a2a8(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x95a2a8: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x95a39c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Network12FilterResultEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Network::FilterResult> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Network12FilterResultEEEE13initSingletonEv")]
pub fn stub_95a39c() {
    // IDA 0x95a39c: Singleton<EnumDesc<T>>::initSingleton (no doGet stub found in-crate): cutover no-op.
}

// 0x95a480 — __ZN3RBX10Reflection8EnumDescINS_7Network12FilterResultEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Network12FilterResultEED0Ev")]
pub fn stub_95a480() {
    // IDA 0x95a480: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x95a520 — __ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE6lookupEPKc")]
pub fn stub_95a520(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0x95a520: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x95a5b0 — __ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE6lookupERKNS0_7VariantE")]
pub fn stub_95a5b0(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x95a5b0: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x95a6b4 — __ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::convertToString(RBX::Network::FilterResult const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE15convertToStringERKS3_")]
pub fn stub_95a6b4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x95a6b4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x95a860 — __ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::convertToItem(RBX::Network::FilterResult const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE13convertToItemERKS3_")]
pub fn stub_95a860(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x95a860: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x95a92c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescI17PacketReliabilityEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<PacketReliability> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescI17PacketReliabilityEEE13initSingletonEv")]
pub fn stub_95a92c() {
    // IDA 0x95a92c: Singleton<EnumDesc<T>>::initSingleton (no doGet stub found in-crate): cutover no-op.
}

// 0x95aa10 — __ZN3RBX10Reflection8EnumDescI17PacketReliabilityED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<PacketReliability>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescI17PacketReliabilityED1Ev")]
pub fn stub_95aa10() {
    // IDA 0x95aa10: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x95aa1c — __ZN3RBX10Reflection8EnumDescI17PacketReliabilityED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<PacketReliability>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescI17PacketReliabilityED2Ev")]
pub fn stub_95aa1c() {
    // IDA 0x95aa1c: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x95ac98 — __ZN3RBX10Reflection8EnumDescI17PacketReliabilityED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<PacketReliability>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescI17PacketReliabilityED0Ev")]
pub fn stub_95ac98() {
    // IDA 0x95ac98: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x95ad38 — __ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<PacketReliability>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE6lookupEPKc")]
pub fn stub_95ad38(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0x95ad38: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x95adc8 — __ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<PacketReliability>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE6lookupERKNS0_7VariantE")]
pub fn stub_95adc8(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x95adc8: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x95aecc — __ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<PacketReliability>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE14convertToValueEmRNS0_7VariantE")]
pub fn stub_95aecc() {
    // IDA 0x95aecc: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0x95aef4 — __ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<PacketReliability>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE15convertToStringEmRSs")]
pub fn stub_95aef4(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0x95aef4: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0x95b038 — __ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE15convertToStringERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<PacketReliability>::convertToString(PacketReliability const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE15convertToStringERKS2_")]
pub fn stub_95b038(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x95b038: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x95b1d8 — __ZN3RBX10Reflection7VariantaSI17PacketReliabilityEERS1_RKT_
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<PacketReliability>(PacketReliability const&)")]
#[doc(alias = "__ZN3RBX10Reflection7VariantaSI17PacketReliabilityEERS1_RKT_")]
pub fn stub_95b1d8() -> ! {
    todo!("0x95b1d8 RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<PacketReliability>(PacketReliability const&)")
}

// 0x95b39c — __ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE13convertToItemERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<PacketReliability>::convertToItem(PacketReliability const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI17PacketReliabilityE13convertToItemERKS2_")]
pub fn stub_95b39c(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x95b39c: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x95b490 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescI14PacketPriorityEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<PacketPriority> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescI14PacketPriorityEEE13initSingletonEv")]
pub fn stub_95b490() {
    // IDA 0x95b490: Singleton<EnumDesc<T>>::initSingleton (no doGet stub found in-crate): cutover no-op.
}

// 0x95b574 — __ZN3RBX10Reflection8EnumDescI14PacketPriorityED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<PacketPriority>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescI14PacketPriorityED1Ev")]
pub fn stub_95b574() {
    // IDA 0x95b574: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x95b580 — __ZN3RBX10Reflection8EnumDescI14PacketPriorityED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<PacketPriority>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescI14PacketPriorityED2Ev")]
pub fn stub_95b580() {
    // IDA 0x95b580: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x95b7fc — __ZN3RBX10Reflection8EnumDescI14PacketPriorityED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<PacketPriority>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescI14PacketPriorityED0Ev")]
pub fn stub_95b7fc() {
    // IDA 0x95b7fc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x95b89c — __ZNK3RBX10Reflection8EnumDescI14PacketPriorityE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<PacketPriority>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI14PacketPriorityE6lookupEPKc")]
pub fn stub_95b89c(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0x95b89c: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x95b92c — __ZNK3RBX10Reflection8EnumDescI14PacketPriorityE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<PacketPriority>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI14PacketPriorityE6lookupERKNS0_7VariantE")]
pub fn stub_95b92c(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x95b92c: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x95ba30 — __ZNK3RBX10Reflection8EnumDescI14PacketPriorityE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<PacketPriority>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI14PacketPriorityE14convertToValueEmRNS0_7VariantE")]
pub fn stub_95ba30() {
    // IDA 0x95ba30: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0x95ba58 — __ZNK3RBX10Reflection8EnumDescI14PacketPriorityE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<PacketPriority>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI14PacketPriorityE15convertToStringEmRSs")]
pub fn stub_95ba58(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0x95ba58: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0x95bb9c — __ZNK3RBX10Reflection8EnumDescI14PacketPriorityE15convertToStringERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<PacketPriority>::convertToString(PacketPriority const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI14PacketPriorityE15convertToStringERKS2_")]
pub fn stub_95bb9c(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0x95bb9c: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0x95bd3c — __ZN3RBX10Reflection7VariantaSI14PacketPriorityEERS1_RKT_
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<PacketPriority>(PacketPriority const&)")]
#[doc(alias = "__ZN3RBX10Reflection7VariantaSI14PacketPriorityEERS1_RKT_")]
pub fn stub_95bd3c() -> ! {
    todo!("0x95bd3c RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<PacketPriority>(PacketPriority const&)")
}

// 0x95bf00 — __ZNK3RBX10Reflection8EnumDescI14PacketPriorityE13convertToItemERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<PacketPriority>::convertToItem(PacketPriority const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescI14PacketPriorityE13convertToItemERKS2_")]
pub fn stub_95bf00(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0x95bf00: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0x95d5d0 — __ZN3RBX7Network13serializeEnumEPKNS_10Reflection14EnumDescriptorERKNS1_7VariantERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::serializeEnum(RBX::Reflection::EnumDescriptor const*,RBX::Reflection::Variant const&,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network13serializeEnumEPKNS_10Reflection14EnumDescriptorERKNS1_7VariantERN6RakNet9BitStreamE")]
pub fn stub_95d5d0() -> ! {
    todo!("0x95d5d0 RBX::Network::serializeEnum(RBX::Reflection::EnumDescriptor const*,RBX::Reflection::Variant const&,RakNet::BitStream &)")
}

// 0x95d694 — __ZN3RBX7Network15deserializeEnumEPKNS_10Reflection14EnumDescriptorERNS1_7VariantERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::deserializeEnum(RBX::Reflection::EnumDescriptor const*,RBX::Reflection::Variant &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network15deserializeEnumEPKNS_10Reflection14EnumDescriptorERNS1_7VariantERN6RakNet9BitStreamE")]
pub fn stub_95d694() -> ! {
    todo!("0x95d694 RBX::Network::deserializeEnum(RBX::Reflection::EnumDescriptor const*,RBX::Reflection::Variant &,RakNet::BitStream &)")
}

// 0x95d968 — __ZN3RBX7Network21serializeEnumPropertyERKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::serializeEnumProperty(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network21serializeEnumPropertyERKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE")]
pub fn stub_95d968() -> ! {
    todo!("0x95d968 RBX::Network::serializeEnumProperty(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")
}

// 0x95da34 — __ZN3RBX7Network23deserializeEnumPropertyERNS_10Reflection8PropertyERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::deserializeEnumProperty(RBX::Reflection::Property &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network23deserializeEnumPropertyERNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
pub fn stub_95da34() -> ! {
    todo!("0x95da34 RBX::Network::deserializeEnumProperty(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x95fe40 — __ZN3RBX7Network9serializeINS_9ContentIdEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::serialize<RBX::ContentId>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network9serializeINS_9ContentIdEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE")]
pub fn stub_95fe40() -> ! {
    todo!("0x95fe40 void RBX::Network::serialize<RBX::ContentId>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")
}

// 0x95ff60 — __ZN3RBX7Network9serializeINS_4UDimEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::serialize<RBX::UDim>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network9serializeINS_4UDimEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE")]
pub fn stub_95ff60() -> ! {
    todo!("0x95ff60 void RBX::Network::serialize<RBX::UDim>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")
}

// 0x95ff8c — __ZN3RBX7Network11deserializeINS_4UDimEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::deserialize<RBX::UDim>(RBX::Reflection::Property &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network11deserializeINS_4UDimEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
pub fn stub_95ff8c() -> ! {
    todo!("0x95ff8c void RBX::Network::deserialize<RBX::UDim>(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x95ffc4 — __ZN3RBX7Network9serializeINS_5UDim2EEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::serialize<RBX::UDim2>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network9serializeINS_5UDim2EEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE")]
pub fn stub_95ffc4() -> ! {
    todo!("0x95ffc4 void RBX::Network::serialize<RBX::UDim2>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")
}

// 0x960008 — __ZN3RBX7Network11deserializeINS_5UDim2EEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::deserialize<RBX::UDim2>(RBX::Reflection::Property &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network11deserializeINS_5UDim2EEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
pub fn stub_960008() -> ! {
    todo!("0x960008 void RBX::Network::deserialize<RBX::UDim2>(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x960064 — __ZN3RBX7Network9serializeINS_6RbxRayEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::serialize<RBX::RbxRay>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network9serializeINS_6RbxRayEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE")]
pub fn stub_960064() -> ! {
    todo!("0x960064 void RBX::Network::serialize<RBX::RbxRay>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")
}

// 0x9600f0 — __ZN3RBX7Network11deserializeINS_6RbxRayEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::deserialize<RBX::RbxRay>(RBX::Reflection::Property &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network11deserializeINS_6RbxRayEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
pub fn stub_9600f0() -> ! {
    todo!("0x9600f0 void RBX::Network::deserialize<RBX::RbxRay>(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x960178 — __ZN3RBX7Network9serializeINS_5FacesEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::serialize<RBX::Faces>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network9serializeINS_5FacesEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE")]
pub fn stub_960178() -> ! {
    todo!("0x960178 void RBX::Network::serialize<RBX::Faces>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")
}

// 0x960194 — __ZN3RBX7Network11deserializeINS_5FacesEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::deserialize<RBX::Faces>(RBX::Reflection::Property &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network11deserializeINS_5FacesEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
pub fn stub_960194() -> ! {
    todo!("0x960194 void RBX::Network::deserialize<RBX::Faces>(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x9601c0 — __ZN3RBX7Network9serializeINS_4AxesEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::serialize<RBX::Axes>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network9serializeINS_4AxesEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE")]
pub fn stub_9601c0() -> ! {
    todo!("0x9601c0 void RBX::Network::serialize<RBX::Axes>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")
}

// 0x9601dc — __ZN3RBX7Network11deserializeINS_4AxesEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::deserialize<RBX::Axes>(RBX::Reflection::Property &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network11deserializeINS_4AxesEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
pub fn stub_9601dc() -> ! {
    todo!("0x9601dc void RBX::Network::deserialize<RBX::Axes>(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x960208 — __ZN3RBX7Network9serializeINS_10BrickColorEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::serialize<RBX::BrickColor>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network9serializeINS_10BrickColorEEEvRKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE")]
pub fn stub_960208() -> ! {
    todo!("0x960208 void RBX::Network::serialize<RBX::BrickColor>(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")
}

// 0x960234 — __ZN3RBX7Network11deserializeINS_10BrickColorEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::deserialize<RBX::BrickColor>(RBX::Reflection::Property &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network11deserializeINS_10BrickColorEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
pub fn stub_960234() -> ! {
    todo!("0x960234 void RBX::Network::deserialize<RBX::BrickColor>(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x96025c — __ZN3RBX7Network11deserializeINS_9ContentIdEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE
#[doc(alias = "void RBX::Network::deserialize<RBX::ContentId>(RBX::Reflection::Property &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network11deserializeINS_9ContentIdEEEvRNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
pub fn stub_96025c() -> ! {
    todo!("0x96025c void RBX::Network::deserialize<RBX::ContentId>(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x960380 — __ZN3RBX7Network25deserializeStringPropertyERNS_10Reflection8PropertyERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::deserializeStringProperty(RBX::Reflection::Property &,RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network25deserializeStringPropertyERNS_10Reflection8PropertyERN6RakNet9BitStreamE")]
pub fn stub_960380() -> ! {
    todo!("0x960380 RBX::Network::deserializeStringProperty(RBX::Reflection::Property &,RakNet::BitStream &)")
}

// 0x961178 — __ZN3RBX7Network12IdSerializer13addPendingRefEPKNS_10Reflection21RefPropertyDescriptorEN5boost10shared_ptrINS_8InstanceEEENS_4Guid4DataE
#[doc(alias = "RBX::Network::IdSerializer::addPendingRef(RBX::Reflection::RefPropertyDescriptor const*,rbx_core::SharedPtr<RBX::Instance>,RBX::Guid::Data)")]
#[doc(alias = "__ZN3RBX7Network12IdSerializer13addPendingRefEPKNS_10Reflection21RefPropertyDescriptorEN5boost10shared_ptrINS_8InstanceEEENS_4Guid4DataE")]
pub fn stub_961178() -> ! {
    todo!("0x961178 RBX::Network::IdSerializer::addPendingRef(RBX::Reflection::RefPropertyDescriptor const*,rbx_core::SharedPtr<RBX::Instance>,RBX::Guid::Data)")
}

// 0x961480 — __ZNK3RBX7Network16DescriptorSenderINS_10Reflection15ClassDescriptorEE9teachNameEPKS3_
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::ClassDescriptor>::teachName(RBX::Reflection::ClassDescriptor const*)const")]
#[doc(alias = "__ZNK3RBX7Network16DescriptorSenderINS_10Reflection15ClassDescriptorEE9teachNameEPKS3_")]
pub fn stub_961480() -> ! {
    todo!("0x961480 RBX::Network::DescriptorSender<RBX::Reflection::ClassDescriptor>::teachName(RBX::Reflection::ClassDescriptor const*)const")
}

// 0x961490 — __ZN3RBX7Network18DescriptorReceiverINS_10Reflection15ClassDescriptorEE9learnNameESsi
#[doc(alias = "RBX::Network::DescriptorReceiver<RBX::Reflection::ClassDescriptor>::learnName(std::string,int)")]
#[doc(alias = "__ZN3RBX7Network18DescriptorReceiverINS_10Reflection15ClassDescriptorEE9learnNameESsi")]
pub fn stub_961490() -> ! {
    todo!("0x961490 RBX::Network::DescriptorReceiver<RBX::Reflection::ClassDescriptor>::learnName(std::string,int)")
}

// 0x961700 — __ZNK3RBX7Network16DescriptorSenderINS_10Reflection15EventDescriptorEE9teachNameEPKS3_
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::EventDescriptor>::teachName(RBX::Reflection::EventDescriptor const*)const")]
#[doc(alias = "__ZNK3RBX7Network16DescriptorSenderINS_10Reflection15EventDescriptorEE9teachNameEPKS3_")]
pub fn stub_961700() -> ! {
    todo!("0x961700 RBX::Network::DescriptorSender<RBX::Reflection::EventDescriptor>::teachName(RBX::Reflection::EventDescriptor const*)const")
}

// 0x9618c4 — __ZN3RBX7Network18DescriptorReceiverINS_10Reflection15EventDescriptorEE9learnNameESsi
#[doc(alias = "RBX::Network::DescriptorReceiver<RBX::Reflection::EventDescriptor>::learnName(std::string,int)")]
#[doc(alias = "__ZN3RBX7Network18DescriptorReceiverINS_10Reflection15EventDescriptorEE9learnNameESsi")]
pub fn stub_9618c4() -> ! {
    todo!("0x9618c4 RBX::Network::DescriptorReceiver<RBX::Reflection::EventDescriptor>::learnName(std::string,int)")
}

// 0x961ca4 — __ZNK3RBX7Network16DescriptorSenderINS_10Reflection18PropertyDescriptorEE9teachNameEPKS3_
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::PropertyDescriptor>::teachName(RBX::Reflection::PropertyDescriptor const*)const")]
#[doc(alias = "__ZNK3RBX7Network16DescriptorSenderINS_10Reflection18PropertyDescriptorEE9teachNameEPKS3_")]
pub fn stub_961ca4() -> ! {
    todo!("0x961ca4 RBX::Network::DescriptorSender<RBX::Reflection::PropertyDescriptor>::teachName(RBX::Reflection::PropertyDescriptor const*)const")
}

// 0x961e68 — __ZN3RBX7Network18DescriptorReceiverINS_10Reflection18PropertyDescriptorEE9learnNameESsi
#[doc(alias = "RBX::Network::DescriptorReceiver<RBX::Reflection::PropertyDescriptor>::learnName(std::string,int)")]
#[doc(alias = "__ZN3RBX7Network18DescriptorReceiverINS_10Reflection18PropertyDescriptorEE9learnNameESsi")]
pub fn stub_961e68() -> ! {
    todo!("0x961e68 RBX::Network::DescriptorReceiver<RBX::Reflection::PropertyDescriptor>::learnName(std::string,int)")
}

// 0x96208c — __ZNK3RBX7Network16DescriptorSenderINS_10Reflection4TypeEE9teachNameEPKS3_
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::Type>::teachName(RBX::Reflection::Type const*)const")]
#[doc(alias = "__ZNK3RBX7Network16DescriptorSenderINS_10Reflection4TypeEE9teachNameEPKS3_")]
pub fn stub_96208c() -> ! {
    todo!("0x96208c RBX::Network::DescriptorSender<RBX::Reflection::Type>::teachName(RBX::Reflection::Type const*)const")
}

// 0x96209c — __ZN3RBX7Network18DescriptorReceiverINS_10Reflection4TypeEE9learnNameESsi
#[doc(alias = "RBX::Network::DescriptorReceiver<RBX::Reflection::Type>::learnName(std::string,int)")]
#[doc(alias = "__ZN3RBX7Network18DescriptorReceiverINS_10Reflection4TypeEE9learnNameESsi")]
pub fn stub_96209c() -> ! {
    todo!("0x96209c RBX::Network::DescriptorReceiver<RBX::Reflection::Type>::learnName(std::string,int)")
}

// 0x962300 — __ZN3RBX7Network16DescriptorSenderINS_10Reflection15ClassDescriptorEEC2Ev
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::ClassDescriptor>::DescriptorSender(void)")]
#[doc(alias = "__ZN3RBX7Network16DescriptorSenderINS_10Reflection15ClassDescriptorEEC2Ev")]
pub fn stub_962300() -> ! {
    todo!("0x962300 RBX::Network::DescriptorSender<RBX::Reflection::ClassDescriptor>::DescriptorSender(void)")
}

// 0x962464 — __ZN3RBX7Network16DescriptorSenderINS_10Reflection18PropertyDescriptorEEC2Ev
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::PropertyDescriptor>::DescriptorSender(void)")]
#[doc(alias = "__ZN3RBX7Network16DescriptorSenderINS_10Reflection18PropertyDescriptorEEC2Ev")]
pub fn stub_962464() -> ! {
    todo!("0x962464 RBX::Network::DescriptorSender<RBX::Reflection::PropertyDescriptor>::DescriptorSender(void)")
}

// 0x962694 — __ZN3RBX7Network16DescriptorSenderINS_10Reflection15EventDescriptorEEC2Ev
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::EventDescriptor>::DescriptorSender(void)")]
#[doc(alias = "__ZN3RBX7Network16DescriptorSenderINS_10Reflection15EventDescriptorEEC2Ev")]
pub fn stub_962694() -> ! {
    todo!("0x962694 RBX::Network::DescriptorSender<RBX::Reflection::EventDescriptor>::DescriptorSender(void)")
}

// 0x9628c4 — __ZN3RBX7Network16DescriptorSenderINS_10Reflection4TypeEEC2Ev
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::Type>::DescriptorSender(void)")]
#[doc(alias = "__ZN3RBX7Network16DescriptorSenderINS_10Reflection4TypeEEC2Ev")]
pub fn stub_9628c4() -> ! {
    todo!("0x9628c4 RBX::Network::DescriptorSender<RBX::Reflection::Type>::DescriptorSender(void)")
}

// 0x963b20 — __ZNK3RBX10Reflection13ConstProperty8getValueINS_4UDimEEET_v
#[doc(alias = "RBX::UDim RBX::Reflection::ConstProperty::getValue<RBX::UDim>(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection13ConstProperty8getValueINS_4UDimEEET_v")]
pub fn stub_963b20() -> ! {
    todo!("0x963b20 RBX::UDim RBX::Reflection::ConstProperty::getValue<RBX::UDim>(void)const")
}

// 0x963c08 — __ZN3RBX10Reflection8Property8setValueINS_4UDimEEEvRKT_
#[doc(alias = "void RBX::Reflection::Property::setValue<RBX::UDim>(RBX::UDim const&)")]
#[doc(alias = "__ZN3RBX10Reflection8Property8setValueINS_4UDimEEEvRKT_")]
pub fn stub_963c08() -> ! {
    todo!("0x963c08 void RBX::Reflection::Property::setValue<RBX::UDim>(RBX::UDim const&)")
}

// 0x963cf0 — __ZNK3RBX10Reflection13ConstProperty8getValueINS_5UDim2EEET_v
#[doc(alias = "RBX::UDim2 RBX::Reflection::ConstProperty::getValue<RBX::UDim2>(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection13ConstProperty8getValueINS_5UDim2EEET_v")]
pub fn stub_963cf0() -> ! {
    todo!("0x963cf0 RBX::UDim2 RBX::Reflection::ConstProperty::getValue<RBX::UDim2>(void)const")
}

// 0x963dd8 — __ZN3RBX10Reflection8Property8setValueINS_5UDim2EEEvRKT_
#[doc(alias = "void RBX::Reflection::Property::setValue<RBX::UDim2>(RBX::UDim2 const&)")]
#[doc(alias = "__ZN3RBX10Reflection8Property8setValueINS_5UDim2EEEvRKT_")]
pub fn stub_963dd8() -> ! {
    todo!("0x963dd8 void RBX::Reflection::Property::setValue<RBX::UDim2>(RBX::UDim2 const&)")
}

// 0x963ec0 — __ZNK3RBX10Reflection13ConstProperty8getValueINS_6RbxRayEEET_v
#[doc(alias = "RBX::RbxRay RBX::Reflection::ConstProperty::getValue<RBX::RbxRay>(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection13ConstProperty8getValueINS_6RbxRayEEET_v")]
pub fn stub_963ec0() -> ! {
    todo!("0x963ec0 RBX::RbxRay RBX::Reflection::ConstProperty::getValue<RBX::RbxRay>(void)const")
}

// 0x963fa8 — __ZN3RBX10Reflection8Property8setValueINS_6RbxRayEEEvRKT_
#[doc(alias = "void RBX::Reflection::Property::setValue<RBX::RbxRay>(RBX::RbxRay const&)")]
#[doc(alias = "__ZN3RBX10Reflection8Property8setValueINS_6RbxRayEEEvRKT_")]
pub fn stub_963fa8() -> ! {
    todo!("0x963fa8 void RBX::Reflection::Property::setValue<RBX::RbxRay>(RBX::RbxRay const&)")
}

// 0x964090 — __ZNK3RBX10Reflection13ConstProperty8getValueINS_5FacesEEET_v
#[doc(alias = "RBX::Faces RBX::Reflection::ConstProperty::getValue<RBX::Faces>(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection13ConstProperty8getValueINS_5FacesEEET_v")]
pub fn stub_964090() -> ! {
    todo!("0x964090 RBX::Faces RBX::Reflection::ConstProperty::getValue<RBX::Faces>(void)const")
}

// 0x964174 — __ZN3RBX10Reflection8Property8setValueINS_5FacesEEEvRKT_
#[doc(alias = "void RBX::Reflection::Property::setValue<RBX::Faces>(RBX::Faces const&)")]
#[doc(alias = "__ZN3RBX10Reflection8Property8setValueINS_5FacesEEEvRKT_")]
pub fn stub_964174() -> ! {
    todo!("0x964174 void RBX::Reflection::Property::setValue<RBX::Faces>(RBX::Faces const&)")
}

// 0x96425c — __ZNK3RBX10Reflection13ConstProperty8getValueINS_4AxesEEET_v
#[doc(alias = "RBX::Axes RBX::Reflection::ConstProperty::getValue<RBX::Axes>(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection13ConstProperty8getValueINS_4AxesEEET_v")]
pub fn stub_96425c() -> ! {
    todo!("0x96425c RBX::Axes RBX::Reflection::ConstProperty::getValue<RBX::Axes>(void)const")
}

// 0x964340 — __ZN3RBX10Reflection8Property8setValueINS_4AxesEEEvRKT_
#[doc(alias = "void RBX::Reflection::Property::setValue<RBX::Axes>(RBX::Axes const&)")]
#[doc(alias = "__ZN3RBX10Reflection8Property8setValueINS_4AxesEEEvRKT_")]
pub fn stub_964340() -> ! {
    todo!("0x964340 void RBX::Reflection::Property::setValue<RBX::Axes>(RBX::Axes const&)")
}

// 0x964428 — __ZNK3RBX10Reflection13ConstProperty8getValueINS_10BrickColorEEET_v
#[doc(alias = "RBX::BrickColor RBX::Reflection::ConstProperty::getValue<RBX::BrickColor>(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection13ConstProperty8getValueINS_10BrickColorEEET_v")]
pub fn stub_964428() -> ! {
    todo!("0x964428 RBX::BrickColor RBX::Reflection::ConstProperty::getValue<RBX::BrickColor>(void)const")
}

// 0x964510 — __ZN3RBX10Reflection8Property8setValueINS_10BrickColorEEEvRKT_
#[doc(alias = "void RBX::Reflection::Property::setValue<RBX::BrickColor>(RBX::BrickColor const&)")]
#[doc(alias = "__ZN3RBX10Reflection8Property8setValueINS_10BrickColorEEEvRKT_")]
pub fn stub_964510() -> ! {
    todo!("0x964510 void RBX::Reflection::Property::setValue<RBX::BrickColor>(RBX::BrickColor const&)")
}

// 0x96481c — __ZNSt8_Rb_treeIPKN3RBX10Reflection4TypeESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Reflection::Type const*,std::pair<RBX::Reflection::Type const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::Type const* const,unsigned int>>,std::less<RBX::Reflection::Type const*>,std::allocator<std::pair<RBX::Reflection::Type const* const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::Type const* const,unsigned int>>,std::pair<RBX::Reflection::Type const* const,unsigned int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX10Reflection4TypeESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
pub fn stub_96481c() -> ! {
    todo!("0x96481c std::_Rb_tree<RBX::Reflection::Type const*,std::pair<RBX::Reflection::Type const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::Type const* const,unsigned int>>,std::less<RBX::Reflection::Type const*>,std::allocator<std::pair<RBX::Reflection::Type const* const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::Type const* const,unsigned int>>,std::pair<RBX::Reflection::Type const* const,unsigned int> const&)")
}

// 0x9649d0 — __ZNSt8_Rb_treeIPKN3RBX10Reflection4TypeESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Reflection::Type const*,std::pair<RBX::Reflection::Type const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::Type const* const,unsigned int>>,std::less<RBX::Reflection::Type const*>,std::allocator<std::pair<RBX::Reflection::Type const* const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Reflection::Type const* const,unsigned int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX10Reflection4TypeESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_")]
pub fn stub_9649d0() -> ! {
    todo!("0x9649d0 std::_Rb_tree<RBX::Reflection::Type const*,std::pair<RBX::Reflection::Type const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::Type const* const,unsigned int>>,std::less<RBX::Reflection::Type const*>,std::allocator<std::pair<RBX::Reflection::Type const* const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Reflection::Type const* const,unsigned int> const&)")
}

// 0x964ac0 — __ZNSt8_Rb_treeIPKN3RBX10Reflection15EventDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Reflection::EventDescriptor const*,std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::EventDescriptor const*>,std::allocator<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>,std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX10Reflection15EventDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
pub fn stub_964ac0() -> ! {
    todo!("0x964ac0 std::_Rb_tree<RBX::Reflection::EventDescriptor const*,std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::EventDescriptor const*>,std::allocator<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>,std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int> const&)")
}

// 0x964c74 — __ZNSt8_Rb_treeIPKN3RBX10Reflection15EventDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Reflection::EventDescriptor const*,std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::EventDescriptor const*>,std::allocator<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX10Reflection15EventDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_")]
pub fn stub_964c74() -> ! {
    todo!("0x964c74 std::_Rb_tree<RBX::Reflection::EventDescriptor const*,std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::EventDescriptor const*>,std::allocator<std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Reflection::EventDescriptor const* const,unsigned int> const&)")
}

// 0x964d64 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>,std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
pub fn stub_964d64() -> ! {
    todo!("0x964d64 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>,std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int> const&)")
}

// 0x964f18 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_")]
pub fn stub_964f18() -> ! {
    todo!("0x964f18 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Reflection::PropertyDescriptor const* const,unsigned int> const&)")
}

// 0x965008 — __ZNSt8_Rb_treeIPKN3RBX10Reflection15ClassDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Reflection::ClassDescriptor const*,std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::ClassDescriptor const*>,std::allocator<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>,std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX10Reflection15ClassDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
pub fn stub_965008() -> ! {
    todo!("0x965008 std::_Rb_tree<RBX::Reflection::ClassDescriptor const*,std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::ClassDescriptor const*>,std::allocator<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>,std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int> const&)")
}

// 0x9651bc — __ZNSt8_Rb_treeIPKN3RBX10Reflection15ClassDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Reflection::ClassDescriptor const*,std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::ClassDescriptor const*>,std::allocator<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX10Reflection15ClassDescriptorESt4pairIKS4_jESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_")]
pub fn stub_9651bc() -> ! {
    todo!("0x9651bc std::_Rb_tree<RBX::Reflection::ClassDescriptor const*,std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>,std::_Select1st<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>,std::less<RBX::Reflection::ClassDescriptor const*>,std::allocator<std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Reflection::ClassDescriptor const* const,unsigned int> const&)")
}

// 0x965efc — __ZNK5boost9unordered13unordered_mapIPKcPN3RBX10Reflection15EventDescriptorENS5_19StringHashPredicateENS5_20StringEqualPredicateESaISt4pairIKS3_S7_EEE4findERSB_
#[doc(alias = "boost::unordered::unordered_map<char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate,std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>>::find(char const* const&)const")]
#[doc(alias = "__ZNK5boost9unordered13unordered_mapIPKcPN3RBX10Reflection15EventDescriptorENS5_19StringHashPredicateENS5_20StringEqualPredicateESaISt4pairIKS3_S7_EEE4findERSB_")]
pub fn stub_965efc() -> ! {
    todo!("0x965efc boost::unordered::unordered_map<char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate,std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>>::find(char const* const&)const")
}

// 0x96c484 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFN5boost10shared_ptrINS_8InstanceEEEiSsiiiELi5EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,rbx_core::SharedPtr<RBX::Instance> ()(int,std::string,int,int,int),5>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFN5boost10shared_ptrINS_8InstanceEEEiSsiiiELi5EED1Ev")]
pub fn stub_96c484() {
    // IDA 0x96c484: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x96c490 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFviELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,void ()(int),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFviELi1EED1Ev")]
pub fn stub_96c490() {
    // IDA 0x96c490: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x96c4f8 — __ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Client::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev")]
pub fn stub_96c4f8() {
    // IDA 0x96c4f8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x96c540 — __ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Client::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev")]
pub fn stub_96c540() {
    // IDA 0x96c540: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x96c588 — __ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev")]
pub fn stub_96c588() {
    // IDA 0x96c588: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x96fd88 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network16ClientReplicatorES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ClientReplicator,RBX::Network::ClientReplicator>(rbx_core::SharedPtr<RBX::Network::ClientReplicator> const*,RBX::Network::ClientReplicator *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network16ClientReplicatorES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_96fd88() {
    // IDA 0x96fd88: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x9709b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network6ClientES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Client,RBX::Network::Client>(rbx_core::SharedPtr<RBX::Network::Client> const*,RBX::Network::Client *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network6ClientES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_9709b0() {
    // IDA 0x9709b0: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x970f5c — __ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_970f5c() -> ! {
    todo!("0x970f5c RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x97133c — __ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_ED0Ev")]
pub fn stub_97133c() {
    // IDA 0x97133c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x971418 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_971418() -> ! {
    todo!("0x971418 RBX::Reflection::EventDescImpl<3,RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x97189c — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
pub fn stub_97189c() -> ! {
    todo!("0x97189c RBX::Reflection::EventDescImpl<3,RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x971bec — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_971bec() -> ! {
    todo!("0x971bec RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x971db4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,int,std::string)> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE")]
pub fn stub_971db4() -> ! {
    todo!("0x971db4 RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,int,std::string)> const&)const")
}

// 0x971f88 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKiS5_NS_10shared_ptrIS3_EENS_3argILi1EEENSA_ILi2EEENSA_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISG_T0_T1_T2_T3_EENSE_9list_av_4IT4_T5_T6_T7_E4typeEEEMSJ_FSG_SK_SL_SM_ESP_SQ_SR_SS_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,std::string const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,std::string const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,int const&,std::string const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKiS5_NS_10shared_ptrIS3_EENS_3argILi1EEENSA_ILi2EEENSA_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISG_T0_T1_T2_T3_EENSE_9list_av_4IT4_T5_T6_T7_E4typeEEEMSJ_FSG_SK_SL_SM_ESP_SQ_SR_SS_")]
pub fn stub_971f88() -> ! {
    todo!("0x971f88 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,std::string const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,std::string const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,int const&,std::string const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x9723f4 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3ISsiSsEEvRKT_RKT0_RKT1_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<std::string,int,std::string>(std::string const&,int const&,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute3ISsiSsEEvRKT_RKT0_RKT1_")]
pub fn stub_9723f4() -> ! {
    todo!("0x9723f4 void RBX::Reflection::GenericSlotWrapper::execute3<std::string,int,std::string>(std::string const&,int const&,std::string const&)")
}

// 0x972c28 — __ZN5boost9function3IvSsiSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEEvT_
#[doc(alias = "void boost::function3<void,std::string,int,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
#[doc(alias = "__ZN5boost9function3IvSsiSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEEvT_")]
pub fn stub_972c28() -> ! {
    todo!("0x972c28 void boost::function3<void,std::string,int,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0x9730a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE")]
pub fn stub_9730a0() -> ! {
    todo!("0x9730a0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,std::string const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}
