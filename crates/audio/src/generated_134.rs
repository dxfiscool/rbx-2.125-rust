//! audio generated_134 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Soundscape exhausted (2398 distinct) — filler EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0xf6f8d0..0x106b8 EA-sorted asc filler after 0xf6f8c4, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// Host models for IDA 0xfa00..0x10074 (`Described<CRenderSettingsItem>` +
// `PropDescriptor<CRenderSettingsItem,int>` GetSetImpl +
// `BoundFuncDesc<CRenderSettingsItem,int()>` + `Call0Helper` +
// `EnumPropDescriptor<CRenderSettingsItem,ResolutionPreset>`).
// Member-pointer thunk details (the `a2-36`/`+96` adjusts at 0xfcc0..0xfd06)
// have no host effect.

/// Host model of the `RenderSettings` `ClassDescriptor` singleton (IDA 0xfa00
/// function-local static `describedClassDescriptor`, parented to the base
/// `Instance` descriptor fetched at 0xfa68).
#[derive(Debug)]
pub struct RenderSettingsClassDescriptor {
    pub name: &'static str,
    pub parent: &'static str,
}

/// Minimal host stand-ins for the two objects the 0xfb74/0xfe84 get/set pairs
/// bridge: the global `CRenderSettings` (getter source; IDA 0xb4f4/0xb4f8
/// `+0x40`/`+0x44` cache slots, 0xb4a4 `+0x18` resolution slot) and the
/// `CRenderSettingsItem` instance (setter target; IDA 0x97c0/0x97c8
/// `+160`/`+164` cache slots, 0x97a4 `+120` resolution slot). Only the slots
/// this property family touches are modelled.
#[derive(Default)]
pub struct CacheSettings {
    pub texture_cache_size: u32,
    pub mesh_cache_size: u32,
    pub resolution_preference: i32,
}

/// Minimal host stand-in for the `CRenderSettingsItem` setter target.
#[derive(Default)]
pub struct CacheItem {
    pub texture_cache_size: u32,
    pub mesh_cache_size: u32,
    pub resolution_preset: i32,
}

/// Host carrier for `PropDescriptor<CRenderSettingsItem,int>` with the
/// `GetSetImpl<unsigned int (CRenderSettings::*)() const,
/// void (CRenderSettingsItem::*)(unsigned int)>` pair. `None` models a null
/// member pointer, which is what the image `isReadOnly`/`isWriteOnly`
/// virtuals (and the 0xfe84 flag fixups) test.
pub struct CacheSizeProp {
    pub name: String,
    pub category: String,
    pub getter: Option<fn(&CacheSettings) -> u32>,
    pub setter: Option<fn(&mut CacheItem, u32)>,
    pub attributes: u32,
    pub permissions: u32,
}

impl CacheSizeProp {
    /// IDA 0xfcb4 (disasm 0xfcb4..0xfcb6 `MOVS R0,#0; BX LR`): a bound getter
    /// is never read-only.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0xfcb8 (disasm 0xfcb8..0xfcba `MOVS R0,#0; BX LR`): a bound setter
    /// is never write-only.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}

/// Host carrier for `BoundFuncDesc<CRenderSettingsItem,int()>` (IDA 0xfd0c:
/// member-fn pair at +40, `Type::getSingleton<int>()` at +0x1C).
pub struct BoundFunc0 {
    pub name: String,
    pub permissions: u32,
    pub attributes: u32,
    pub func: fn(&CacheItem) -> i32,
    pub return_type: &'static str,
}

/// Host model of the `(Type const*, Variant)` out-pair IDA 0xfe54 writes:
/// `*a4 = Type::getSingleton<int>()` (0xfe72), then
/// `placement_any<int>(a4 + 1, &result)` (0xfe7a..0xfe80).
pub struct IntCallResult {
    pub type_name: &'static str,
    pub value: i32,
}

/// Host carrier for `EnumPropDescriptor<CRenderSettingsItem,ResolutionPreset>`
/// (IDA 0xfe84): enum singleton at +40/+48 (0xfec8..0xffa4), GetSetImpl
/// {getter, setter} at +44 (0xff62..0xff88), attribute flag fixups at +28
/// from the isReadOnly/isWriteOnly virtuals (0xffb4..0xffda).
pub struct ResolutionProp {
    pub name: String,
    pub category: String,
    pub getter: Option<fn(&CacheSettings) -> i32>,
    pub setter: Option<fn(&mut CacheItem, i32)>,
    pub attributes: u32,
    pub permissions: u32,
    pub enum_type: &'static str,
}

impl ResolutionProp {
    /// IDA 0x10064 (disasm 0x10064..0x10070: load impl at `[a1+0x2C]`,
    /// tail-call its slot-0 virtual): delegates to the +44 GetSetImpl's
    /// isReadOnly.
    pub fn is_read_only(&self) -> bool {
        self.getter.is_none()
    }

    /// IDA 0x10074 (disasm 0x10074..0x10080: same, slot-1 virtual):
    /// delegates to the +44 GetSetImpl's isWriteOnly.
    pub fn is_write_only(&self) -> bool {
        self.setter.is_none()
    }
}
/// Host model of `EnumDesc<ResolutionPreset>` for IDA 0x10084..0x10674.
/// `items` is the ordered (value, name) table (image `+28`/`+32` item vector
/// at 0x10572..0x1058e); `index_to_value` is the legacy index->value map
/// (image `+132` qword at 0x10682 / `+156` qword at 0x1065e) with `-1` holes
/// for unmapped indices. Mirrors `EnumDescModel` in `generated_18.rs`.
#[derive(Default)]
pub struct ResolutionEnumDesc {
    pub items: Vec<(i32, String)>,
    pub index_to_value: Vec<i32>,
}

impl ResolutionEnumDesc {
    /// Host helper mirroring `EnumDescModel::add_pair`: appends the (value,
    /// name) item and records it in the legacy index->value map at `index`
    /// (gaps stay `-1`, as in the image table).
    pub fn add_pair(&mut self, value: i32, name: &str, index: usize) {
        self.items.push((value, name.to_owned()));
        if self.index_to_value.len() <= index {
            self.index_to_value.resize(index + 1, -1);
        }
        self.index_to_value[index] = value;
    }

