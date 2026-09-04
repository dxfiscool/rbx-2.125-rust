//! audio generated_137 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Soundscape exhausted (2398 distinct) — filler EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x14cf4..0x18c98 EA-sorted asc filler after 0x14ccc, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_136::NamePresetMap;

/// Host model of `RBX::Reflection::EnumDesc<T>` for the CRenderSettings enum
/// singletons below (IDA 0x16548..0x16b04): `{items, index_to_value}` table
/// plus the enum type name passed to the C2 ctor (cf. rbx-reflection
/// `enum_desc.rs`: "Shadow", "Resolution", "QualityLevel", "Antialiasing",
/// "FramerateManagerMode", "GraphicsMode", "AASamples").
/// Audio owns no rbx-reflection dependency (AGENTS.md DAG), so the carrier
/// lives here; population runs through `add_pair` from the descriptor init
/// code, mirroring `QualityEnumDesc` in generated_136.rs.
pub struct RenderEnumDesc {
    pub name: &'static str,
    pub items: Vec<(i32, String)>,
    pub index_to_value: Vec<i32>,
}

impl RenderEnumDesc {
    pub fn new(name: &'static str) -> Self {
        RenderEnumDesc {
            name,
            items: Vec::new(),
            index_to_value: Vec::new(),
        }
    }

    pub fn add_pair(&mut self, value: i32, name: &str, index: usize) {
        self.items.push((value, name.to_owned()));
        if self.index_to_value.len() <= index {
            self.index_to_value.resize(index + 1, -1);
        }
        self.index_to_value[index] = value;
    }

    pub fn lookup_value(&self, name: &str) -> Option<i32> {
        self.items.iter().find(|(_, n)| n == name).map(|(v, _)| *v)
    }

    pub fn value_to_string(&self, value: i32, out: &mut String) -> bool {
        if let Some((_, name)) = self.items.iter().find(|(v, _)| *v == value) {
            *out = name.clone();
            true
        } else {
            false
        }
    }

    pub fn convert_to_index(&self, value: i32) -> i32 {
        if value >= 0 && (value as usize) < self.index_to_value.len() {
            return self.index_to_value[value as usize];
        }
        -1
    }
}

/// IDA 0x8c4c (`EnumDesc<ShadowMode>::EnumDesc`, string "Shadow" cf.
/// rbx-reflection `enum_desc_shadow_mode_ctor`).
pub fn render_enum_desc_shadow_mode_ctor() -> RenderEnumDesc {
    RenderEnumDesc::new("Shadow")
}

/// IDA 0x9100 (`EnumDesc<ResolutionPreset>::EnumDesc`, string "Resolution"
/// at 0x10c4cb0).
pub fn render_enum_desc_resolution_preset_ctor() -> RenderEnumDesc {
    RenderEnumDesc::new("Resolution")
}

/// IDA 0x8e24 (`EnumDesc<QualityLevel>::EnumDesc`, string "QualityLevel" at
/// 0x10c4c8c).
pub fn render_enum_desc_quality_level_ctor() -> RenderEnumDesc {
    RenderEnumDesc::new("QualityLevel")
}

/// IDA 0x8a88 (`EnumDesc<AntialiasingMode>::EnumDesc`, string "Antialiasing"
/// cf. rbx-reflection `enum_desc_antialiasing_mode_ctor`).
pub fn render_enum_desc_antialiasing_mode_ctor() -> RenderEnumDesc {
    RenderEnumDesc::new("Antialiasing")
}

/// IDA 0x88c4 (`EnumDesc<FrameRateManagerMode>::EnumDesc`, string
/// "FramerateManagerMode" cf. rbx-reflection
/// `enum_desc_frame_rate_manager_mode_ctor`).
pub fn render_enum_desc_frame_rate_manager_mode_ctor() -> RenderEnumDesc {
    RenderEnumDesc::new("FramerateManagerMode")
}

/// IDA 0x86d0 (`EnumDesc<GraphicsMode>::EnumDesc`, string "GraphicsMode" cf.
/// rbx-reflection `enum_desc_graphics_mode_ctor`).
pub fn render_enum_desc_graphics_mode_ctor() -> RenderEnumDesc {
    RenderEnumDesc::new("GraphicsMode")
}

/// IDA 0x850c (`EnumDesc<AASamples>::EnumDesc`, string "AASamples" cf.
/// rbx-reflection `enum_desc_aa_samples_ctor`).
pub fn render_enum_desc_aa_samples_ctor() -> RenderEnumDesc {
    RenderEnumDesc::new("AASamples")
}

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x14cf4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings10ShadowModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::ShadowMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_14cf4<'a>(map: &'a mut NamePresetMap, name: &str) -> &'a mut i32 {
    // IDA 0x14cf4 (`map::operator[]`, decompiled lower_bound walk for the
    // key; on miss `_M_insert_unique` a default-constructed mapped value,
    // host: `or_default` = 0; returns the mapped reference): same shape as
    // 0x142b8. was: std::map -> HashMap (AGENTS.md section 4).
    map.entry(name.to_owned()).or_default()
}

