//! Auto-generated skeletons for rbx-network — RBX::Network|RakNet filtered EA-sorted ascending
//! Filter: RakNet|RBX::Network -> 4479 funcs, 4479 already stubbed (0 remaining before batch); filler global ascending
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x152f0..0x19028 | existing 16859 -> 16959 total (filler global ascending EA-sorted, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_138::{EnumDescModel, RenderSettingsItem};

// 0x152f0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_152f0(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) {
    // IDA 0x152f0: _Rb_tree<AntialiasingMode>::_M_insert — fresh 0x18 node, key+value copy (cf. 0x143c4: 0x143f4..0x143fa), rebalance (0x14406), count++ (0x14410); the node heap folds into the HashMap entry.
        map.insert(name.to_owned(), value);}

// 0x15348 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode> const&)")]
pub fn stub_15348(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x15348: _Rb_tree<AntialiasingMode>::_M_insert_unique(value) — lower_bound + dup check (cf. 0x1441c: 0x14428..0x1447a); existing keys keep their value. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x153b0 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_153b0(xs: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x153b0: vector<AntialiasingMode>::_M_insert_aux — same shift-store template as 0xf704 (finish bump, copy_backward, store); Vec::insert covers both arms.
        xs.insert(index, value);}

// 0x15494 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_allocate(unsigned long)")]
pub fn stub_15494(n: usize) -> Vec<i32> {
    // IDA 0x15494: _Vector_base<AntialiasingMode>::_M_allocate — same template as 0xf7e8 (bad_alloc past the max, else operator new(4n)); with_capacity is the uninit-storage carrier.
        Vec::with_capacity(n)}

// 0x154ac — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings16AntialiasingModeES6_EET0_T_S8_S7_
// demangled: RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::AntialiasingMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *>(RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *,RBX::CRenderSettings::AntialiasingMode *)")]
pub fn stub_154ac(xs: &mut [i32], first: usize, last: usize, result: usize) -> usize {
    // IDA 0x154ac: __copy_backward<AntialiasingMode> — same word-loop template as 0xf800; copy_within is the overlapping-backward carrier.
        let n = last - first;
    xs.copy_within(first..last, result - n);
    result - n}

// 0x154e8 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AntialiasingMode*,std::vector<RBX::CRenderSettings::AntialiasingMode,std::allocator<RBX::CRenderSettings::AntialiasingMode>>>,unsigned long,RBX::CRenderSettings::AntialiasingMode const&)")]
pub fn stub_154e8(xs: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x154e8: vector<AntialiasingMode>::_M_fill_insert — n-copy fill at pos with spare/realloc paths (cf. 0x14618); splice with repeat covers all arms.
        xs.splice(index..index, std::iter::repeat(value).take(n));}

// 0x15678 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::resize(unsigned long,RBX::CRenderSettings::FrameRateManagerMode)")]
pub fn stub_15678(xs: &mut Vec<i32>, n: usize, value: i32) {
    // IDA 0x15678: vector<FrameRateManagerMode>::resize — shrink finish (cf. 0x14484: 0x144a2) or _M_fill_insert grow (0x144ac); Vec::resize covers both arms.
        xs.resize(n, value);}

// 0x156ac — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::push_back(RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_156ac(xs: &mut Vec<i32>, value: i32) {
    // IDA 0x156ac: vector<FrameRateManagerMode>::push_back — fast store + finish bump (cf. 0x144b8: 0x144c8..0x144d0), _M_insert_aux when full (0x144da); Vec::push covers both arms.
        xs.push(value);}

// 0x156d4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings20FrameRateManagerModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::FrameRateManagerMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_156d4(map: &mut std::collections::HashMap<String, i32>, name: &str) -> i32 {
    // IDA 0x156d4: map<Name const*, FrameRateManagerMode>::operator[] — lower_bound walk (cf. 0x142b8: 0x142d0..0x142f0); miss inserts via _M_insert_unique (0x14304), returns &mapped (0x1430c). T() default is 0 for the int enum; entry API is the same observable.
        *map.entry(name.to_owned()).or_insert(0)}

// 0x1572c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_1572c(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x1572c: _Rb_tree<FrameRateManagerMode>::_M_insert_unique(hint, value) — hint-aware unique insert into the name→value map (cf. 0x14310, falling into _M_insert at 0x1435e); HashMap has no order hints. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x157e0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_157e0(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) {
    // IDA 0x157e0: _Rb_tree<FrameRateManagerMode>::_M_insert — fresh 0x18 node, key+value copy (cf. 0x143c4: 0x143f4..0x143fa), rebalance (0x14406), count++ (0x14410); the node heap folds into the HashMap entry.
        map.insert(name.to_owned(), value);}