    /// Host search behind 0x1026c/0x105d0/0x102cc (`Name::lookup` +
    /// `EnumDesc::convertToValue` at 0x1027a..0x1028c, 0x105da..0x105e6,
    /// 0x1037e..0x1039e). `Name::lookup` interning has no host effect; the
    /// host compares names directly.
    pub fn lookup_value(&self, name: &str) -> Option<i32> {
        self.items.iter().find(|(_, n)| n == name).map(|(v, _)| *v)
    }

    /// Host `EnumDesc::convertToString` behind 0x10248 (0x1025c..0x1026a):
    /// assigns the item name for `value`, returns false with `out` untouched
    /// when unmapped.
    pub fn value_to_string(&self, value: i32, out: &mut String) -> bool {
        if let Some((_, name)) = self.items.iter().find(|(v, _)| *v == value) {
            *out = name.clone();
            true
        } else {
            false
        }
    }

    /// Host `EnumDesc::convertToIndex` behind 0x1050c/0x10604: the legacy
    /// index->value table indexed by value, `-1` when out of range.
    pub fn convert_to_index(&self, value: i32) -> i32 {
        if value >= 0 && (value as usize) < self.index_to_value.len() {
            return self.index_to_value[value as usize];
        }
        -1
    }
}

/// Host model of the `XmlElement` int slot IDA 0x102ac writes: `clearValue`
/// on the pair (0x102bc..0x102c0), type tag `5` at `+16` (0x102c6), value at
/// `+20` (0x102c8); returns `5` (0x102ca).
#[derive(Default)]
pub struct XmlIntSlot {
    pub value_type: i32,
    pub int_value: i32,
}

/// Host model of the `XmlElement` read paths IDA 0x102cc discriminates:
/// xsi:nil early-out (0x102f0), int pair (0x10338..0x10348), string pair
/// (0x10356..0x103d6), otherwise the `ReleaseAssert(false)` at
/// 0x1040c..0x104aa (Reflection.h:359).
pub enum XmlReadValue {
    Nil,
    Int(i32),
    Text(String),
    Other,
}

// 0xf6f8d0 — sub_F6F8D0
#[doc(alias = "sub_F6F8D0")]
pub fn stub_f6f8d0() -> ! {
    todo!("0xf6f8d0 sub_F6F8D0")
}

// 0xf6f8dc — sub_F6F8DC
#[doc(alias = "sub_F6F8DC")]
pub fn stub_f6f8dc() -> ! {
    todo!("0xf6f8dc sub_F6F8DC")
}

// 0xf6f8e8 — sub_F6F8E8
#[doc(alias = "sub_F6F8E8")]
pub fn stub_f6f8e8() -> ! {
    todo!("0xf6f8e8 sub_F6F8E8")
}

// 0xf6f8f4 — sub_F6F8F4
#[doc(alias = "sub_F6F8F4")]
pub fn stub_f6f8f4() -> ! {
    todo!("0xf6f8f4 sub_F6F8F4")
}

// 0xf6f900 — sub_F6F900
#[doc(alias = "sub_F6F900")]
pub fn stub_f6f900() -> ! {
    todo!("0xf6f900 sub_F6F900")
}

// 0xf6f90c — sub_F6F90C
#[doc(alias = "sub_F6F90C")]
pub fn stub_f6f90c() -> ! {
    todo!("0xf6f90c sub_F6F90C")
}

// 0xf6f918 — sub_F6F918
#[doc(alias = "sub_F6F918")]
pub fn stub_f6f918() -> ! {
    todo!("0xf6f918 sub_F6F918")
}

// 0xf6f924 — sub_F6F924
#[doc(alias = "sub_F6F924")]
pub fn stub_f6f924() -> ! {
    todo!("0xf6f924 sub_F6F924")
}

// 0xf6f930 — sub_F6F930
#[doc(alias = "sub_F6F930")]
pub fn stub_f6f930() -> ! {
    todo!("0xf6f930 sub_F6F930")
}

// 0xf6f93c — sub_F6F93C
#[doc(alias = "sub_F6F93C")]
pub fn stub_f6f93c() -> ! {
    todo!("0xf6f93c sub_F6F93C")
}

// 0xf6f948 — sub_F6F948
#[doc(alias = "sub_F6F948")]
pub fn stub_f6f948() -> ! {
    todo!("0xf6f948 sub_F6F948")
}

// 0xf6f954 — sub_F6F954
#[doc(alias = "sub_F6F954")]
pub fn stub_f6f954() -> ! {
    todo!("0xf6f954 sub_F6F954")
}

// 0xf6f960 — sub_F6F960
#[doc(alias = "sub_F6F960")]
pub fn stub_f6f960() -> ! {
    todo!("0xf6f960 sub_F6F960")
}

// 0xf6f96c — sub_F6F96C
#[doc(alias = "sub_F6F96C")]
pub fn stub_f6f96c() -> ! {
    todo!("0xf6f96c sub_F6F96C")
}

// 0xf6f978 — sub_F6F978
#[doc(alias = "sub_F6F978")]
pub fn stub_f6f978() -> ! {
    todo!("0xf6f978 sub_F6F978")
}

// 0xf6f984 — sub_F6F984
#[doc(alias = "sub_F6F984")]
pub fn stub_f6f984() -> ! {
    todo!("0xf6f984 sub_F6F984")
}

// 0xf6f990 — sub_F6F990
#[doc(alias = "sub_F6F990")]
pub fn stub_f6f990() -> ! {
    todo!("0xf6f990 sub_F6F990")
}

// 0xf6f99c — sub_F6F99C
#[doc(alias = "sub_F6F99C")]
pub fn stub_f6f99c() -> ! {
    todo!("0xf6f99c sub_F6F99C")
}

// 0xf6f9a8 — sub_F6F9A8
#[doc(alias = "sub_F6F9A8")]
pub fn stub_f6f9a8() -> ! {
    todo!("0xf6f9a8 sub_F6F9A8")
}

// 0xf6f9b4 — sub_F6F9B4
#[doc(alias = "sub_F6F9B4")]
pub fn stub_f6f9b4() -> ! {
    todo!("0xf6f9b4 sub_F6F9B4")
}

// 0xf6f9c0 — sub_F6F9C0
#[doc(alias = "sub_F6F9C0")]
pub fn stub_f6f9c0() -> ! {
    todo!("0xf6f9c0 sub_F6F9C0")
}

// 0xf6f9cc — sub_F6F9CC
#[doc(alias = "sub_F6F9CC")]
pub fn stub_f6f9cc() -> ! {
    todo!("0xf6f9cc sub_F6F9CC")
}

// 0xf6f9d8 — sub_F6F9D8
#[doc(alias = "sub_F6F9D8")]
pub fn stub_f6f9d8() -> ! {
    todo!("0xf6f9d8 sub_F6F9D8")
}

// 0xf6f9e4 — sub_F6F9E4
#[doc(alias = "sub_F6F9E4")]
pub fn stub_f6f9e4() -> ! {
    todo!("0xf6f9e4 sub_F6F9E4")
}

// 0xf6f9f0 — sub_F6F9F0
#[doc(alias = "sub_F6F9F0")]
pub fn stub_f6f9f0() -> ! {
    todo!("0xf6f9f0 sub_F6F9F0")
}

// 0xf6f9fc — sub_F6F9FC
#[doc(alias = "sub_F6F9FC")]
pub fn stub_f6f9fc() -> ! {
    todo!("0xf6f9fc sub_F6F9FC")
}

// 0xf6fa08 — sub_F6FA08
#[doc(alias = "sub_F6FA08")]
pub fn stub_f6fa08() -> ! {
    todo!("0xf6fa08 sub_F6FA08")
}

// 0xf6fa14 — sub_F6FA14
#[doc(alias = "sub_F6FA14")]
pub fn stub_f6fa14() -> ! {
    todo!("0xf6fa14 sub_F6FA14")
}

// 0xf6fa20 — sub_F6FA20
#[doc(alias = "sub_F6FA20")]
pub fn stub_f6fa20() -> ! {
    todo!("0xf6fa20 sub_F6FA20")
}

// 0xf6fa2c — sub_F6FA2C
#[doc(alias = "sub_F6FA2C")]
pub fn stub_f6fa2c() -> ! {
    todo!("0xf6fa2c sub_F6FA2C")
}

// 0xf6fa38 — sub_F6FA38
#[doc(alias = "sub_F6FA38")]
pub fn stub_f6fa38() -> ! {
    todo!("0xf6fa38 sub_F6FA38")
}

// 0xf6fa44 — sub_F6FA44
#[doc(alias = "sub_F6FA44")]
pub fn stub_f6fa44() -> ! {
    todo!("0xf6fa44 sub_F6FA44")
}

// 0xf6fa50 — sub_F6FA50
#[doc(alias = "sub_F6FA50")]
pub fn stub_f6fa50() -> ! {
    todo!("0xf6fa50 sub_F6FA50")
}

// 0xf6fa5c — sub_F6FA5C
#[doc(alias = "sub_F6FA5C")]
pub fn stub_f6fa5c() -> ! {
    todo!("0xf6fa5c sub_F6FA5C")
}

// 0xf6fa68 — sub_F6FA68
#[doc(alias = "sub_F6FA68")]
pub fn stub_f6fa68() -> ! {
    todo!("0xf6fa68 sub_F6FA68")
}

// 0xf6fa74 — sub_F6FA74
#[doc(alias = "sub_F6FA74")]
pub fn stub_f6fa74() -> ! {
    todo!("0xf6fa74 sub_F6FA74")
}

// 0xf6fa80 — sub_F6FA80
#[doc(alias = "sub_F6FA80")]
pub fn stub_f6fa80() -> ! {
    todo!("0xf6fa80 sub_F6FA80")
}

// 0xf6fa8c — sub_F6FA8C
#[doc(alias = "sub_F6FA8C")]
pub fn stub_f6fa8c() -> ! {
    todo!("0xf6fa8c sub_F6FA8C")
}

// 0xf6fa98 — sub_F6FA98
#[doc(alias = "sub_F6FA98")]
pub fn stub_f6fa98() -> ! {
    todo!("0xf6fa98 sub_F6FA98")
}

// 0xf6faa4 — sub_F6FAA4
#[doc(alias = "sub_F6FAA4")]
pub fn stub_f6faa4() -> ! {
    todo!("0xf6faa4 sub_F6FAA4")
}

// 0xf6fab0 — sub_F6FAB0
#[doc(alias = "sub_F6FAB0")]
pub fn stub_f6fab0() -> ! {
    todo!("0xf6fab0 sub_F6FAB0")
}

// 0xf6fabc — sub_F6FABC
#[doc(alias = "sub_F6FABC")]
pub fn stub_f6fabc() -> ! {
    todo!("0xf6fabc sub_F6FABC")
}

// 0xf6fac8 — sub_F6FAC8
#[doc(alias = "sub_F6FAC8")]
pub fn stub_f6fac8() -> ! {
    todo!("0xf6fac8 sub_F6FAC8")
}

// 0xf6fad4 — sub_F6FAD4
#[doc(alias = "sub_F6FAD4")]
pub fn stub_f6fad4() -> ! {
    todo!("0xf6fad4 sub_F6FAD4")
}

// 0xf6fae0 — sub_F6FAE0
#[doc(alias = "sub_F6FAE0")]
pub fn stub_f6fae0() -> ! {
    todo!("0xf6fae0 sub_F6FAE0")
}

// 0xf6faec — sub_F6FAEC
#[doc(alias = "sub_F6FAEC")]
pub fn stub_f6faec() -> ! {
    todo!("0xf6faec sub_F6FAEC")
}

// 0xf6faf8 — sub_F6FAF8
#[doc(alias = "sub_F6FAF8")]
pub fn stub_f6faf8() -> ! {
    todo!("0xf6faf8 sub_F6FAF8")
}

// 0xf6fb04 — sub_F6FB04
#[doc(alias = "sub_F6FB04")]
pub fn stub_f6fb04() -> ! {
    todo!("0xf6fb04 sub_F6FB04")
}

// 0xf6fb10 — sub_F6FB10
#[doc(alias = "sub_F6FB10")]
pub fn stub_f6fb10() -> ! {
    todo!("0xf6fb10 sub_F6FB10")
}

// 0xf6fb1c — sub_F6FB1C
#[doc(alias = "sub_F6FB1C")]
pub fn stub_f6fb1c() -> ! {
    todo!("0xf6fb1c sub_F6FB1C")
}

// 0xf6fb28 — sub_F6FB28
#[doc(alias = "sub_F6FB28")]
pub fn stub_f6fb28() -> ! {
    todo!("0xf6fb28 sub_F6FB28")
}

// 0xf6fb34 — sub_F6FB34
#[doc(alias = "sub_F6FB34")]
pub fn stub_f6fb34() -> ! {
    todo!("0xf6fb34 sub_F6FB34")
}

// 0xf6fb40 — sub_F6FB40
#[doc(alias = "sub_F6FB40")]
pub fn stub_f6fb40() -> ! {
    todo!("0xf6fb40 sub_F6FB40")
}

// 0xf6fb4c — sub_F6FB4C
// type: int()
#[doc(alias = "sub_F6FB4C")]
pub fn stub_f6fb4c() -> ! {
    todo!("0xf6fb4c sub_F6FB4C")
}

// 0xf83c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_f83c() {
    // IDA 0xf83c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf87c — __ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_f87c() {
    // IDA 0xf87c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf8c8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_f8c8() {
    // IDA 0xf8c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf90c — __ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_f90c() {
    // IDA 0xf90c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf964 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED1Ev")]
pub fn stub_f964() {
    // IDA 0xf964: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf9a8 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEED0Ev")]
pub fn stub_f9a8() {
    // IDA 0xf9a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xfa00 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_fa00() -> &'static RenderSettingsClassDescriptor {
    // IDA 0xfa00 (decompiled 0xfa00..0xfaee; disasm guard acquire 0xfa5c,
    // base Instance descriptor 0xfa68, ClassDescriptor(local, &base,
    // "RenderSettings") 0xfaa0, __cxa_atexit 0xfabe, return &local 0xfaee):
    // function-local static once-init. A Rust `static` with a const
    // initializer is already once-initialized, so the guard/`atexit` have
    // no host effect.
    static DESC: RenderSettingsClassDescriptor = RenderSettingsClassDescriptor {
        name: "RenderSettings",
        parent: "Instance",
    };
    &DESC
}

// 0xfb1c — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_fb1c() {
    // IDA 0xfb1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xfb20 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_fb20() {
    // IDA 0xfb20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xfb34 — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_fb34() {
    // IDA 0xfb34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xfb3c — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_fb3c() {
    // IDA 0xfb3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xfb54 — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_fb54() {
    // IDA 0xfb54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xfb5c — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_fb5c() {
    // IDA 0xfb5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xfb74 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_fb74(
    name: &str,
    category: &str,
    getter: fn(&CacheSettings) -> u32,
    setter: fn(&mut CacheItem, u32),
    attributes: u32,
    permissions: u32,
) -> CacheSizeProp {
    // IDA 0xfb74 (decompiled 0xfb74..0xfc56; disasm classDescriptor 0xfb9c,
    // GetSetImpl alloc + vtable/getter/setter stores 0xfba2..0xfbd8,
    // TypedPropertyDescriptor<int> ctor 0xfc1a, vtable install 0xfc38):
    // register the get/set pair against the RenderSettings class descriptor.
    let _ = stub_fa00();
    CacheSizeProp {
        name: name.to_owned(),
        category: category.to_owned(),
        getter: Some(getter),
        setter: Some(setter),
        attributes,
        permissions,
    }
}

// 0xfc88 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
pub fn stub_fc88() {
    // IDA 0xfc88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xfcb4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const")]
pub fn stub_fcb4(prop: &CacheSizeProp) -> bool {
    // IDA 0xfcb4 (disasm 0xfcb4..0xfcb6 `MOVS R0,#0; BX LR`): the getter
    // member pointer is bound at 0xfb74, so never read-only.
    prop.is_read_only()
}

// 0xfcb8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")]
pub fn stub_fcb8(prop: &CacheSizeProp) -> bool {
    // IDA 0xfcb8 (disasm 0xfcb8..0xfcba `MOVS R0,#0; BX LR`): the setter
    // member pointer is bound at 0xfb74, so never write-only.
    prop.is_write_only()
}

// 0xfcbc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_fcbc(prop: &CacheSizeProp, settings: &CacheSettings) -> i32 {
    // IDA 0xfcbc (decompiled 0xfcbc..0xfce6; disasm member-ptr adjust
    // 0xfcc0..0xfcd8, indirect getter call 0xfce6): resolves the stored
    // `unsigned int (CRenderSettings::*)() const` against the described
    // object and returns its value. The ARM member-pointer adjust/thunk
    // dance has no host effect; the getter call is the whole body. A null
    // getter faults in the image; the host panics.
    let get = prop.getter.expect("bound getter at IDA 0xfb74");
    get(settings) as i32
}

// 0xfce8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_fce8(prop: &CacheSizeProp, item: &mut CacheItem, value: i32) {
    // IDA 0xfce8 (decompiled 0xfce8..0xfd08; disasm member-ptr adjust
    // 0xfcf0..0xfcfc, indirect setter call 0xfd06..0xfd08): resolves the
    // stored `void (CRenderSettingsItem::*)(unsigned int)` against the
    // described object (`a2-36` adjust) and invokes it with the new value.
    // A null setter faults in the image; the host panics.
    let set = prop.setter.expect("bound setter at IDA 0xfb74");
    set(item, value as u32);
}