// 0x14d4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14d4c(map: &mut NamePresetMap, name: &str, value: i32) -> bool {
    // IDA 0x14d4c (`_Rb_tree::_M_insert_unique` hint overload, decompiled
    // empty-tree/head/less-than paths, node alloc + rebalance + count++ on
    // miss): the hint position has no host effect; reports whether the key
    // was newly inserted. Same shape as 0x14310.
    use std::collections::hash_map::Entry;
    match map.entry(name.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x14e00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14e00(map: &mut NamePresetMap, name: &str, value: i32) -> Option<i32> {
    // IDA 0x14e00 (`_Rb_tree::_M_insert`, decompiled node alloc + pair copy
    // + `_Rb_tree_insert_and_rebalance` + count++): unconditional node
    // insert; the host returns the displaced value, if any. Same shape as
    // 0x143c4.
    map.insert(name.to_owned(), value)
}

// 0x14e58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode> const&)")]
pub fn stub_14e58(map: &mut NamePresetMap, name: &str, value: i32) -> bool {
    // IDA 0x14e58 (`_Rb_tree::_M_insert_unique` key overload, decompiled
    // lower_bound walk + exact-key recheck): reports whether the key was
    // newly inserted. Same shape as 0x1441c.
    use std::collections::hash_map::Entry;
    match map.entry(name.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x14ec0 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ec0<'a>(vec: &'a mut Vec<i32>, index: usize, value: &i32) -> &'a mut Vec<i32> {
    // IDA 0x14ec0 (`vector::_M_insert_aux`, decompiled doubled growth w/
    // max_size cap, allocate, prefix copy, place, suffix copy, delete old +
    // republish): grow + memmove; host `Vec::insert` is both paths. Same
    // shape as 0x144e0.
    vec.insert(index, *value);
    vec
}

// 0x14fa4 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings10ShadowModeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_allocate(unsigned long)")]
pub fn stub_14fa4(n: usize) -> Vec<i32> {
    // IDA 0x14fa4 (`_Vector_base::_M_allocate`, decompiled large-`n`
    // `__throw_bad_alloc` guard): fresh lanes; host capacity-only Vec. Same
    // shape as 0x145c4.
    assert!(n < 0x40000000, "std::bad_alloc (IDA 0x14fa4)");
    Vec::with_capacity(n)
}

// 0x14fbc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings10ShadowModeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::ShadowMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *>(RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *,RBX::CRenderSettings::ShadowMode *)")]
pub fn stub_14fbc(
    buf: &mut [i32],
    first: usize,
    last: usize,
    result_end: usize,
) -> usize {
    // IDA 0x14fbc (`__copy_b`, copy_backward, decompiled `n = last-first`
    // backward word loop): host `copy_within` (memmove). Same shape as
    // 0x145dc.
    let n = last - first;
    if n >= 1 {
        buf.copy_within(first..last, result_end - n);
    }
    result_end - n
}

// 0x14ff8 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::ShadowMode*,std::vector<RBX::CRenderSettings::ShadowMode,std::allocator<RBX::CRenderSettings::ShadowMode>>>,RBX::CRenderSettings::ShadowMode const&)")]
pub fn stub_14ff8(vec: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x14ff8 (`vector::_M_fill_insert`, decompiled growth computation +
    // allocate + prefix/value/suffix fills): inserts `n` copies of `value`
    // at `index`; the reallocation dance has no host effect beyond the
    // insert. Same shape as 0x14618.
    let tail = vec.split_off(index);
    vec.extend(std::iter::repeat_n(value, n));
    vec.extend(tail);
}

// 0x15188 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::resize(unsigned long,RBX::CRenderSettings::AntialiasingMode)")]
pub fn stub_15188(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x15188 (`vector::resize`, decompiled grow check tail-calling
    // `_M_fill_insert`, shrink finish reset): growing value-fills with
    // `AntialiasingMode()` (= 0), shrinking drops the tail. Same shape as
    // 0x14484.
    vec.resize(n, 0);
}

// 0x151bc — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::push_back(RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_151bc<'a>(vec: &'a mut Vec<i32>, value: &i32) -> &'a mut Vec<i32> {
    // IDA 0x151bc (`vector::push_back`, decompiled finish/capacity check,
    // in-place store or `_M_insert_aux` growth): both paths are `Vec::push`.
    // Same shape as 0x144b8.
    vec.push(*value);
    vec
}

// 0x151e4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings16AntialiasingModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AntialiasingMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_151e4<'a>(map: &'a mut NamePresetMap, name: &str) -> &'a mut i32 {
    // IDA 0x151e4 (`map::operator[]`, decompiled lower_bound walk for the
    // key; on miss `_M_insert_unique` a default-constructed mapped value,
    // host: `or_default` = 0; returns the mapped reference): same shape as
    // 0x142b8. was: std::map -> HashMap (AGENTS.md section 4).
    map.entry(name.to_owned()).or_default()
}

// 0x1523c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_1523c(map: &mut NamePresetMap, name: &str, value: i32) -> bool {
    // IDA 0x1523c (`_Rb_tree::_M_insert_unique` hint overload, decompiled
    // empty-tree/head/less-than paths, node alloc + rebalance + count++ on
    // miss): the hint position has no host effect; reports whether the key
    // was newly inserted. Same shape as 0x14310.
    use std::collections::hash_map::Entry;
    match map.entry(name.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x152f0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_152f0(map: &mut NamePresetMap, name: &str, value: i32) -> Option<i32> {
    // IDA 0x152f0 (`_Rb_tree::_M_insert`, decompiled node alloc + pair copy
    // + `_Rb_tree_insert_and_rebalance` + count++): unconditional node
    // insert; the host returns the displaced value, if any. Same shape as
    // 0x143c4.
    map.insert(name.to_owned(), value)
}

// 0x15348 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_15348(map: &mut NamePresetMap, name: &str, value: i32) -> bool {
    // IDA 0x15348 (`_Rb_tree::_M_insert_unique` key overload, decompiled
    // lower_bound walk + exact-key recheck): reports whether the key was
    // newly inserted. Same shape as 0x1441c.
    use std::collections::hash_map::Entry;
    match map.entry(name.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x153b0 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_153b0<'a>(vec: &'a mut Vec<i32>, index: usize, value: &i32) -> &'a mut Vec<i32> {
    // IDA 0x153b0 (`vector::_M_insert_aux`, decompiled doubled growth w/
    // max_size cap, allocate, prefix copy, place, suffix copy, delete old +
    // republish): grow + memmove; host `Vec::insert` is both paths. Same
    // shape as 0x144e0.
    vec.insert(index, *value);
    vec
}

// 0x15494 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)")]
pub fn stub_15494(n: usize) -> Vec<i32> {
    // IDA 0x15494 (`_Vector_base::_M_allocate`, decompiled large-`n`
    // `__throw_bad_alloc` guard): fresh lanes; host capacity-only Vec. Same
    // shape as 0x145c4.
    assert!(n < 0x40000000, "std::bad_alloc (IDA 0x15494)");
    Vec::with_capacity(n)
}

// 0x154ac — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)")]
pub fn stub_154ac(
    buf: &mut [i32],
    first: usize,
    last: usize,
    result_end: usize,
) -> usize {
    // IDA 0x154ac (`__copy_b`, copy_backward, decompiled `n = last-first`
    // backward word loop): host `copy_within` (memmove). Same shape as
    // 0x145dc.
    let n = last - first;
    if n >= 1 {
        buf.copy_within(first..last, result_end - n);
    }
    result_end - n
}

// 0x154e8 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_154e8(vec: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x154e8 (`vector::_M_fill_insert`, decompiled growth computation +
    // allocate + prefix/value/suffix fills): inserts `n` copies of `value`
    // at `index`; the reallocation dance has no host effect beyond the
    // insert. Same shape as 0x14618.
    let tail = vec.split_off(index);
    vec.extend(std::iter::repeat_n(value, n));
    vec.extend(tail);
}