// 0x15838 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode> const&)")]
pub fn stub_15838(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x15838: _Rb_tree<FrameRateManagerMode>::_M_insert_unique(value) — lower_bound + dup check (cf. 0x1441c: 0x14428..0x1447a); existing keys keep their value. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x158a0 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_158a0(xs: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x158a0: vector<FrameRateManagerMode>::_M_insert_aux — same shift-store template as 0xf704 (finish bump, copy_backward, store); Vec::insert covers both arms.
        xs.insert(index, value);}

// 0x15984 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_allocate(unsigned long)")]
pub fn stub_15984(n: usize) -> Vec<i32> {
    // IDA 0x15984: _Vector_base<FrameRateManagerMode>::_M_allocate — same template as 0xf7e8 (bad_alloc past the max, else operator new(4n)); with_capacity is the uninit-storage carrier.
        Vec::with_capacity(n)}

// 0x1599c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings20FrameRateManagerModeES6_EET0_T_S8_S7_
// demangled: RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::FrameRateManagerMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *>(RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *,RBX::CRenderSettings::FrameRateManagerMode *)")]
pub fn stub_1599c(xs: &mut [i32], first: usize, last: usize, result: usize) -> usize {
    // IDA 0x1599c: __copy_backward<FrameRateManagerMode> — same word-loop template as 0xf800; copy_within is the overlapping-backward carrier.
        let n = last - first;
    xs.copy_within(first..last, result - n);
    result - n}

// 0x159d8 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::FrameRateManagerMode*,std::vector<RBX::CRenderSettings::FrameRateManagerMode,std::allocator<RBX::CRenderSettings::FrameRateManagerMode>>>,unsigned long,RBX::CRenderSettings::FrameRateManagerMode const&)")]
pub fn stub_159d8(xs: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x159d8: vector<FrameRateManagerMode>::_M_fill_insert — n-copy fill at pos with spare/realloc paths (cf. 0x14618); splice with repeat covers all arms.
        xs.splice(index..index, std::iter::repeat(value).take(n));}

// 0x15b68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::resize(unsigned long,RBX::CRenderSettings::GraphicsMode)")]
pub fn stub_15b68(xs: &mut Vec<i32>, n: usize, value: i32) {
    // IDA 0x15b68: vector<GraphicsMode>::resize — shrink finish (cf. 0x14484: 0x144a2) or _M_fill_insert grow (0x144ac); Vec::resize covers both arms.
        xs.resize(n, value);}

// 0x15b9c — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings12GraphicsModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::GraphicsMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_15b9c(map: &mut std::collections::HashMap<String, i32>, name: &str) -> i32 {
    // IDA 0x15b9c: map<Name const*, GraphicsMode>::operator[] — lower_bound walk (cf. 0x142b8: 0x142d0..0x142f0); miss inserts via _M_insert_unique (0x14304), returns &mapped (0x1430c). T() default is 0 for the int enum; entry API is the same observable.
        *map.entry(name.to_owned()).or_insert(0)}

// 0x15bf4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15bf4(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x15bf4: _Rb_tree<GraphicsMode>::_M_insert_unique(hint, value) — hint-aware unique insert into the name→value map (cf. 0x14310, falling into _M_insert at 0x1435e); HashMap has no order hints. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x15ca8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15ca8(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) {
    // IDA 0x15ca8: _Rb_tree<GraphicsMode>::_M_insert — fresh 0x18 node, key+value copy (cf. 0x143c4: 0x143f4..0x143fa), rebalance (0x14406), count++ (0x14410); the node heap folds into the HashMap entry.
        map.insert(name.to_owned(), value);}

// 0x15d00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode> const&)")]
pub fn stub_15d00(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x15d00: _Rb_tree<GraphicsMode>::_M_insert_unique(value) — lower_bound + dup check (cf. 0x1441c: 0x14428..0x1447a); existing keys keep their value. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x15d68 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,unsigned long,RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15d68(xs: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x15d68: vector<GraphicsMode>::_M_fill_insert — n-copy fill at pos with spare/realloc paths (cf. 0x14618); splice with repeat covers all arms.
        xs.splice(index..index, std::iter::repeat(value).take(n));}

// 0x15ef8 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_allocate(unsigned long)")]
pub fn stub_15ef8(n: usize) -> Vec<i32> {
    // IDA 0x15ef8: _Vector_base<GraphicsMode>::_M_allocate — same template as 0xf7e8 (bad_alloc past the max, else operator new(4n)); with_capacity is the uninit-storage carrier.
        Vec::with_capacity(n)}

// 0x15f10 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings12GraphicsModeES6_EET0_T_S8_S7_
// demangled: RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::GraphicsMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *>(RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *,RBX::CRenderSettings::GraphicsMode *)")]
pub fn stub_15f10(xs: &mut [i32], first: usize, last: usize, result: usize) -> usize {
    // IDA 0x15f10: __copy_backward<GraphicsMode> — same word-loop template as 0xf800; copy_within is the overlapping-backward carrier.
        let n = last - first;
    xs.copy_within(first..last, result - n);
    result - n}

// 0x15f4c — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::push_back(RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15f4c(xs: &mut Vec<i32>, value: i32) {
    // IDA 0x15f4c: vector<GraphicsMode>::push_back — fast store + finish bump (cf. 0x144b8: 0x144c8..0x144d0), _M_insert_aux when full (0x144da); Vec::push covers both arms.
        xs.push(value);}