// 0xfd0c — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_fd0c(
    name: &str,
    func: fn(&CacheItem) -> i32,
    permissions: u32,
    attributes: u32,
) -> BoundFunc0 {
    // IDA 0xfd0c (decompiled 0xfd0c..0xfdc2; disasm classDescriptor 0xfd32,
    // FunctionDescriptor base 0xfd52, vtable 0xfd6e, member-fn pair at +40
    // 0xfd7a, `Type::getSingleton<int>()` return type at +0x1C 0xfda2).
    let _ = stub_fa00();
    BoundFunc0 {
        name: name.to_owned(),
        permissions,
        attributes,
        func,
        return_type: "int",
    }
}

// 0xfe04 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
pub fn stub_fe04() {
    // IDA 0xfe04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xfe30 — __ZNK3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_fe30(desc: &BoundFunc0, item: &CacheItem) -> IntCallResult {
    // IDA 0xfe30 (decompiled 0xfe30..0xfe50; disasm obj adjust `a2-36`
    // 0xfe38..0xfe3a, Call0Helper::call tail-call 0xfe3e..0xfe50 with
    // (obj, [a1+40] mf, [a1+44], args+4)): the whole body is the helper call.
    stub_fe54(item, desc.func)
}

// 0xfe54 — __ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE
// type: int __fastcall(int, int (__fastcall *)(_DWORD), int, _DWORD *)
#[doc(alias = "RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_fe54(item: &CacheItem, func: fn(&CacheItem) -> i32) -> IntCallResult {
    // IDA 0xfe54 (decompiled 0xfe54..0xfe80; disasm member-ptr adjust
    // 0xfe5a..0xfe68, call 0xfe6a..0xfe6c): invoke the 0-arg member fn, tag
    // the out slot with `Type::getSingleton<int>()` (0xfe6e..0xfe72), then
    // `placement_any<int>` the value (0xfe7a..0xfe80).
    IntCallResult {
        type_name: "int",
        value: func(item),
    }
}