// 0x15678 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)")]
pub fn stub_15678(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x15678 (`vector::resize`, decompiled grow check tail-calling
    // `_M_fill_insert`, shrink finish reset): growing value-fills with
    // `FrameRateManagerMode()` (= 0), shrinking drops the tail. Same shape
    // as 0x14484.
    vec.resize(n, 0);
}

// 0x156ac — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_156ac<'a>(vec: &'a mut Vec<i32>, value: &i32) -> &'a mut Vec<i32> {
    // IDA 0x156ac (`vector::push_back`, decompiled finish/capacity check,
    // in-place store or `_M_insert_aux` growth): both paths are `Vec::push`.
    // Same shape as 0x144b8.
    vec.push(*value);
    vec
}

// 0x156d4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_156d4<'a>(map: &'a mut NamePresetMap, name: &str) -> &'a mut i32 {
    // IDA 0x156d4 (`map::operator[]`, decompiled lower_bound walk for the
    // key; on miss `_M_insert_unique` a default-constructed mapped value,
    // host: `or_default` = 0; returns the mapped reference): same shape as
    // 0x142b8. was: std::map -> HashMap (AGENTS.md section 4).
    map.entry(name.to_owned()).or_default()
}

// 0x1572c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_1572c(map: &mut NamePresetMap, name: &str, value: i32) -> bool {
    // IDA 0x1572c (`_Rb_tree::_M_insert_unique` hint overload, decompiled
    // empty-tree/head/less-than paths, node alloc + rebalance + count++ on
    // miss): the hint position has no host effect; reports whether the key
    // was newly inserted. Same shape as 0x14310.
    use std::collections::hash_map::Entry;
    match map.entry(name.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x157e0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_157e0(map: &mut NamePresetMap, name: &str, value: i32) -> Option<i32> {
    // IDA 0x157e0 (`_Rb_tree::_M_insert`, decompiled node alloc + pair copy
    // + `_Rb_tree_insert_and_rebalance` + count++): unconditional node
    // insert; the host returns the displaced value, if any. Same shape as
    // 0x143c4.
    map.insert(name.to_owned(), value)
}

// 0x15838 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_15838(map: &mut NamePresetMap, name: &str, value: i32) -> bool {
    // IDA 0x15838 (`_Rb_tree::_M_insert_unique` key overload, decompiled
    // lower_bound walk + exact-key recheck): reports whether the key was
    // newly inserted. Same shape as 0x1441c.
    use std::collections::hash_map::Entry;
    match map.entry(name.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x158a0 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_158a0<'a>(vec: &'a mut Vec<i32>, index: usize, value: &i32) -> &'a mut Vec<i32> {
    // IDA 0x158a0 (`vector::_M_insert_aux`, decompiled doubled growth w/
    // max_size cap, allocate, prefix copy, place, suffix copy, delete old +
    // republish): grow + memmove; host `Vec::insert` is both paths. Same
    // shape as 0x144e0.
    vec.insert(index, *value);
    vec
}

// 0x15984 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)")]
pub fn stub_15984(n: usize) -> Vec<i32> {
    // IDA 0x15984 (`_Vector_base::_M_allocate`, decompiled large-`n`
    // `__throw_bad_alloc` guard): fresh lanes; host capacity-only Vec. Same
    // shape as 0x145c4.
    assert!(n < 0x40000000, "std::bad_alloc (IDA 0x15984)");
    Vec::with_capacity(n)
}

// 0x1599c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)")]
pub fn stub_1599c(
    buf: &mut [i32],
    first: usize,
    last: usize,
    result_end: usize,
) -> usize {
    // IDA 0x1599c (`__copy_b`, copy_backward, decompiled `n = last-first`
    // backward word loop): host `copy_within` (memmove). Same shape as
    // 0x145dc.
    let n = last - first;
    if n >= 1 {
        buf.copy_within(first..last, result_end - n);
    }
    result_end - n
}