// 0x15f74 — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::GraphicsMode*,std::vector<RBX::CRenderSettings::GraphicsMode,std::allocator<RBX::CRenderSettings::GraphicsMode>>>,RBX::CRenderSettings::GraphicsMode const&)")]
pub fn stub_15f74(xs: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x15f74: vector<GraphicsMode>::_M_insert_aux — same shift-store template as 0xf704 (finish bump, copy_backward, store); Vec::insert covers both arms.
        xs.insert(index, value);}

// 0x16058 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::resize(unsigned long,RBX::CRenderSettings::AASamples)")]
pub fn stub_16058(xs: &mut Vec<i32>, n: usize, value: i32) {
    // IDA 0x16058: vector<AASamples>::resize — shrink finish (cf. 0x14484: 0x144a2) or _M_fill_insert grow (0x144ac); Vec::resize covers both arms.
        xs.resize(n, value);}

// 0x1608c — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::push_back(RBX::CRenderSettings::AASamples const&)")]
pub fn stub_1608c(xs: &mut Vec<i32>, value: i32) {
    // IDA 0x1608c: vector<AASamples>::push_back — fast store + finish bump (cf. 0x144b8: 0x144c8..0x144d0), _M_insert_aux when full (0x144da); Vec::push covers both arms.
        xs.push(value);}

// 0x160b4 — __ZNSt3mapIPKN3RBX4NameENS0_15CRenderSettings9AASamplesESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::CRenderSettings::AASamples,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::operator[](RBX::Name const* const&)")]
pub fn stub_160b4(map: &mut std::collections::HashMap<String, i32>, name: &str) -> i32 {
    // IDA 0x160b4: map<Name const*, AASamples>::operator[] — lower_bound walk (cf. 0x142b8: 0x142d0..0x142f0); miss inserts via _M_insert_unique (0x14304), returns &mapped (0x1430c). T() default is 0 for the int enum; entry API is the same observable.
        *map.entry(name.to_owned()).or_insert(0)}

// 0x1610c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_1610c(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x1610c: _Rb_tree<AASamples>::_M_insert_unique(hint, value) — hint-aware unique insert into the name→value map (cf. 0x14310, falling into _M_insert at 0x1435e); HashMap has no order hints. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x161c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_161c0(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) {
    // IDA 0x161c0: _Rb_tree<AASamples>::_M_insert — fresh 0x18 node, key+value copy (cf. 0x143c4: 0x143f4..0x143fa), rebalance (0x14406), count++ (0x14410); the node heap folds into the HashMap entry.
        map.insert(name.to_owned(), value);}

// 0x16218 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples> const&)")]
pub fn stub_16218(map: &mut std::collections::HashMap<String, i32>, name: &str, value: i32) -> bool {
    // IDA 0x16218: _Rb_tree<AASamples>::_M_insert_unique(value) — lower_bound + dup check (cf. 0x1441c: 0x14428..0x1447a); existing keys keep their value. True when newly inserted.
        map.insert(name.to_owned(), value).is_none()}

// 0x16280 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,RBX::CRenderSettings::AASamples const&)")]
pub fn stub_16280(xs: &mut Vec<i32>, index: usize, value: i32) {
    // IDA 0x16280: vector<AASamples>::_M_insert_aux — same shift-store template as 0xf704 (finish bump, copy_backward, store); Vec::insert covers both arms.
        xs.insert(index, value);}

// 0x16364 — __ZNSt12_Vector_baseIN3RBX15CRenderSettings9AASamplesESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_allocate(unsigned long)")]
pub fn stub_16364(n: usize) -> Vec<i32> {
    // IDA 0x16364: _Vector_base<AASamples>::_M_allocate — same template as 0xf7e8 (bad_alloc past the max, else operator new(4n)); with_capacity is the uninit-storage carrier.
        Vec::with_capacity(n)}

// 0x1637c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15CRenderSettings9AASamplesES6_EET0_T_S8_S7_
// demangled: RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CRenderSettings::AASamples * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *>(RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *,RBX::CRenderSettings::AASamples *)")]
pub fn stub_1637c(xs: &mut [i32], first: usize, last: usize, result: usize) -> usize {
    // IDA 0x1637c: __copy_backward<AASamples> — same word-loop template as 0xf800; copy_within is the overlapping-backward carrier.
        let n = last - first;
    xs.copy_within(first..last, result - n);
    result - n}

// 0x163b8 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)
// type: int(void)
#[doc(alias = "std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CRenderSettings::AASamples*,std::vector<RBX::CRenderSettings::AASamples,std::allocator<RBX::CRenderSettings::AASamples>>>,unsigned long,RBX::CRenderSettings::AASamples const&)")]
pub fn stub_163b8(xs: &mut Vec<i32>, index: usize, n: usize, value: i32) {
    // IDA 0x163b8: vector<AASamples>::_M_fill_insert — n-copy fill at pos with spare/realloc paths (cf. 0x14618); splice with repeat covers all arms.
        xs.splice(index..index, std::iter::repeat(value).take(n));}