// 0xfe84 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_fe84(
    name: &str,
    category: &str,
    getter: fn(&CacheSettings) -> i32,
    setter: fn(&mut CacheItem, i32),
    attributes: u32,
    permissions: u32,
) -> ResolutionProp {
    // IDA 0xfe84 (decompiled 0xfe84..0xfffa; disasm enum singleton
    // 0xfec8..0xfecc, PropertyDescriptor base 0xff16, enum desc at +40
    // 0xff3a, GetSetImpl alloc + getter/setter stores at +44 0xff62..0xff88,
    // attribute flag fixups at +28 0xffb4..0xffda).
    // was: boost::call_once -> one-time init (host: EnumDesc singleton
    // already constructed; no host effect).
    let _ = stub_fa00();
    ResolutionProp {
        name: name.to_owned(),
        category: category.to_owned(),
        getter: Some(getter),
        setter: Some(setter),
        attributes,
        permissions,
        enum_type: "ResolutionPreset",
    }
}

// 0x10038 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
pub fn stub_10038() {
    // IDA 0x10038: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x10064 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const")]
pub fn stub_10064(prop: &ResolutionProp) -> bool {
    // IDA 0x10064 (decompiled; disasm 0x10064..0x10070: load impl at
    // `[a1+0x2C]`, tail-call its slot-0 virtual): delegates to the +44
    // GetSetImpl's isReadOnly. Same answer as 0xfcb4.
    prop.is_read_only()
}

// 0x10074 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const")]
pub fn stub_10074(prop: &ResolutionProp) -> bool {
    // IDA 0x10074 (decompiled; disasm 0x10074..0x10080: load impl at
    // `[a1+0x2C]`, tail-call its slot-1 virtual): delegates to the +44
    // GetSetImpl's isWriteOnly. Same answer as 0xfcb8.
    prop.is_write_only()
}

// 0x10084 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_10084(prop: &ResolutionProp, a: &CacheSettings, b: &CacheSettings) -> bool {
    // IDA 0x10084 (decompiled 0x10084..0x100aa; disasm getValue slot-2 calls
    // 0x1008a..0x10094 and 0x10096..0x100aa): compares the +44 GetSetImpl
    // `getValue` of both described objects.
    let get = prop.getter.expect("bound getter at IDA 0xfe84");
    get(a) == get(b)
}