// 0x159d8 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_159d8(vec: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x159d8 (`vector::_M_fill_insert`, decompiled growth computation +
    // allocate + prefix/value/suffix fills): inserts `n` copies of `value`
    // at `index`; the reallocation dance has no host effect beyond the
    // insert. Same shape as 0x14618.
    let tail = vec.split_off(index);
    vec.extend(std::iter::repeat_n(value, n));
    vec.extend(tail);
}

// 0x15b68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)")]
pub fn stub_15b68(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x15b68 (`vector::resize`, decompiled grow check tail-calling
    // `_M_fill_insert`, shrink finish reset): growing value-fills with
    // `GraphicsMode()` (= 0), shrinking drops the tail. Same shape as
    // 0x14484.
    vec.resize(n, 0);
}

// 0x15b9c — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_15b9c<'a>(map: &'a mut NamePresetMap, name: &str) -> &'a mut i32 {
    // IDA 0x15b9c (`map::operator[]`, decompiled lower_bound walk for the
    // key; on miss `_M_insert_unique` a default-constructed mapped value,
    // host: `or_default` = 0; returns the mapped reference): same shape as
    // 0x142b8. was: std::map -> HashMap (AGENTS.md section 4).
    map.entry(name.to_owned()).or_default()
}

// 0x15bf4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15bf4(map: &mut NamePresetMap, name: &str, value: i32) -> bool {
    // IDA 0x15bf4 (`_Rb_tree::_M_insert_unique` hint overload, decompiled
    // empty-tree/head/less-than paths, node alloc + rebalance + count++ on
    // miss): the hint position has no host effect; reports whether the key
    // was newly inserted. Same shape as 0x14310.
    use std::collections::hash_map::Entry;
    match map.entry(name.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x15ca8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15ca8(map: &mut NamePresetMap, name: &str, value: i32) -> Option<i32> {
    // IDA 0x15ca8 (`_Rb_tree::_M_insert`, decompiled node alloc + pair copy
    // + `_Rb_tree_insert_and_rebalance` + count++): unconditional node
    // insert; the host returns the displaced value, if any. Same shape as
    // 0x143c4.
    map.insert(name.to_owned(), value)
}

// 0x15d00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15d00(map: &mut NamePresetMap, name: &str, value: i32) -> bool {
    // IDA 0x15d00 (`_Rb_tree::_M_insert_unique` key overload, decompiled
    // lower_bound walk + exact-key recheck): reports whether the key was
    // newly inserted. Same shape as 0x1441c.
    use std::collections::hash_map::Entry;
    match map.entry(name.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x15d68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15d68(vec: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x15d68 (`vector::_M_fill_insert`, decompiled growth computation +
    // allocate + prefix/value/suffix fills): inserts `n` copies of `value`
    // at `index`; the reallocation dance has no host effect beyond the
    // insert. Same shape as 0x14618.
    let tail = vec.split_off(index);
    vec.extend(std::iter::repeat_n(value, n));
    vec.extend(tail);
}

// 0x15ef8 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)")]
pub fn stub_15ef8(n: usize) -> Vec<i32> {
    // IDA 0x15ef8 (`_Vector_base::_M_allocate`, decompiled large-`n`
    // `__throw_bad_alloc` guard): fresh lanes; host capacity-only Vec. Same
    // shape as 0x145c4.
    assert!(n < 0x40000000, "std::bad_alloc (IDA 0x15ef8)");
    Vec::with_capacity(n)
}

// 0x15f10 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)")]
pub fn stub_15f10(
    buf: &mut [i32],
    first: usize,
    last: usize,
    result_end: usize,
) -> usize {
    // IDA 0x15f10 (`__copy_b`, copy_backward, decompiled `n = last-first`
    // backward word loop): host `copy_within` (memmove). Same shape as
    // 0x145dc.
    let n = last - first;
    if n >= 1 {
        buf.copy_within(first..last, result_end - n);
    }
    result_end - n
}

// 0x15f4c — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15f4c<'a>(vec: &'a mut Vec<i32>, value: &i32) -> &'a mut Vec<i32> {
    // IDA 0x15f4c (`vector::push_back`, decompiled finish/capacity check,
    // in-place store or `_M_insert_aux` growth): both paths are `Vec::push`.
    // Same shape as 0x144b8.
    vec.push(*value);
    vec
}

// 0x15f74 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15f74<'a>(vec: &'a mut Vec<i32>, index: usize, value: &i32) -> &'a mut Vec<i32> {
    // IDA 0x15f74 (`vector::_M_insert_aux`, decompiled doubled growth w/
    // max_size cap, allocate, prefix copy, place, suffix copy, delete old +
    // republish): grow + memmove; host `Vec::insert` is both paths. Same
    // shape as 0x144e0.
    vec.insert(index, *value);
    vec
}

// 0x16058 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)")]
pub fn stub_16058(vec: &mut Vec<i32>, n: usize) {
    // IDA 0x16058 (`vector::resize`, decompiled grow check tail-calling
    // `_M_fill_insert`, shrink finish reset): growing value-fills with
    // `AASamples()` (= 0), shrinking drops the tail. Same shape as 0x14484.
    vec.resize(n, 0);
}

// 0x1608c — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)")]
pub fn stub_1608c<'a>(vec: &'a mut Vec<i32>, value: &i32) -> &'a mut Vec<i32> {
    // IDA 0x1608c (`vector::push_back`, decompiled finish/capacity check,
    // in-place store or `_M_insert_aux` growth): both paths are `Vec::push`.
    // Same shape as 0x144b8.
    vec.push(*value);
    vec
}

// 0x160b4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)")]
pub fn stub_160b4<'a>(map: &'a mut NamePresetMap, name: &str) -> &'a mut i32 {
    // IDA 0x160b4 (`map::operator[]`, decompiled lower_bound walk for the
    // key; on miss `_M_insert_unique` a default-constructed mapped value,
    // host: `or_default` = 0; returns the mapped reference): same shape as
    // 0x142b8. was: std::map -> HashMap (AGENTS.md section 4).
    map.entry(name.to_owned()).or_default()
}