/// Once-guarded `Singleton<EnumDesc<T>>` tables (IDA 0x1654c family):
/// each image static runs the EnumDesc C2 under `__cxa_guard` + `__cxa_atexit`
/// dtor; the host keeps a `LazyLock<EnumDescModel>` populated by the same
/// C2 ports from `generated_138`.
static SHADOW_MODE_ENUM_DESC: std::sync::LazyLock<EnumDescModel> = std::sync::LazyLock::new(|| { let mut d = EnumDescModel::default(); crate::generated_138::stub_8c4c(&mut d); d });
static RESOLUTION_PRESET_ENUM_DESC: std::sync::LazyLock<EnumDescModel> = std::sync::LazyLock::new(|| { let mut d = EnumDescModel::default(); crate::generated_138::stub_9100(&mut d); d });
static QUALITY_LEVEL_ENUM_DESC: std::sync::LazyLock<EnumDescModel> = std::sync::LazyLock::new(|| { let mut d = EnumDescModel::default(); crate::generated_138::stub_8e24(&mut d); d });
static ANTIALIASING_MODE_ENUM_DESC: std::sync::LazyLock<EnumDescModel> = std::sync::LazyLock::new(|| { let mut d = EnumDescModel::default(); crate::generated_138::stub_8a88(&mut d); d });
static FRAME_RATE_MANAGER_ENUM_DESC: std::sync::LazyLock<EnumDescModel> = std::sync::LazyLock::new(|| { let mut d = EnumDescModel::default(); crate::generated_138::stub_88c4(&mut d); d });
static GRAPHICS_MODE_ENUM_DESC: std::sync::LazyLock<EnumDescModel> = std::sync::LazyLock::new(|| { let mut d = EnumDescModel::default(); crate::generated_138::stub_86d0(&mut d); d });
static AA_SAMPLES_ENUM_DESC: std::sync::LazyLock<EnumDescModel> = std::sync::LazyLock::new(|| { let mut d = EnumDescModel::default(); crate::generated_138::stub_850c(&mut d); d });
// 0x16548 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::initSingleton(void)
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::initSingleton(void)")]
pub fn stub_16548() -> &'static EnumDescModel {
    // IDA 0x16548: Singleton<ShadowMode>::initSingleton — thunk tail-calling doGetSingleton (decompile: single shim call); LazyLock guard lives in the doGet port.
    stub_1654c()}

// 0x1654c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)")]
pub fn stub_1654c() -> &'static EnumDescModel {
    // IDA 0x1654c: Singleton<ShadowMode>::doGetSingleton — guarded EnumDesc C2 (cf. 0x1654c: 0x165c2), atexit dtor (0x165e0), release (0x165e6), returns &s (0x16610). LazyLock is the guard; the C2 port populates.
    &SHADOW_MODE_ENUM_DESC}

// 0x1663c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::initSingleton(void)
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::initSingleton(void)")]
pub fn stub_1663c() -> &'static EnumDescModel {
    // IDA 0x1663c: Singleton<ResolutionPreset>::initSingleton — thunk tail-calling doGetSingleton (decompile: single shim call); LazyLock guard lives in the doGet port.
    stub_16640()}

// 0x16640 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)")]
pub fn stub_16640() -> &'static EnumDescModel {
    // IDA 0x16640: Singleton<ResolutionPreset>::doGetSingleton — guarded EnumDesc C2 (cf. 0x1654c: 0x165c2), atexit dtor (0x165e0), release (0x165e6), returns &s (0x16610). LazyLock is the guard; the C2 port populates.
    &RESOLUTION_PRESET_ENUM_DESC}

// 0x16730 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::initSingleton(void)
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::initSingleton(void)")]
pub fn stub_16730() -> &'static EnumDescModel {
    // IDA 0x16730: Singleton<QualityLevel>::initSingleton — thunk tail-calling doGetSingleton (decompile: single shim call); LazyLock guard lives in the doGet port.
    stub_16734()}

// 0x16734 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)")]
pub fn stub_16734() -> &'static EnumDescModel {
    // IDA 0x16734: Singleton<QualityLevel>::doGetSingleton — guarded EnumDesc C2 (cf. 0x1654c: 0x165c2), atexit dtor (0x165e0), release (0x165e6), returns &s (0x16610). LazyLock is the guard; the C2 port populates.
    &QUALITY_LEVEL_ENUM_DESC}

// 0x16824 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::initSingleton(void)
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::initSingleton(void)")]
pub fn stub_16824() -> &'static EnumDescModel {
    // IDA 0x16824: Singleton<AntialiasingMode>::initSingleton — thunk tail-calling doGetSingleton (decompile: single shim call); LazyLock guard lives in the doGet port.
    stub_16828()}