// 0x100ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_100ac(prop: &ResolutionProp, settings: &CacheSettings) -> IntCallResult {
    // IDA 0x100ac (decompiled 0x100ac..0x100ce; disasm getEnumValue slot-0x44
    // call 0x100b4..0x100ba, `Type::getSingleton<int>()` 0x100bc..0x100c0,
    // `placement_any<int>` 0x100c0..0x100ce): tags the out slot with the int
    // singleton, then stores the enum value as int — same shape as 0xfe54.
    let get = prop.getter.expect("bound getter at IDA 0xfe84");
    IntCallResult {
        type_name: "int",
        value: get(settings),
    }
}

// 0x100d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_100d0(prop: &ResolutionProp, item: &mut CacheItem, value: i32) {
    // IDA 0x100d0 (decompiled 0x100d0..0x10204; disasm typeinfo-for-int fast
    // path 0x1011a..0x101cc, `Variant::convert<int>` slow path
    // 0x10150..0x1018e, `setIntValue` slot-72 call 0x101da..0x10204):
    // coerces the variant to int, then stores it. The variant-boxing dance
    // has no host effect; the caller passes the coerced int. The image
    // returns the stack guard (void).
    let set = prop.setter.expect("bound setter at IDA 0xfe84");
    set(item, value);
}

// 0x10220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_10220(prop: &ResolutionProp, src: &CacheSettings, dst: &mut CacheItem) {
    // IDA 0x10220 (decompiled 0x10220..0x10242; disasm getValue slot-8
    // 0x1022a..0x10232, setValue slot-12 0x10236..0x10242): reads the source
    // through the +44 GetSetImpl getter, writes it through the setter.
    let get = prop.getter.expect("bound getter at IDA 0xfe84");
    let set = prop.setter.expect("bound setter at IDA 0xfe84");
    set(dst, get(src));
}

// 0x10244 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::hasStringValue(void)const")]
pub fn stub_10244() -> bool {
    // IDA 0x10244 (disasm 0x10244..0x10246 `MOVS R0,#1; BX LR`): enum
    // properties always have a string value.
    true
}

// 0x10248 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_10248(
    prop: &ResolutionProp,
    desc: &ResolutionEnumDesc,
    settings: &CacheSettings,
    out: &mut String,
) -> bool {
    // IDA 0x10248 (decompiled 0x10248..0x1026a; disasm getValue slot-8
    // 0x10250..0x1025a, `EnumDesc::convertToString` 0x1025c..0x1026a):
    // renders the current enum value through the +48 enum singleton.
    let get = prop.getter.expect("bound getter at IDA 0xfe84");
    desc.value_to_string(get(settings), out)
}