// 0x1610c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_1610c(map: &mut NamePresetMap, name: &str, value: i32) -> bool {
    // IDA 0x1610c (`_Rb_tree::_M_insert_unique` hint overload, decompiled
    // empty-tree/head/less-than paths, node alloc + rebalance + count++ on
    // miss): the hint position has no host effect; reports whether the key
    // was newly inserted. Same shape as 0x14310.
    use std::collections::hash_map::Entry;
    match map.entry(name.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x161c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_161c0(map: &mut NamePresetMap, name: &str, value: i32) -> Option<i32> {
    // IDA 0x161c0 (`_Rb_tree::_M_insert`, decompiled node alloc + pair copy
    // + `_Rb_tree_insert_and_rebalance` + count++): unconditional node
    // insert; the host returns the displaced value, if any. Same shape as
    // 0x143c4.
    map.insert(name.to_owned(), value)
}

// 0x16218 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_16218(map: &mut NamePresetMap, name: &str, value: i32) -> bool {
    // IDA 0x16218 (`_Rb_tree::_M_insert_unique` key overload, decompiled
    // lower_bound walk + exact-key recheck): reports whether the key was
    // newly inserted. Same shape as 0x1441c.
    use std::collections::hash_map::Entry;
    match map.entry(name.to_owned()) {
        Entry::Vacant(slot) => {
            slot.insert(value);
            true
        }
        Entry::Occupied(_) => false,
    }
}

// 0x16280 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)")]
pub fn stub_16280<'a>(vec: &'a mut Vec<i32>, index: usize, value: &i32) -> &'a mut Vec<i32> {
    // IDA 0x16280 (`vector::_M_insert_aux`, decompiled doubled growth w/
    // max_size cap, allocate, prefix copy, place, suffix copy, delete old +
    // republish): grow + memmove; host `Vec::insert` is both paths. Same
    // shape as 0x144e0.
    vec.insert(index, *value);
    vec
}

// 0x16364 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)")]
pub fn stub_16364(n: usize) -> Vec<i32> {
    // IDA 0x16364 (`_Vector_base::_M_allocate`, decompiled large-`n`
    // `__throw_bad_alloc` guard): fresh lanes; host capacity-only Vec. Same
    // shape as 0x145c4.
    assert!(n < 0x40000000, "std::bad_alloc (IDA 0x16364)");
    Vec::with_capacity(n)
}

// 0x1637c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_
#[doc(alias = "RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)")]
pub fn stub_1637c(
    buf: &mut [i32],
    first: usize,
    last: usize,
    result_end: usize,
) -> usize {
    // IDA 0x1637c (`__copy_b`, copy_backward, decompiled `n = last-first`
    // backward word loop): host `copy_within` (memmove). Same shape as
    // 0x145dc.
    let n = last - first;
    if n >= 1 {
        buf.copy_within(first..last, result_end - n);
    }
    result_end - n
}

// 0x163b8 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)")]
pub fn stub_163b8(vec: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x163b8 (`vector::_M_fill_insert`, decompiled growth computation +
    // allocate + prefix/value/suffix fills): inserts `n` copies of `value`
    // at `index`; the reallocation dance has no host effect beyond the
    // insert. Same shape as 0x14618.
    let tail = vec.split_off(index);
    vec.extend(std::iter::repeat_n(value, n));
    vec.extend(tail);
}

// 0x16548 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::initSingleton(void)")]
pub fn stub_16548() -> &'static RenderEnumDesc {
    // IDA 0x16548 (`initSingleton`, `// attributes: thunk`): tail-calls
    // `doGetSingleton` (0x1654c).
    stub_1654c()
}

// 0x1654c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)")]
pub fn stub_1654c() -> &'static RenderEnumDesc {
    // IDA 0x1654c (`doGetSingleton`: `__cxa_guard_acquire` once-guard,
    // in-place `EnumDesc::EnumDesc` (0x8c4c) + `__cxa_atexit` dtor): host
    // `LazyLock`; destructor runs at process exit.
    static S: std::sync::LazyLock<RenderEnumDesc> =
        std::sync::LazyLock::new(render_enum_desc_shadow_mode_ctor);
    &S
}

// 0x1663c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::initSingleton(void)")]
pub fn stub_1663c() -> &'static RenderEnumDesc {
    // IDA 0x1663c (`initSingleton`, `// attributes: thunk`): tail-calls
    // `doGetSingleton` (0x16640).
    stub_16640()
}

// 0x16640 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)")]
pub fn stub_16640() -> &'static RenderEnumDesc {
    // IDA 0x16640 (`doGetSingleton`: `__cxa_guard_acquire` once-guard,
    // in-place `EnumDesc::EnumDesc` (0x9100) + `__cxa_atexit` dtor): host
    // `LazyLock`; destructor runs at process exit.
    static S: std::sync::LazyLock<RenderEnumDesc> =
        std::sync::LazyLock::new(render_enum_desc_resolution_preset_ctor);
    &S
}

// 0x16730 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::initSingleton(void)")]
pub fn stub_16730() -> &'static RenderEnumDesc {
    // IDA 0x16730 (`initSingleton`, `// attributes: thunk`): tail-calls
    // `doGetSingleton` (0x16734).
    stub_16734()
}