// 0x16828 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)")]
pub fn stub_16828() -> &'static EnumDescModel {
    // IDA 0x16828: Singleton<AntialiasingMode>::doGetSingleton — guarded EnumDesc C2 (cf. 0x1654c: 0x165c2), atexit dtor (0x165e0), release (0x165e6), returns &s (0x16610). LazyLock is the guard; the C2 port populates.
    &ANTIALIASING_MODE_ENUM_DESC}

// 0x16918 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::initSingleton(void)
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::initSingleton(void)")]
pub fn stub_16918() -> &'static EnumDescModel {
    // IDA 0x16918: Singleton<FrameRateManagerMode>::initSingleton — thunk tail-calling doGetSingleton (decompile: single shim call); LazyLock guard lives in the doGet port.
    stub_1691c()}

// 0x1691c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)")]
pub fn stub_1691c() -> &'static EnumDescModel {
    // IDA 0x1691c: Singleton<FrameRateManagerMode>::doGetSingleton — guarded EnumDesc C2 (cf. 0x1654c: 0x165c2), atexit dtor (0x165e0), release (0x165e6), returns &s (0x16610). LazyLock is the guard; the C2 port populates.
    &FRAME_RATE_MANAGER_ENUM_DESC}

// 0x16a0c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::initSingleton(void)
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::initSingleton(void)")]
pub fn stub_16a0c() -> &'static EnumDescModel {
    // IDA 0x16a0c: Singleton<GraphicsMode>::initSingleton — thunk tail-calling doGetSingleton (decompile: single shim call); LazyLock guard lives in the doGet port.
    stub_16a10()}

// 0x16a10 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)")]
pub fn stub_16a10() -> &'static EnumDescModel {
    // IDA 0x16a10: Singleton<GraphicsMode>::doGetSingleton — guarded EnumDesc C2 (cf. 0x1654c: 0x165c2), atexit dtor (0x165e0), release (0x165e6), returns &s (0x16610). LazyLock is the guard; the C2 port populates.
    &GRAPHICS_MODE_ENUM_DESC}

// 0x16b00 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE13initSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::initSingleton(void)
// type: 
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::initSingleton(void)")]
pub fn stub_16b00() -> &'static EnumDescModel {
    // IDA 0x16b00: Singleton<AASamples>::initSingleton — thunk tail-calling doGetSingleton (decompile: single shim call); LazyLock guard lives in the doGet port.
    stub_16b04()}

// 0x16b04 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv
// demangled: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)")]
pub fn stub_16b04() -> &'static EnumDescModel {
    // IDA 0x16b04: Singleton<AASamples>::doGetSingleton — guarded EnumDesc C2 (cf. 0x1654c: 0x165c2), atexit dtor (0x165e0), release (0x165e6), returns &s (0x16610). LazyLock is the guard; the C2 port populates.
    &AA_SAMPLES_ENUM_DESC}

// 0x16bf4 — __ZN19CRenderSettingsItemD2Ev
// demangled: CRenderSettingsItem::~CRenderSettingsItem()
// type: void __fastcall(CRenderSettingsItem *__hidden this)
#[doc(alias = "CRenderSettingsItem::~CRenderSettingsItem()")]
pub fn stub_16bf4(item: &mut RenderSettingsItem) {
    // IDA 0x16bf4: CRenderSettingsItem D2 — vtable resets (0x16c28..0x16c42), signal disconnectAll (0x16c74), slot release (0x16c7a..0x16c82), member delete + string dtor (0x16c88..0x16c98), GASI teardown; drops and disconnects fold into Rust ownership.
    item.changed.disconnect_all();}

// 0x16d34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16ResolutionPresetEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ResolutionPreset>> *)")]
pub fn stub_16d34(map: &mut std::collections::HashMap<String, i32>) {
    // IDA 0x16d34: _Rb_tree<ResolutionPreset>::_M_erase — recursive post-order node delete (cf. 0x16d34: 0x16d46..0x16d4e); the node heap folds into the HashMap.
    map.clear();}

// 0x16d5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12QualityLevelEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::QualityLevel>> *)")]
pub fn stub_16d5c(map: &mut std::collections::HashMap<String, i32>) {
    // IDA 0x16d5c: _Rb_tree<QualityLevel>::_M_erase — recursive post-order node delete (cf. 0x16d34: 0x16d46..0x16d4e); the node heap folds into the HashMap.
    map.clear();}

// 0x16d84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)")]
pub fn stub_16d84(map: &mut std::collections::HashMap<String, i32>) {
    // IDA 0x16d84: _Rb_tree<ShadowMode>::_M_erase — recursive post-order node delete (cf. 0x16d34: 0x16d46..0x16d4e); the node heap folds into the HashMap.
    map.clear();}

// 0x16dac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)")]
pub fn stub_16dac(map: &mut std::collections::HashMap<String, i32>) {
    // IDA 0x16dac: _Rb_tree<AntialiasingMode>::_M_erase — recursive post-order node delete (cf. 0x16d34: 0x16d46..0x16d4e); the node heap folds into the HashMap.
    map.clear();}