// 0x1026c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_1026c(
    prop: &ResolutionProp,
    desc: &ResolutionEnumDesc,
    item: &mut CacheItem,
    name: &str,
) -> bool {
    // IDA 0x1026c (decompiled 0x1026c..0x102a8; disasm `Name::lookup`
    // 0x1027a..0x1027e, `EnumDesc::convertToValue` 0x1027e..0x1028c, setValue
    // slot-12 0x10298..0x102a2, return 1/0 at 0x102a4/0x102a8).
    // `Name::lookup` interning has no host effect; the host compares names
    // directly.
    match desc.lookup_value(name) {
        Some(value) => {
            let set = prop.setter.expect("bound setter at IDA 0xfe84");
            set(item, value);
            true
        }
        None => false,
    }
}

// 0x102ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_102ac(prop: &ResolutionProp, settings: &CacheSettings, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x102ac (decompiled 0x102ac..0x102ca; disasm getValue slot-8
    // 0x102ae..0x102ba, `clearValue` 0x102bc..0x102c0, tag `5` at +16
    // 0x102c6, value at +20 0x102c8, return 5 at 0x102ca).
    let get = prop.getter.expect("bound getter at IDA 0xfe84");
    out.value_type = 0; // `clearValue` resets the pair first (0x102c0).
    out.value_type = 5; // int tag at +16 (0x102c6).
    out.int_value = get(settings); // value at +20 (0x102c8).
    5
}

// 0x102cc — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_102cc(
    prop: &ResolutionProp,
    desc: &ResolutionEnumDesc,
    item: &mut CacheItem,
    xml: &XmlReadValue,
) {
    // IDA 0x102cc (decompiled 0x102cc..0x104aa): xsi:nil early-out (0x102f0);
    // int pair → `setIntValue` (0x10338..0x10348); string pair →
    // `Name::lookup` + `convertToValue` + setValue (0x10356..0x103b2) with
    // the empty-string `validate` fallback (0x103d4..0x10486); anything else
    // falls into `ReleaseAssert(false)` (0x1040c..0x104aa,
    // Reflection.h:359), which faults in the image — the host panics with
    // the same message. The trailing `FLog::Asserts` gate (0x103c4..0x1040c)
    // has no host effect; the host takes the asserts-on path.
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if stub_10674(prop, desc, item, *value) {
                return;
            }
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
        XmlReadValue::Text(text) => {
            if let Some(value) = desc.lookup_value(text) {
                let set = prop.setter.expect("bound setter at IDA 0xfe84");
                set(item, value);
                return;
            }
            if text.is_empty() {
                // Empty string → `validate` virtual (slot-64); the host has
                // no validators, so this is a no-op.
                return;
            }
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
        XmlReadValue::Other => {
            panic!("false file: ../App/include/Reflection/Reflection.h line: 359");
        }
    }
}

// 0x1050c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1050c(prop: &ResolutionProp, desc: &ResolutionEnumDesc, settings: &CacheSettings) -> i32 {
    // IDA 0x1050c (decompiled 0x1050c..0x10526; disasm getValue slot-8
    // 0x1050e..0x1051c, `convertToIndex` tail-call 0x1051e..0x10524).
    let get = prop.getter.expect("bound getter at IDA 0xfe84");
    desc.convert_to_index(get(settings))
}

// 0x10528 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_10528(
    prop: &ResolutionProp,
    desc: &ResolutionEnumDesc,
    item: &mut CacheItem,
    index: u32,
) -> bool {
    // IDA 0x10528 (decompiled 0x10528..0x10558; disasm count check
    // 0x1052e..0x1053a, table load `[[desc+144] + 4*index]` 0x1053c..0x10544,
    // setValue slot-12 0x10544..0x1054e, return 1/0 at 0x10550/0x10558).
    // Unlike 0x10674, a `-1` hole is stored as-is.
    if (index as usize) < desc.index_to_value.len() {
        let value = desc.index_to_value[index as usize];
        let set = prop.setter.expect("bound setter at IDA 0xfe84");
        set(item, value);
        return true;
    }
    false
}

// 0x1055c — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_1055c(prop: &ResolutionProp, settings: &CacheSettings) -> i32 {
    // IDA 0x1055c (disasm 0x1055c..0x10562: load impl at `[a1+0x2C]`,
    // tail-call its slot-2/getValue virtual): the whole body is the getter.
    let get = prop.getter.expect("bound getter at IDA 0xfe84");
    get(settings)
}

// 0x10564 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_10564(
    prop: &ResolutionProp,
    desc: &ResolutionEnumDesc,
    item: &mut CacheItem,
    value: i32,
) -> bool {
    // IDA 0x10564 (decompiled 0x10564..0x105ac; disasm `__find_if` with
    // `EnumDescriptor::equalValue` over `[desc+28, desc+32)` 0x10572..0x1058e,
    // setValue slot-12 0x10596..0x105a2, return 1/0 at 0x105a4/0x105ac).
    // was: boost::bind + __find_if → iterator search (no boost crate per §4).
    if desc.items.iter().any(|(v, _)| *v == value) {
        let set = prop.setter.expect("bound setter at IDA 0xfe84");
        set(item, value);
        return true;
    }
    false
}

// 0x105b0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_105b0(prop: &ResolutionProp, desc: &ResolutionEnumDesc, settings: &CacheSettings) -> i32 {
    // IDA 0x105b0 (decompiled 0x105b0..0x105ce; disasm getValue slot-8
    // 0x105b6..0x105c2, `convertToItem` 0x105c4..0x105ce): resolves the
    // current value to its descriptor item. Item pointers have no host
    // meaning; the host returns the item's position, or -1 when unmapped.
    let get = prop.getter.expect("bound getter at IDA 0xfe84");
    let value = get(settings);
    desc.items
        .iter()
        .position(|(v, _)| *v == value)
        .map(|i| i as i32)
        .unwrap_or(-1)
}

// 0x105d0 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_105d0(
    prop: &ResolutionProp,
    desc: &ResolutionEnumDesc,
    item: &mut CacheItem,
    name: &str,
) -> bool {
    // IDA 0x105d0 (decompiled 0x105d0..0x10602; disasm `convertToValue`
    // 0x105da..0x105e6, setValue slot-12 0x105f2..0x105fc, return 1/0 at
    // 0x105fe/0x10602): the `Name` overload twin of 0x1026c.
    match desc.lookup_value(name) {
        Some(value) => {
            let set = prop.setter.expect("bound setter at IDA 0xfe84");
            set(item, value);
            true
        }
        None => false,
    }
}

