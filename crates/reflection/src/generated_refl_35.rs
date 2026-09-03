//! Auto-generated refl 35 — 100 stubs EA-sorted asc 0xf3d6c4..0xf3ed34 (Reflection namespace, global-deduped)
//! Source: ida/export.json (85545 funcs) filtered mangled/demangled contains Reflection (RBX::Reflection 16171 total, 1671 remain)
//! Filter: RBX::Reflection namespace, EA asc, skip EAs in /tmp/global_eas.txt

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf3d6c4 — j___ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE15convertToStringERKS3_")]
pub fn stub_0xf3d6c4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d6c4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d6d4 — j___ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE13convertToItemERKS3_")]
pub fn stub_0xf3d6d4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d6d4: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d6e4 — j___ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d6e4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d6e4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d6f4 — j___ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE15convertToStringERKS3_")]
pub fn stub_0xf3d6f4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d6f4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d704 — j___ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE13convertToItemERKS3_")]
pub fn stub_0xf3d704(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d704: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d714 — j___ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d714(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d714: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d724 — j___ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE15convertToStringERKS3_")]
pub fn stub_0xf3d724(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d724: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d734 — j___ZNK3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE13convertToItemERKS3_")]
pub fn stub_0xf3d734(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d734: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d744 — j___ZNK3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d744(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d744: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d754 — j___ZNK3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE15convertToStringERKS3_")]
pub fn stub_0xf3d754(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d754: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d764 — j___ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE13convertToItemERKS3_")]
pub fn stub_0xf3d764(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d764: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d774 — j___ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d774(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d774: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d784 — j___ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE15convertToStringERKS3_")]
pub fn stub_0xf3d784(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d784: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d794 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature5InOutEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature5InOutEE13convertToItemERKS3_")]
pub fn stub_0xf3d794(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d794: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d7a4 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature5InOutEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature5InOutEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d7a4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d7a4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d7b4 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature5InOutEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature5InOutEE15convertToStringERKS3_")]
pub fn stub_0xf3d7b4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d7b4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d7c4 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE13convertToItemERKS3_")]
pub fn stub_0xf3d7c4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d7c4: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d7d4 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d7d4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d7d4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d7e4 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE15convertToStringERKS3_")]
pub fn stub_0xf3d7e4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d7e4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d7f4 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE13convertToItemERKS3_")]
pub fn stub_0xf3d7f4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d7f4: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d804 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d804(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d804: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d814 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE15convertToStringERKS3_")]
pub fn stub_0xf3d814(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d814: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d824 — j___ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE13convertToItemERKS3_")]
pub fn stub_0xf3d824(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d824: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d834 — j___ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d834(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d834: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d844 — j___ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE15convertToStringERKS3_")]
pub fn stub_0xf3d844(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d844: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d8b4 — j___ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d8b4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d8b4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d8c4 — j___ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE15convertToStringERKS3_")]
pub fn stub_0xf3d8c4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d8c4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d8d4 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE13convertToItemERKS3_")]
pub fn stub_0xf3d8d4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d8d4: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d8e4 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d8e4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d8e4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d8f4 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringERKS3_")]
pub fn stub_0xf3d8f4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d8f4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d904 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEE13convertToItemERKS3_")]
pub fn stub_0xf3d904(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d904: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d914 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d914(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d914: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d924 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject11TweenStatusEE15convertToStringERKS3_")]
pub fn stub_0xf3d924(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d924: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d934 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE13convertToItemERKS3_")]
pub fn stub_0xf3d934(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d934: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d944 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d944(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d944: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d954 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject14SizeConstraintEE15convertToStringERKS3_")]
pub fn stub_0xf3d954(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d954: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d964 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEE13convertToItemERKS3_")]
pub fn stub_0xf3d964(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d964: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d974 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d974(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d974: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d984 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject16TweenEasingStyleEE15convertToStringERKS3_")]
pub fn stub_0xf3d984(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d984: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d994 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEE13convertToItemERKS3_")]
pub fn stub_0xf3d994(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d994: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d9a4 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d9a4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d9a4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d9b4 — j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9GuiObject20TweenEasingDirectionEE15convertToStringERKS3_")]
pub fn stub_0xf3d9b4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d9b4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3d9c4 — j___ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE13convertToItemERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE13convertToItemERKS3_")]
pub fn stub_0xf3d9c4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf3d9c4: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf3d9d4 — j___ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf3d9d4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf3d9d4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf3d9e4 — j___ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE15convertToStringERKS3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9HopperBin7BinTypeEE15convertToStringERKS3_")]
pub fn stub_0xf3d9e4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf3d9e4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3da94 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ForceFieldES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ForceFieldES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3da94() {
    // IDA 0xf3da94: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3daa4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11CustomEventES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11CustomEventES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3daa4() {
    // IDA 0xf3daa4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3dab4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13BindableEventES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13BindableEventES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3dab4() {
    // IDA 0xf3dab4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3dac4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16BindableFunctionES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16BindableFunctionES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3dac4() {
    // IDA 0xf3dac4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3dad4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17BasicPartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17BasicPartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3dad4() {
    // IDA 0xf3dad4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3dae4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CustomEventReceiverES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CustomEventReceiverES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3dae4() {
    // IDA 0xf3dae4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3daf4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8SparklesES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8SparklesES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3daf4() {
    // IDA 0xf3daf4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3db04 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9AnimationES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9AnimationES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3db04() {
    // IDA 0xf3db04: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3df04 — j___ZN3RBX10Reflection14PropDescriptorINS_13VelocityMotorEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13VelocityMotorEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3df04() -> ! {
    todo!("0xf3df04 j___ZN3RBX10Reflection14PropDescriptorINS_13VelocityMotorEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3df14 — j___ZN3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3df14() -> ! {
    todo!("0xf3df14 j___ZN3RBX10Reflection17RefPropDescriptorINS_13VelocityMotorENS_4HoleEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3df24 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_5InOutEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_5InOutEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3df24() -> ! {
    todo!("0xf3df24 j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_5InOutEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3df34 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9LeftRightEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9LeftRightEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3df34() -> ! {
    todo!("0xf3df34 j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9LeftRightEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3df44 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9TopBottomEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9TopBottomEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3df44() -> ! {
    todo!("0xf3df44 j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9TopBottomEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3df54 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3df54() -> ! {
    todo!("0xf3df54 j___ZN3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3df64 — j___ZN3RBX10Reflection4TypeC2IPNS_4HoleEEEPKcS6_PT_
#[doc(alias = "j___ZN3RBX10Reflection4TypeC2IPNS_4HoleEEEPKcS6_PT_")]
pub fn stub_0xf3df64() -> ! {
    todo!("0xf3df64 j___ZN3RBX10Reflection4TypeC2IPNS_4HoleEEEPKcS6_PT_")
}

// 0xf3df74 — j___ZN3RBX10Reflection7RefTypeIPNS_4HoleEE9singletonEv
#[doc(alias = "j___ZN3RBX10Reflection7RefTypeIPNS_4HoleEE9singletonEv")]
pub fn stub_0xf3df74() -> ! {
    todo!("0xf3df74 j___ZN3RBX10Reflection7RefTypeIPNS_4HoleEE9singletonEv")
}

// 0xf3df84 — j___ZN3RBX10Reflection8EnumDescINS_7Feature5InOutEE7addPairES3_PKc
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_7Feature5InOutEE7addPairES3_PKc")]
pub fn stub_0xf3df84(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3df84: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3df94 — j___ZN3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE7addPairES3_PKc
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE7addPairES3_PKc")]
pub fn stub_0xf3df94(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3df94: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3dfa4 — j___ZN3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE7addPairES3_PKc
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE7addPairES3_PKc")]
pub fn stub_0xf3dfa4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3dfa4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3e154 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_5InOutEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_5InOutEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf3e154() -> ! {
    todo!("0xf3e154 j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_5InOutEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0xf3e164 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9LeftRightEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9LeftRightEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf3e164() -> ! {
    todo!("0xf3e164 j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9LeftRightEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0xf3e174 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9TopBottomEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9TopBottomEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf3e174() -> ! {
    todo!("0xf3e174 j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS2_9TopBottomEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0xf3e184 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf3e184() -> ! {
    todo!("0xf3e184 j___ZNK3RBX10Reflection18EnumPropDescriptorINS_7FeatureENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0xf3e194 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature5InOutEE14convertToIndexES3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature5InOutEE14convertToIndexES3_")]
pub fn stub_0xf3e194(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3e194: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf3e1a4 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE14convertToIndexES3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE14convertToIndexES3_")]
pub fn stub_0xf3e1a4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3e1a4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf3e1b4 — j___ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE14convertToIndexES3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE14convertToIndexES3_")]
pub fn stub_0xf3e1b4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3e1b4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf3e1f4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12MotorFeatureES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12MotorFeatureES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3e1f4() {
    // IDA 0xf3e1f4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3e204 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13VelocityMotorES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13VelocityMotorES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3e204() {
    // IDA 0xf3e204: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3e214 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4HoleES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4HoleES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3e214() {
    // IDA 0xf3e214: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3e424 — j___ZN3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3e424() -> ! {
    todo!("0xf3e424 j___ZN3RBX10Reflection23TypedPropertyDescriptorINS_6MeshIdEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3e454 — j___ZN3RBX10Reflection14PropDescriptorINS_4FireEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_4FireEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3e454() -> ! {
    todo!("0xf3e454 j___ZN3RBX10Reflection14PropDescriptorINS_4FireEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3e464 — j___ZN3RBX10Reflection14PropDescriptorINS_4FireEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_4FireEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3e464() -> ! {
    todo!("0xf3e464 j___ZN3RBX10Reflection14PropDescriptorINS_4FireEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3e474 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4FireEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4FireEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3e474() -> ! {
    todo!("0xf3e474 j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4FireEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3e504 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4FireES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4FireES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3e504() {
    // IDA 0xf3e504: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3e514 — j___ZN3RBX10Reflection14PropDescriptorINS_4FlagENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_4FlagENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3e514() -> ! {
    todo!("0xf3e514 j___ZN3RBX10Reflection14PropDescriptorINS_4FlagENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3e5c4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4FlagES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4FlagES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3e5c4() {
    // IDA 0xf3e5c4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3e5e4 — j___ZN3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3e5e4() -> ! {
    todo!("0xf3e5e4 j___ZN3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3e7a4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16FlagStandServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16FlagStandServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3e7a4() {
    // IDA 0xf3e7a4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3e7b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9FlagStandES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9FlagStandES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3e7b4() {
    // IDA 0xf3e7b4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3e844 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3e844() -> ! {
    todo!("0xf3e844 j___ZN3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3e854 — j___ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEE7addPairES3_PKc
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEE7addPairES3_PKc")]
pub fn stub_0xf3e854(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3e854: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3e864 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf3e864() -> ! {
    todo!("0xf3e864 j___ZNK3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0xf3e874 — j___ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE14convertToIndexES3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE14convertToIndexES3_")]
pub fn stub_0xf3e874(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3e874: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf3ec44 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_12VideoQualityEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_12VideoQualityEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3ec44() -> ! {
    todo!("0xf3ec44 j___ZN3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_12VideoQualityEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3ec54 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_13UploadSettingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_13UploadSettingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3ec54() -> ! {
    todo!("0xf3ec54 j___ZN3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_13UploadSettingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3ec64 — j___ZN3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE7addPairES3_PKc
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE7addPairES3_PKc")]
pub fn stub_0xf3ec64(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3ec64: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3ec74 — j___ZN3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE7addPairES3_PKc
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE7addPairES3_PKc")]
pub fn stub_0xf3ec74(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3ec74: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3ec84 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_12GameSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_12GameSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3ec84() -> ! {
    todo!("0xf3ec84 j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_12GameSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3ec94 — j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_12GameSettingsEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_12GameSettingsEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3ec94() -> ! {
    todo!("0xf3ec94 j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_12GameSettingsEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3eca4 — j___ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_12GameSettingsEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_12GameSettingsEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3eca4() -> ! {
    todo!("0xf3eca4 j___ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_12GameSettingsEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0xf3ecb4 — j___ZN3RBX10Reflection9EventDescINS_12GameSettingsEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_12GameSettingsEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xf3ecb4() -> ! {
    todo!("0xf3ecb4 j___ZN3RBX10Reflection9EventDescINS_12GameSettingsEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

// 0xf3ed04 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_12VideoQualityEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_12VideoQualityEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf3ed04() -> ! {
    todo!("0xf3ed04 j___ZNK3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_12VideoQualityEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0xf3ed14 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_13UploadSettingEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_13UploadSettingEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf3ed14() -> ! {
    todo!("0xf3ed14 j___ZNK3RBX10Reflection18EnumPropDescriptorINS_12GameSettingsENS2_13UploadSettingEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0xf3ed24 — j___ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE14convertToIndexES3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE14convertToIndexES3_")]
pub fn stub_0xf3ed24(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3ed24: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf3ed34 — j___ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE14convertToIndexES3_
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE14convertToIndexES3_")]
pub fn stub_0xf3ed34(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3ed34: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}