// 0x16dd4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)")]
pub fn stub_16dd4(map: &mut std::collections::HashMap<String, i32>) {
    // IDA 0x16dd4: _Rb_tree<FrameRateManagerMode>::_M_erase — recursive post-order node delete (cf. 0x16d34: 0x16d46..0x16d4e); the node heap folds into the HashMap.
    map.clear();}

// 0x16dfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)")]
pub fn stub_16dfc(map: &mut std::collections::HashMap<String, i32>) {
    // IDA 0x16dfc: _Rb_tree<GraphicsMode>::_M_erase — recursive post-order node delete (cf. 0x16d34: 0x16d46..0x16d4e); the node heap folds into the HashMap.
    map.clear();}

// 0x16e24 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)")]
pub fn stub_16e24(map: &mut std::collections::HashMap<String, i32>) {
    // IDA 0x16e24: _Rb_tree<AASamples>::_M_erase — recursive post-order node delete (cf. 0x16d34: 0x16d46..0x16d4e); the node heap folds into the HashMap.
    map.clear();}

// 0x16e4c — __GLOBAL__I_a
// demangled: global constructor keyed to_a
// type: 
#[doc(alias = "global constructor keyed to_a")]
pub fn stub_16e4c() {
    // IDA 0x16e4c: __GLOBAL__I_a — boost::system category static-inits (disasm: generic_category/system_category stores into MergedGlobals); was: boost::system -> std::io error categories, no host state — static-init no-op shell.
}

// 0x179e8 — __ZN3RBX9DataModel10serverSaveEv
// demangled: RBX::DataModel::serverSave(void)
// type: void __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::serverSave(void)")]
pub fn stub_179e8() {
    // IDA 0x179e8: DataModel::serverSave — empty body (decompile: single `;`); save pipeline lives in the datamodel crate — faithful no-op shell.
}

// 0x179ec — __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
// demangled: RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)
// type: void()
#[doc(alias = "RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)")]
pub fn stub_179ec() {
    // IDA 0x179ec: DataModel::internalSaveAsync — empty body (decompile: single `;`); save pipeline lives in the datamodel crate — faithful no-op shell.
}

// 0x179f0 — __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
// demangled: RBX::DataModel::internalSave(RBX::ContentId)
// type: void()
#[doc(alias = "RBX::DataModel::internalSave(RBX::ContentId)")]
pub fn stub_179f0() {
    // IDA 0x179f0: DataModel::internalSave — empty body (decompile: single `;`); save pipeline lives in the datamodel crate — faithful no-op shell.
}