// 0x16734 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)")]
pub fn stub_16734() -> &'static RenderEnumDesc {
    // IDA 0x16734 (`doGetSingleton`: `__cxa_guard_acquire` once-guard,
    // in-place `EnumDesc::EnumDesc` (0x8e24) + `__cxa_atexit` dtor): host
    // `LazyLock`; destructor runs at process exit.
    static S: std::sync::LazyLock<RenderEnumDesc> =
        std::sync::LazyLock::new(render_enum_desc_quality_level_ctor);
    &S
}

// 0x16824 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::initSingleton(void)")]
pub fn stub_16824() -> &'static RenderEnumDesc {
    // IDA 0x16824 (`initSingleton`, `// attributes: thunk`): tail-calls
    // `doGetSingleton` (0x16828).
    stub_16828()
}

// 0x16828 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)")]
pub fn stub_16828() -> &'static RenderEnumDesc {
    // IDA 0x16828 (`doGetSingleton`: `__cxa_guard_acquire` once-guard,
    // in-place `EnumDesc::EnumDesc` (0x8a88) + `__cxa_atexit` dtor): host
    // `LazyLock`; destructor runs at process exit.
    static S: std::sync::LazyLock<RenderEnumDesc> =
        std::sync::LazyLock::new(render_enum_desc_antialiasing_mode_ctor);
    &S
}

// 0x16918 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::initSingleton(void)")]
pub fn stub_16918() -> &'static RenderEnumDesc {
    // IDA 0x16918 (`initSingleton`, `// attributes: thunk`): tail-calls
    // `doGetSingleton` (0x1691c).
    stub_1691c()
}

// 0x1691c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)")]
pub fn stub_1691c() -> &'static RenderEnumDesc {
    // IDA 0x1691c (`doGetSingleton`: `__cxa_guard_acquire` once-guard,
    // in-place `EnumDesc::EnumDesc` (0x88c4) + `__cxa_atexit` dtor): host
    // `LazyLock`; destructor runs at process exit.
    static S: std::sync::LazyLock<RenderEnumDesc> =
        std::sync::LazyLock::new(render_enum_desc_frame_rate_manager_mode_ctor);
    &S
}

// 0x16a0c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::initSingleton(void)")]
pub fn stub_16a0c() -> &'static RenderEnumDesc {
    // IDA 0x16a0c (`initSingleton`, `// attributes: thunk`): tail-calls
    // `doGetSingleton` (0x16a10).
    stub_16a10()
}

// 0x16a10 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)")]
pub fn stub_16a10() -> &'static RenderEnumDesc {
    // IDA 0x16a10 (`doGetSingleton`: `__cxa_guard_acquire` once-guard,
    // in-place `EnumDesc::EnumDesc` (0x86d0) + `__cxa_atexit` dtor): host
    // `LazyLock`; destructor runs at process exit.
    static S: std::sync::LazyLock<RenderEnumDesc> =
        std::sync::LazyLock::new(render_enum_desc_graphics_mode_ctor);
    &S
}

// 0x16b00 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::initSingleton(void)")]
pub fn stub_16b00() -> &'static RenderEnumDesc {
    // IDA 0x16b00 (`initSingleton`, `// attributes: thunk`): tail-calls
    // `doGetSingleton` (0x16b04).
    stub_16b04()
}

// 0x16b04 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)")]
pub fn stub_16b04() -> &'static RenderEnumDesc {
    // IDA 0x16b04 (`doGetSingleton`: `__cxa_guard_acquire` once-guard,
    // in-place `EnumDesc::EnumDesc` (0x850c) + `__cxa_atexit` dtor): host
    // `LazyLock`; destructor runs at process exit.
    static S: std::sync::LazyLock<RenderEnumDesc> =
        std::sync::LazyLock::new(render_enum_desc_aa_samples_ctor);
    &S
}