// 0x10604 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToIndex(RBX::CRenderSettings::ResolutionPreset)const")]
pub fn stub_10604(desc: &ResolutionEnumDesc, value: i32) -> i32 {
    // IDA 0x10604 (decompiled 0x10604..0x10672; disasm `value>=0` assert chain
    // 0x10618..0x1065c, index-table load `[[a1+156] + 4*value]`
    // 0x1065e..0x1066e, default -1 at 0x10666..0x10672): the
    // `FLog::Asserts` / `_debugHook` / `ReleaseAssert` gate has no host
    // effect beyond the assert; the host takes the asserts-on path via
    // `debug_assert`.
    debug_assert!(value >= 0, "value>=0 file: ../App/include/reflection/enumconverter.h line: 350");
    desc.convert_to_index(value)
}

// 0x10674 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_10674(
    prop: &ResolutionProp,
    desc: &ResolutionEnumDesc,
    item: &mut CacheItem,
    value: i32,
) -> bool {
    // IDA 0x10674 (decompiled 0x10674..0x106b0; disasm `value>=0` at 0x10678,
    // count check 0x10682..0x10690, table load 0x10692, `-1` hole check at
    // 0x1069c, setValue slot-12 0x1069e..0x106a8, return 1/0 at
    // 0x106aa/0x106b0): the `-1` check is what 0x10528 lacks.
    if value >= 0 && (value as usize) < desc.index_to_value.len() {
        let mapped = desc.index_to_value[value as usize];
        if mapped != -1 {
            let set = prop.setter.expect("bound setter at IDA 0xfe84");
            set(item, mapped);
            return true;
        }
    }
    false
}

// 0x106b4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isReadOnly(void)const")]
pub fn stub_106b4(prop: &ResolutionProp) -> bool {
    // IDA 0x106b4 (disasm 0x106b4..0x106b6 `MOVS R0,#0; BX LR`): the
    // ResolutionPreset member pointer is bound at 0xfe84, so never read-only.
    prop.is_read_only()
}

// 0x106b8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::GetSetImpl<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>::isWriteOnly(void)const")]
pub fn stub_106b8(prop: &ResolutionProp) -> bool {
    // IDA 0x106b8 (disasm 0x106b8..0x106ba `MOVS R0,#0; BX LR`): the
    // ResolutionPreset member pointer is bound at 0xfe84, so never write-only.
    prop.is_write_only()
}

#[cfg(test)]
mod batch3_tests {
    use super::*;

    fn tex_get(settings: &CacheSettings) -> u32 {
        settings.texture_cache_size
    }

    fn tex_set(item: &mut CacheItem, value: u32) {
        item.texture_cache_size = value;
    }

    fn res_get(settings: &CacheSettings) -> i32 {
        settings.resolution_preference
    }

    fn res_set(item: &mut CacheItem, value: i32) {
        item.resolution_preset = value;
    }

    fn cache_total(item: &CacheItem) -> i32 {
        item.texture_cache_size as i32 + item.mesh_cache_size as i32
    }

    #[test]
    fn class_descriptor_singleton() {
        // IDA 0xfa00: once-init ClassDescriptor(local, &base, "RenderSettings").
        let desc = stub_fa00();
        assert_eq!(desc.name, "RenderSettings");
        assert_eq!(desc.parent, "Instance");
        assert!(std::ptr::eq(desc, stub_fa00()));
    }

    #[test]
    fn int_prop_getset_roundtrip() {
        // IDA 0xfb74 ctor + 0xfcb4/0xfcb8/0xfcbc/0xfce8 virtuals: the
        // texture-cache-size get/set pair.
        let prop = stub_fb74("TextureCacheSize", "Rendering", tex_get, tex_set, 0, 0);
        assert_eq!(prop.name, "TextureCacheSize");
        assert_eq!(prop.category, "Rendering");
        assert!(!stub_fcb4(&prop));
        assert!(!stub_fcb8(&prop));
        let settings = CacheSettings {
            texture_cache_size: 512,
            ..CacheSettings::default()
        };
        assert_eq!(stub_fcbc(&prop, &settings), 512);
        let mut item = CacheItem::default();
        stub_fce8(&prop, &mut item, 256);
        assert_eq!(item.texture_cache_size, 256);
    }

    #[test]
    fn bound_func_execute_tags_int_result() {
        // IDA 0xfd0c ctor + 0xfe30 execute + 0xfe54 helper: invoke the 0-arg
        // member fn, tag the out slot with the int singleton, store the value.
        let desc = stub_fd0c("GetCacheTotal", cache_total, 0, 0);
        assert_eq!(desc.name, "GetCacheTotal");
        assert_eq!(desc.return_type, "int");
        let item = CacheItem {
            texture_cache_size: 512,
            mesh_cache_size: 256,
            ..CacheItem::default()
        };
        let out = stub_fe30(&desc, &item);
        assert_eq!(out.type_name, "int");
        assert_eq!(out.value, 768);
        let direct = stub_fe54(&item, cache_total);
        assert_eq!(direct.value, 768);
    }

    #[test]
    fn enum_prop_delegates_to_getset() {
        // IDA 0xfe84 ctor + 0x10064/0x10074 delegating virtuals: the
        // resolution-preset enum property is neither read- nor write-only.
        let prop = stub_fe84("Resolution", "Rendering", res_get, res_set, 0, 0);
        assert_eq!(prop.enum_type, "ResolutionPreset");
        assert!(!stub_10064(&prop));
        assert!(!stub_10074(&prop));
        let settings = CacheSettings {
            resolution_preference: 4,
            ..CacheSettings::default()
        };
        assert_eq!((prop.getter.expect("bound"))(&settings), 4);
        let mut item = CacheItem::default();
        (prop.setter.expect("bound"))(&mut item, 4);
        assert_eq!(item.resolution_preset, 4);
    }
}

#[cfg(test)]
mod batch4_tests {
    use super::*;

    fn res_get(settings: &CacheSettings) -> i32 {
        settings.resolution_preference
    }