// 0x179f4 — __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
// demangled: RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
// type: void __fastcall(int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_179f4() -> ! {
    todo!("0x179f4 RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x17aac — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
// demangled: boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)
// type: 
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
pub fn stub_17aac() -> ! {
    todo!("0x17aac boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")
}

// 0x17b80 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
// demangled: boost::shared_ptr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)
// type: 
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")]
pub fn stub_17b80() -> ! {
    todo!("0x17b80 boost::shared_ptr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")
}

// 0x17c58 — __GLOBAL__I_a_0
// demangled: global constructor keyed to_a_0
// type: 
#[doc(alias = "global constructor keyed to_a_0")]
pub fn stub_17c58() {
    // IDA 0x17c58: __GLOBAL__I_a — boost::system category static-inits (disasm: generic_category/system_category stores into MergedGlobals); was: boost::system -> std::io error categories, no host state — static-init no-op shell.
}

// 0x17df0 — +[Appirater setAppId:]
// demangled: +[Appirater setAppId:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[Appirater setAppId:]")]
pub fn stub_17df0() -> ! {
    todo!("0x17df0 +[Appirater setAppId:]")
}

// 0x17e00 — +[Appirater setDaysUntilPrompt:]
// demangled: +[Appirater setDaysUntilPrompt:]
// type: void __cdecl(id, SEL, double)
#[doc(alias = "+[Appirater setDaysUntilPrompt:]")]
pub fn stub_17e00() -> ! {
    todo!("0x17e00 +[Appirater setDaysUntilPrompt:]")
}

// 0x17e14 — +[Appirater setUsesUntilPrompt:]
// demangled: +[Appirater setUsesUntilPrompt:]
// type: void __cdecl(id, SEL, int)
#[doc(alias = "+[Appirater setUsesUntilPrompt:]")]
pub fn stub_17e14() -> ! {
    todo!("0x17e14 +[Appirater setUsesUntilPrompt:]")
}

// 0x17e24 — +[Appirater setSignificantEventsUntilPrompt:]
// demangled: +[Appirater setSignificantEventsUntilPrompt:]
// type: void __cdecl(id, SEL, int)
#[doc(alias = "+[Appirater setSignificantEventsUntilPrompt:]")]
pub fn stub_17e24() -> ! {
    todo!("0x17e24 +[Appirater setSignificantEventsUntilPrompt:]")
}

// 0x17e34 — +[Appirater setTimeBeforeReminding:]
// demangled: +[Appirater setTimeBeforeReminding:]
// type: void __cdecl(id, SEL, double)
#[doc(alias = "+[Appirater setTimeBeforeReminding:]")]
pub fn stub_17e34() -> ! {
    todo!("0x17e34 +[Appirater setTimeBeforeReminding:]")
}

// 0x17e48 — +[Appirater setDebug:]
// demangled: +[Appirater setDebug:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater setDebug:]")]
pub fn stub_17e48() -> ! {
    todo!("0x17e48 +[Appirater setDebug:]")
}

// 0x17e58 — +[Appirater setDelegate:]
// demangled: +[Appirater setDelegate:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[Appirater setDelegate:]")]
pub fn stub_17e58() -> ! {
    todo!("0x17e58 +[Appirater setDelegate:]")
}

// 0x17f80 — +[Appirater sharedInstance]
// demangled: +[Appirater sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[Appirater sharedInstance]")]
pub fn stub_17f80() -> ! {
    todo!("0x17f80 +[Appirater sharedInstance]")
}

// 0x17fe4 — ___27+[Appirater sharedInstance]_block_invoke
// demangled: ___27+[Appirater sharedInstance]_block_invoke
// type: 
#[doc(alias = "___27+[Appirater sharedInstance]_block_invoke")]
pub fn stub_17fe4() -> ! {
    todo!("0x17fe4 ___27+[Appirater sharedInstance]_block_invoke")
}

// 0x18094 — ___copy_helper_block_
// demangled: ___copy_helper_block_
// type: 
#[doc(alias = "___copy_helper_block_")]
pub fn stub_18094() {
    // IDA 0x18094: __copy_helper_block — _Block_object_assign shim over the captured object slot (cf. 0x18094: 0x1809a; 0x18bc8 uses +0x14, flag 3 = block retain); ObjC block ref traffic has no host carrier — faithful no-op shell.
}

// 0x180a0 — ___destroy_helper_block_
// demangled: ___destroy_helper_block_
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_")]
pub fn stub_180a0() {
    // IDA 0x180a0: __destroy_helper_block — _Block_object_dispose shim over the captured slot (cf. 0x180a0: 0x180a4); block release has no host carrier — faithful no-op shell.
}

// 0x180a8 — -[Appirater showRatingAlert]
// demangled: -[Appirater showRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater showRatingAlert]")]
pub fn stub_180a8() -> ! {
    todo!("0x180a8 -[Appirater showRatingAlert]")
}

// 0x183d8 — -[Appirater ratingConditionsHaveBeenMet]
// demangled: -[Appirater ratingConditionsHaveBeenMet]
// type: char __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater ratingConditionsHaveBeenMet]")]
pub fn stub_183d8() -> ! {
    todo!("0x183d8 -[Appirater ratingConditionsHaveBeenMet]")
}

// 0x185b0 — -[Appirater incrementUseCount]
// demangled: -[Appirater incrementUseCount]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater incrementUseCount]")]
pub fn stub_185b0() -> ! {
    todo!("0x185b0 -[Appirater incrementUseCount]")
}

// 0x18878 — -[Appirater incrementSignificantEventCount]
// demangled: -[Appirater incrementSignificantEventCount]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater incrementSignificantEventCount]")]
pub fn stub_18878() -> ! {
    todo!("0x18878 -[Appirater incrementSignificantEventCount]")
}

// 0x18b18 — -[Appirater incrementAndRate:]
// demangled: -[Appirater incrementAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
#[doc(alias = "-[Appirater incrementAndRate:]")]
pub fn stub_18b18() -> ! {
    todo!("0x18b18 -[Appirater incrementAndRate:]")
}

// 0x18bb4 — ___30-[Appirater incrementAndRate:]_block_invoke
// demangled: ___30-[Appirater incrementAndRate:]_block_invoke
// type: 
#[doc(alias = "___30-[Appirater incrementAndRate:]_block_invoke")]
pub fn stub_18bb4() -> ! {
    todo!("0x18bb4 ___30-[Appirater incrementAndRate:]_block_invoke")
}

// 0x18bc8 — ___copy_helper_block_125
// demangled: ___copy_helper_block_125
// type: 
#[doc(alias = "___copy_helper_block_125")]
pub fn stub_18bc8() {
    // IDA 0x18bc8: __copy_helper_block — _Block_object_assign shim over the captured object slot (cf. 0x18094: 0x1809a; 0x18bc8 uses +0x14, flag 3 = block retain); ObjC block ref traffic has no host carrier — faithful no-op shell.
}

// 0x18bd4 — ___destroy_helper_block_126
// demangled: ___destroy_helper_block_126
// type: 
#[doc(alias = "___destroy_helper_block_126")]
pub fn stub_18bd4() {
    // IDA 0x18bd4: __destroy_helper_block — _Block_object_dispose shim over the captured slot (cf. 0x180a0: 0x18bd4); block release has no host carrier — faithful no-op shell.
}

// 0x18bdc — -[Appirater incrementSignificantEventAndRate:]
// demangled: -[Appirater incrementSignificantEventAndRate:]
// type: void __cdecl(Appirater *self, SEL, char)
#[doc(alias = "-[Appirater incrementSignificantEventAndRate:]")]
pub fn stub_18bdc() -> ! {
    todo!("0x18bdc -[Appirater incrementSignificantEventAndRate:]")
}

// 0x18c78 — ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
// demangled: ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke
// type: 
#[doc(alias = "___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")]
pub fn stub_18c78() -> ! {
    todo!("0x18c78 ___46-[Appirater incrementSignificantEventAndRate:]_block_invoke")
}

// 0x18c8c — ___copy_helper_block_130
// demangled: ___copy_helper_block_130
// type: 
#[doc(alias = "___copy_helper_block_130")]
pub fn stub_18c8c() {
    // IDA 0x18c8c: __copy_helper_block_130 — same assign-shim template (disasm 0x18c8c..0x18c92: +0x14 slot, flag 3); no host carrier — faithful no-op shell.
}

// 0x18c98 — ___destroy_helper_block_131
// demangled: ___destroy_helper_block_131
// type: 
#[doc(alias = "___destroy_helper_block_131")]
pub fn stub_18c98() {
    // IDA 0x18c98: __destroy_helper_block — _Block_object_dispose shim over the captured slot (cf. 0x180a0: 0x18c98); block release has no host carrier — faithful no-op shell.
}

// 0x18ca0 — +[Appirater appLaunched]
// demangled: +[Appirater appLaunched]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater appLaunched]")]
pub fn stub_18ca0() -> ! {
    todo!("0x18ca0 +[Appirater appLaunched]")
}

// 0x18cc0 — +[Appirater appLaunched:]
// demangled: +[Appirater appLaunched:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater appLaunched:]")]
pub fn stub_18cc0() -> ! {
    todo!("0x18cc0 +[Appirater appLaunched:]")
}

// 0x18d10 — ___25+[Appirater appLaunched:]_block_invoke
// demangled: ___25+[Appirater appLaunched:]_block_invoke
// type: 
#[doc(alias = "___25+[Appirater appLaunched:]_block_invoke")]
pub fn stub_18d10() -> ! {
    todo!("0x18d10 ___25+[Appirater appLaunched:]_block_invoke")
}

// 0x18d4c — -[Appirater hideRatingAlert]
// demangled: -[Appirater hideRatingAlert]
// type: void __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater hideRatingAlert]")]
pub fn stub_18d4c() -> ! {
    todo!("0x18d4c -[Appirater hideRatingAlert]")
}

// 0x18dbc — +[Appirater appWillResignActive]
// demangled: +[Appirater appWillResignActive]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater appWillResignActive]")]
pub fn stub_18dbc() -> ! {
    todo!("0x18dbc +[Appirater appWillResignActive]")
}

// 0x18e0c — +[Appirater appEnteredForeground:]
// demangled: +[Appirater appEnteredForeground:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater appEnteredForeground:]")]
pub fn stub_18e0c() -> ! {
    todo!("0x18e0c +[Appirater appEnteredForeground:]")
}

// 0x18e5c — ___34+[Appirater appEnteredForeground:]_block_invoke
// demangled: ___34+[Appirater appEnteredForeground:]_block_invoke
// type: 
#[doc(alias = "___34+[Appirater appEnteredForeground:]_block_invoke")]
pub fn stub_18e5c() -> ! {
    todo!("0x18e5c ___34+[Appirater appEnteredForeground:]_block_invoke")
}

// 0x18e98 — +[Appirater userDidSignificantEvent:]
// demangled: +[Appirater userDidSignificantEvent:]
// type: void __cdecl(id, SEL, char)
#[doc(alias = "+[Appirater userDidSignificantEvent:]")]
pub fn stub_18e98() -> ! {
    todo!("0x18e98 +[Appirater userDidSignificantEvent:]")
}

// 0x18ee8 — ___37+[Appirater userDidSignificantEvent:]_block_invoke
// demangled: ___37+[Appirater userDidSignificantEvent:]_block_invoke
// type: 
#[doc(alias = "___37+[Appirater userDidSignificantEvent:]_block_invoke")]
pub fn stub_18ee8() -> ! {
    todo!("0x18ee8 ___37+[Appirater userDidSignificantEvent:]_block_invoke")
}

// 0x18f24 — +[Appirater rateApp]
// demangled: +[Appirater rateApp]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Appirater rateApp]")]
pub fn stub_18f24() -> ! {
    todo!("0x18f24 +[Appirater rateApp]")
}

// 0x19028 — -[Appirater alertView:clickedButtonAtIndex:]
// demangled: -[Appirater alertView:clickedButtonAtIndex:]
// type: void __cdecl(Appirater *self, SEL, id, int)
#[doc(alias = "-[Appirater alertView:clickedButtonAtIndex:]")]
pub fn stub_19028() -> ! {
    todo!("0x19028 -[Appirater alertView:clickedButtonAtIndex:]")
}