// 0x16bf4 — __ZN19CRenderSettingsItemD2Ev
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_16bf4() {
    // IDA 0x16bf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x16d34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)")]
pub fn stub_16d34() {
    // IDA 0x16d34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x16d5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)")]
pub fn stub_16d5c() {
    // IDA 0x16d5c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x16d84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)")]
pub fn stub_16d84() {
    // IDA 0x16d84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x16dac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)")]
pub fn stub_16dac() {
    // IDA 0x16dac: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x16dd4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)")]
pub fn stub_16dd4() {
    // IDA 0x16dd4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x16dfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)")]
pub fn stub_16dfc() {
    // IDA 0x16dfc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x16e24 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)")]
pub fn stub_16e24() {
    // IDA 0x16e24: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x16e4c — __GLOBAL__I_a
// was: global constructor keyed to_a
#[doc(alias = "global constructor keyed to_a")]
pub fn stub_16e4c() {
    // IDA 0x16e4c (global static ctor: `boost::system::generic_category` /
    // `system_category` stores 0x16e56..0x16e70, `std::ios_base::Init::Init`
    // + `__cxa_atexit` 0x16e72..0x16e94, registrar guards after 0x16e98):
    // host statics need no init; ErrorCode categories have no host
    // counterpart. Static init — carrier no-op.
}

// 0x179e8 — __ZN3RBX9DataModel10serverSaveEv
#[doc(alias = "RBX::DataModel::serverSave(void)")]
pub fn stub_179e8() {
    // IDA 0x179e8 (decompiled empty body `;`): no observable behavior.
    // DataModel is owned by the datamodel crate; carrier no-op here.
}

// 0x179ec — __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
#[doc(alias = "RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)")]
pub fn stub_179ec() {
    // IDA 0x179ec (decompiled empty body `;`): no observable behavior.
    // was: boost::function -> Box<dyn Fn> (AGENTS.md section 4); nothing to
    // invoke. Carrier no-op.
}

// 0x179f0 — __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
#[doc(alias = "RBX::DataModel::internalSave(RBX::ContentId)")]
pub fn stub_179f0() {
    // IDA 0x179f0 (decompiled empty body `;`): no observable behavior.
    // Carrier no-op.
}

// 0x179f4 — __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_179f4() {
    // IDA 0x179f4..0x17aa2 (decompiled 0x179f4..: `operator new(0xC)`,
    // null `shared_ptr<Tuple>` + aliasing const copy, `sp_counted_base`
    // `release`; no other calls): net-nothing shared_ptr juggling — was:
    // boost::shared_ptr -> rbx_core::SharedPtr (Arc), whose Drop covers the
    // release. Carrier no-op.
}

// 0x17c58 — __GLOBAL__I_a_0
// was: global constructor keyed to_a_0
#[doc(alias = "global constructor keyed to_a_0")]
pub fn stub_17c58() {
    // IDA 0x17c58 (global static ctor twin of 0x16e4c: boost category
    // stores 0x17c5c..0x17c76, `std::ios_base::Init::Init` +
    // `__cxa_atexit` 0x17c78..0x17c9a, exception-object guards after
    // 0x17c9e): static init — carrier no-op.
}

/// Audio-crate host for the `Appirater` ObjC class cluster (IDA
/// 0x17df0..0x180a8): class-level config slots plus the `dispatch_once`
/// shared instance. `NSUserDefaults` persistence and `UIAlertView` collapse
/// to plain host state; `id` tokens are `u64` (`0` is `nil`). Mirrors the
/// platform crate `Appirater` model (which owns the full state machine);
/// audio cannot depend on platform (AGENTS.md DAG), so the slots these
/// filler EAs touch live here.
#[derive(Debug, Default)]
pub struct AudioAppirater {
    app_id: parking_lot::Mutex<String>,
    days_until_prompt: parking_lot::Mutex<f64>,
    uses_until_prompt: std::sync::atomic::AtomicU32,
    significant_events_until_prompt: std::sync::atomic::AtomicU32,
    time_before_reminding: parking_lot::Mutex<f64>,
    debug: std::sync::atomic::AtomicBool,
    pending_delegate: parking_lot::Mutex<u64>,
    delegate: parking_lot::Mutex<u64>,
    resign_active_observed: std::sync::atomic::AtomicBool,
    network_reachable: std::sync::atomic::AtomicBool,
    rating_alert_shows: std::sync::atomic::AtomicU32,
    rating_alert_visible: std::sync::atomic::AtomicBool,
}

impl AudioAppirater {
    fn shared() -> &'static Self {
        static APPIRATER: std::sync::LazyLock<AudioAppirater> =
            std::sync::LazyLock::new(|| {
                let appirater = AudioAppirater::default();
                // Reachability reads clean on a live device, so the
                // `connectedToNetwork` (IDA 0x17e68) fast path starts
                // reachable.
                appirater
                    .network_reachable
                    .store(true, std::sync::atomic::Ordering::SeqCst);
                appirater
            });
        &APPIRATER
    }
}

// 0x17df0 — +[Appirater setAppId:]
#[doc(alias = "+[Appirater setAppId:]")]
pub fn stub_17df0(app_id: &str) {
    // IDA 0x17df0 (`+[Appirater setAppId:]`, ObjC class setter): stores the
    // class-level app id slot.
    *AudioAppirater::shared().app_id.lock() = app_id.to_owned();
}

// 0x17e00 — +[Appirater setDaysUntilPrompt:]
#[doc(alias = "+[Appirater setDaysUntilPrompt:]")]
pub fn stub_17e00(days: f64) {
    // IDA 0x17e00 (`+[Appirater setDaysUntilPrompt:]`): stores the
    // class-level days-until-prompt slot.
    *AudioAppirater::shared().days_until_prompt.lock() = days;
}

// 0x17e14 — +[Appirater setUsesUntilPrompt:]
#[doc(alias = "+[Appirater setUsesUntilPrompt:]")]
pub fn stub_17e14(uses: u32) {
    // IDA 0x17e14 (`+[Appirater setUsesUntilPrompt:]`): stores the
    // class-level uses-until-prompt slot.
    AudioAppirater::shared()
        .uses_until_prompt
        .store(uses, std::sync::atomic::Ordering::SeqCst);
}

// 0x17e24 — +[Appirater setSignificantEventsUntilPrompt:]
#[doc(alias = "+[Appirater setSignificantEventsUntilPrompt:]")]
pub fn stub_17e24(count: u32) {
    // IDA 0x17e24 (`+[Appirater setSignificantEventsUntilPrompt:]`): stores
    // the class-level significant-events-until-prompt slot.
    AudioAppirater::shared()
        .significant_events_until_prompt
        .store(count, std::sync::atomic::Ordering::SeqCst);
}

// 0x17e34 — +[Appirater setTimeBeforeReminding:]
#[doc(alias = "+[Appirater setTimeBeforeReminding:]")]
pub fn stub_17e34(days: f64) {
    // IDA 0x17e34 (`+[Appirater setTimeBeforeReminding:]`): stores the
    // class-level time-before-reminding slot.
    *AudioAppirater::shared().time_before_reminding.lock() = days;
}

// 0x17e48 — +[Appirater setDebug:]
#[doc(alias = "+[Appirater setDebug:]")]
pub fn stub_17e48(debug: bool) {
    // IDA 0x17e48 (`+[Appirater setDebug:]`): stores the class-level debug
    // slot.
    AudioAppirater::shared()
        .debug
        .store(debug, std::sync::atomic::Ordering::SeqCst);
}

// 0x17e58 — +[Appirater setDelegate:]
#[doc(alias = "+[Appirater setDelegate:]")]
pub fn stub_17e58(delegate: u64) {
    // IDA 0x17e58 (`+[Appirater setDelegate:]`): stores the class-level
    // delegate slot consumed by the `sharedInstance` block (0x17fe4); `0`
    // is `nil`.
    *AudioAppirater::shared().pending_delegate.lock() = delegate;
}

// 0x17e68 — -[Appirater connectedToNetwork]
#[doc(alias = "-[Appirater connectedToNetwork]")]
pub fn stub_17e68() -> bool {
    // IDA 0x17e68 (`-[Appirater connectedToNetwork]` reachability check):
    // host reachability starts clean (cf. `AudioAppirater::shared`).
    AudioAppirater::shared()
        .network_reachable
        .load(std::sync::atomic::Ordering::SeqCst)
}

// 0x17f80 — +[Appirater sharedInstance]
#[doc(alias = "+[Appirater sharedInstance]")]
pub fn stub_17f80() -> &'static AudioAppirater {
    // IDA 0x17f80 (`+[Appirater sharedInstance]`: nil-check +
    // `dispatch_once` over the 0x17fe4 block): `LazyLock` is the once cell.
    AudioAppirater::shared()
}

// 0x17fe4 — ___27+[Appirater sharedInstance]_block_invoke
#[doc(alias = "___27+[Appirater sharedInstance]_block_invoke")]
pub fn stub_17fe4() -> &'static AudioAppirater {
    // IDA 0x17fe4 (`__27+[Appirater sharedInstance]_block_invoke`:
    // `[[Appirater alloc] init]`, `setDelegate:` from the class-level slot,
    // `addObserver:...appWillResignActive`): publishes the shared instance,
    // moves the pending delegate onto it, and arms the resign-active
    // observer flag.
    let inst = AudioAppirater::shared();
    *inst.delegate.lock() = *inst.pending_delegate.lock();
    inst.resign_active_observed
        .store(true, std::sync::atomic::Ordering::SeqCst);
    inst
}

// 0x18094 — ___copy_helper_block_
#[doc(alias = "___copy_helper_block_")]
pub fn stub_18094() {
    // IDA 0x18094 (`__copy_helper_block_`: `_Block_object_assign` of the
    // captured `self` at +20): ARC `Block_copy` — carrier no-op.
}

// 0x180a0 — ___destroy_helper_block_
#[doc(alias = "___destroy_helper_block_")]
pub fn stub_180a0() {
    // IDA 0x180a0 (`__destroy_helper_block_`: `_Block_object_dispose` of
    // the capture at +20): ARC `Block_release` — carrier no-op.
}

// 0x180a8 — -[Appirater showRatingAlert]
#[doc(alias = "-[Appirater showRatingAlert]")]
pub fn stub_180a8() {
    // IDA 0x180a8 (`-[Appirater showRatingAlert]`: localized bundle strings
    // + `UIAlertView` presentation, delegate-gated): the alert view
    // collapses to the visible flag + show count; the delegate gating and
    // presentation live in the platform crate twin.
    let inst = AudioAppirater::shared();
    inst.rating_alert_shows
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    inst.rating_alert_visible
        .store(true, std::sync::atomic::Ordering::SeqCst);
}

// 0x183d8 — -[Appirater ratingConditionsHaveBeenMet]
#[doc(alias = "-[Appirater ratingConditionsHaveBeenMet]")]
pub fn stub_183d8() -> ! {
    todo!("0x183d8 -[Appirater ratingConditionsHaveBeenMet]")
}

// 0x185b0 — -[Appirater incrementUseCount]
#[doc(alias = "-[Appirater incrementUseCount]")]
pub fn stub_185b0() -> ! {
    todo!("0x185b0 -[Appirater incrementUseCount]")
}

// 0x18878 — -[Appirater incrementSignificantEventCount]
#[doc(alias = "-[Appirater incrementSignificantEventCount]")]
pub fn stub_18878() -> ! {
    todo!("0x18878 -[Appirater incrementSignificantEventCount]")
}

// 0x18b18 — -[Appirater incrementAndRate:]
#[doc(alias = "-[Appirater incrementAndRate:]")]
pub fn stub_18b18() -> ! {
    todo!("0x18b18 -[Appirater incrementAndRate:]")
}

// 0x18bb4 — ___30-[Appirater incrementAndRate:]_block_invoke
#[doc(alias = "___30-[Appirater incrementAndRate:]_block_invoke")]
pub fn stub_18bb4() -> ! {
    todo!("0x18bb4 ___30-[Appirater incrementAndRate:]_block_invoke")
}

// 0x18bc8 — ___copy_helper_block_125
#[doc(alias = "___copy_helper_block_125")]
pub fn stub_18bc8() -> ! {
    todo!("0x18bc8 ___copy_helper_block_125")
}

// 0x18bd4 — ___destroy_helper_block_126
#[doc(alias = "___destroy_helper_block_126")]
pub fn stub_18bd4() -> ! {
    todo!("0x18bd4 ___destroy_helper_block_126")
}

// 0x18bdc — -[Appirater incrementSignificantEventAndRate:]
#[doc(alias = "-[Appirater incrementSignificantEventAndRate:]")]
pub fn stub_18bdc() -> ! {
    todo!("0x18bdc -[Appirater incrementSignificantEventAndRate:]")
}

// 0x18c78 — ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
#[doc(alias = "___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")]
pub fn stub_18c78() -> ! {
    todo!("0x18c78 ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")
}

// 0x18c8c — ___copy_helper_block_130
#[doc(alias = "___copy_helper_block_130")]
pub fn stub_18c8c() -> ! {
    todo!("0x18c8c ___copy_helper_block_130")
}

// 0x18c98 — ___destroy_helper_block_131
#[doc(alias = "___destroy_helper_block_131")]
pub fn stub_18c98() -> ! {
    todo!("0x18c98 ___destroy_helper_block_131")
}