    fn res_set(item: &mut CacheItem, value: i32) {
        item.resolution_preset = value;
    }

    fn prop() -> ResolutionProp {
        ResolutionProp {
            name: "Resolution".to_owned(),
            category: "Rendering".to_owned(),
            getter: Some(res_get),
            setter: Some(res_set),
            attributes: 0,
            permissions: 0,
            enum_type: "ResolutionPreset",
        }
    }

    fn desc() -> ResolutionEnumDesc {
        // Index 2..=3 left as `-1` holes, as in the image legacy table.
        let mut d = ResolutionEnumDesc::default();
        d.add_pair(0, "Automatic", 0);
        d.add_pair(1, "Low", 1);
        d.add_pair(4, "Ultra", 4);
        d
    }

    fn settings_with(value: i32) -> CacheSettings {
        CacheSettings {
            resolution_preference: value,
            ..CacheSettings::default()
        }
    }

    #[test]
    fn equal_values_compares_both_getters() {
        // IDA 0x10084: getValue(a2) == getValue(a3).
        let p = prop();
        assert!(stub_10084(&p, &settings_with(1), &settings_with(1)));
        assert!(!stub_10084(&p, &settings_with(1), &settings_with(4)));
    }

    #[test]
    fn variant_roundtrip_through_int_singleton() {
        // IDA 0x100ac getVariant + 0x100d0 setVariant + 0x10220 copyValue.
        let p = prop();
        let out = stub_100ac(&p, &settings_with(4));
        assert_eq!(out.type_name, "int");
        assert_eq!(out.value, 4);
        let mut item = CacheItem::default();
        stub_100d0(&p, &mut item, out.value);
        assert_eq!(item.resolution_preset, 4);
        let mut dst = CacheItem::default();
        stub_10220(&p, &settings_with(1), &mut dst);
        assert_eq!(dst.resolution_preset, 1);
    }

    #[test]
    fn string_value_roundtrip() {
        // IDA 0x10244 hardcoded 1; 0x10248 getStringValue; 0x1026c + 0x105d0
        // setStringValue overloads.
        assert!(stub_10244());
        let p = prop();
        let d = desc();
        let mut name = String::new();
        assert!(stub_10248(&p, &d, &settings_with(1), &mut name));
        assert_eq!(name, "Low");
        assert!(!stub_10248(&p, &d, &settings_with(7), &mut name));
        let mut item = CacheItem::default();
        assert!(stub_1026c(&p, &d, &mut item, "Ultra"));
        assert_eq!(item.resolution_preset, 4);
        assert!(!stub_1026c(&p, &d, &mut item, "Bogus"));
        assert!(stub_105d0(&p, &d, &mut item, "Low"));
        assert_eq!(item.resolution_preset, 1);
        assert!(!stub_105d0(&p, &d, &mut item, "Bogus"));
    }

    #[test]
    fn write_value_tags_int_slot() {
        // IDA 0x102ac: clearValue, tag 5 at +16, value at +20, return 5.
        let p = prop();
        let mut slot = XmlIntSlot::default();
        assert_eq!(stub_102ac(&p, &settings_with(4), &mut slot), 5);
        assert_eq!(slot.value_type, 5);
        assert_eq!(slot.int_value, 4);
    }

    #[test]
    fn read_value_dispatch() {
        // IDA 0x102cc: nil no-op, int via setIntValue, text via lookup.
        let p = prop();
        let d = desc();
        let mut item = CacheItem::default();
        stub_102cc(&p, &d, &mut item, &XmlReadValue::Nil);
        assert_eq!(item.resolution_preset, 0);
        stub_102cc(&p, &d, &mut item, &XmlReadValue::Int(1));
        assert_eq!(item.resolution_preset, 1);
        stub_102cc(&p, &d, &mut item, &XmlReadValue::Text("Ultra".to_owned()));
        assert_eq!(item.resolution_preset, 4);
        stub_102cc(&p, &d, &mut item, &XmlReadValue::Text(String::new()));
        assert_eq!(item.resolution_preset, 4);
    }

    #[test]
    #[should_panic(expected = "Reflection.h line: 359")]
    fn read_value_other_hits_release_assert() {
        // IDA 0x102cc trailing `ReleaseAssert(false)` path.
        let p = prop();
        let d = desc();
        let mut item = CacheItem::default();
        stub_102cc(&p, &d, &mut item, &XmlReadValue::Other);
    }

    #[test]
    fn index_and_enum_value_paths() {
        // IDA 0x1050c getIndexValue, 0x10528 setIndexValue (stores `-1`
        // holes as-is), 0x1055c getEnumValue, 0x10564 setEnumValue,
        // 0x105b0 getEnumItem, 0x10604 convertToIndex, 0x10674 setIntValue
        // (rejects holes), 0x106b4/0x106b8 bound GetSetImpl virtuals.
        let p = prop();
        let d = desc();
        assert_eq!(stub_1050c(&p, &d, &settings_with(4)), 4);
        assert_eq!(stub_1050c(&p, &d, &settings_with(7)), -1);
        let mut item = CacheItem::default();
        assert!(stub_10528(&p, &d, &mut item, 1));
        assert_eq!(item.resolution_preset, 1);
        assert!(stub_10528(&p, &d, &mut item, 2));
        assert_eq!(item.resolution_preset, -1);
        assert!(!stub_10528(&p, &d, &mut item, 9));
        assert_eq!(stub_1055c(&p, &settings_with(4)), 4);
        assert!(stub_10564(&p, &d, &mut item, 4));
        assert_eq!(item.resolution_preset, 4);
        assert!(!stub_10564(&p, &d, &mut item, 7));
        assert_eq!(stub_105b0(&p, &d, &settings_with(1)), 1);
        assert_eq!(stub_105b0(&p, &d, &settings_with(7)), -1);
        assert_eq!(stub_10604(&d, 4), 4);
        assert_eq!(stub_10604(&d, 9), -1);
        assert!(stub_10674(&p, &d, &mut item, 1));
        assert_eq!(item.resolution_preset, 1);
        assert!(!stub_10674(&p, &d, &mut item, 2));
        assert!(!stub_10674(&p, &d, &mut item, 9));
        assert!(!stub_106b4(&p));
        assert!(!stub_106b8(&p));
    }
}
