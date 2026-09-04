//! network generated_bg_6 — RakNet + RBX::Network + Replicator (auto-generated, do not edit manually)
//! Global gap filler bg_6 100 stubs 0x264280..0x2684e4 EA-sorted asc next 100 after 0x62718 (RakNet|Network|Replicat|Socket|Upnp|HTTP 6232/6232 complete, 25999->26099 network distinct, rbx_core::SharedPtr not boost) [skeleton batch]

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::collections::{BTreeMap, HashMap};
use std::sync::{Mutex, OnceLock};

use rbx_core::SharedPtr;

/// `boost::unordered` bucket-count policy (IDA 0x264280..0x264300,
/// 0x264540..0x26459c): first prime with `prime >= floor(size / max_load) + 1`.
/// Prime list mirrors `boost::unordered::detail::prime_list_template<unsigned long>::value`.
pub const HASH_PRIMES: &[u32] = &[
    3, 7, 11, 13, 17, 19, 23, 29, 37, 47, 59, 71, 89, 107, 131, 163, 197, 239, 293, 353, 431,
    521, 631, 761, 919, 1103, 1327, 1597, 1931, 2333, 2801, 3371, 4049, 4861, 5839, 7013, 8419,
    10103, 12143, 14591, 17519, 21023, 25229, 30293, 36353, 43627, 52361, 62851, 75431, 90523,
    108631, 130363, 156437, 187751, 225307, 270371, 324449, 389357, 467237, 560689, 672827,
    807403, 968897, 1162687, 1395263, 1674319, 2009191, 2411033, 2893249, 3471899, 4166287,
    4999559, 5999471, 7199369,
];

/// `table::min_buckets_for_size` (IDA 0x264280): `floor(size / max_load) + 1`
/// rounded up to the next prime; saturates at `u32::MAX` like the
/// `v2 < 4294967300.0` guard (0x2642b4).
pub fn hash_min_buckets_for_size(size: usize, max_load: f32) -> usize {
    let need = ((size as f64 / max_load.max(f32::MIN_POSITIVE) as f64).floor() as u64)
        .saturating_add(1)
        .min(u64::from(u32::MAX));
    HASH_PRIMES.iter().find(|p| u64::from(**p) >= need).copied().unwrap_or(u32::MAX) as usize
}

/// `boost::unordered::detail::table` over `char const*` keys
/// (FunctionDescriptor / PropertyDescriptor maps).
/// Replaces bucket storage with `HashMap`; `// was: boost::unordered_map<...>`.
#[derive(Debug, Default)]
pub struct DescriptorTable {
    pub table: HashMap<String, usize>,
    pub max_load: f32,
}

/// `boost::unordered::detail::table` over `type_info const*` keys
/// (EnumDescriptor type lookup). Pointer keys stay engine-side as `usize`.
#[derive(Debug, Default)]
pub struct TypeDescriptorTable {
    pub table: HashMap<usize, usize>,
    pub max_load: f32,
}

/// `std::map<Name const*, EnumDescriptor const*>` mirror.
/// `// was: std::map<...>`.
#[derive(Debug, Default)]
pub struct NameEnumTable {
    pub table: BTreeMap<usize, usize>,
}

/// `std::vector<T*>` mirror for descriptor lists. `// was: std::vector<...>`.
// (Uses the Vec directly at call sites; kept for documentation.)
pub type DescriptorVec = Vec<usize>;

fn hash_reserve_for_insert<K, V>(map: &mut HashMap<K, V>, additional: usize)
where
    K: std::hash::Hash + Eq,
{
    // IDA 0x2669c8: grow target is max(need, len + len/2) (0x2669e2..0x2669e4),
    // then min_buckets_for_size + rehash when the bucket count moves.
    let need = map.len().saturating_add(additional);
    if map.capacity() < need {
        let grown = map.len().saturating_add(map.len() / 2).max(need);
        map.reserve(grown.saturating_sub(map.len()));
    }
}

/// `boost::exception_detail` sync errors.
/// Maps `boost::lock_error` / `thread_resource_error` / `bad_alloc_` /
/// `bad_exception_` (IDA 0x2650b8..0x2665f8). `// was: boost::exception`.
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum BoostSyncError {
    #[error("boost::lock_error: {0}")]
    Lock(String),
    #[error("boost::thread_resource_error: {0}")]
    ThreadResource(String),
    #[error("boost::bad_alloc")]
    BadAlloc,
    #[error("boost::bad_exception")]
    BadException,
}

fn enum_name_table() -> &'static Mutex<BTreeMap<usize, usize>> {
    // IDA 0x2659dc: guard-once std::map init (0x2659f8..0x265a34).
    static CELL: OnceLock<Mutex<BTreeMap<usize, usize>>> = OnceLock::new();
    CELL.get_or_init(Mutex::default)
}

fn enum_type_table() -> &'static Mutex<HashMap<usize, usize>> {
    // IDA 0x265a40: guard-once table init (0x265a9e..0x265adc).
    static CELL: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();
    CELL.get_or_init(Mutex::default)
}

fn all_enums() -> &'static Mutex<Vec<usize>> {
    // IDA 0x265b34: guard-once vector init (0x265b50..0x265b80).
    static CELL: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
    CELL.get_or_init(Mutex::default)
}

fn all_types() -> &'static Mutex<Vec<usize>> {
    // IDA 0x267f44: guard-once vector init (0x267f66..0x267f9a).
    static CELL: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
    CELL.get_or_init(Mutex::default)
}

fn reflection_type_name(cell: &'static OnceLock<&'static str>, name: &'static str) -> &'static str {
    *cell.get_or_init(|| name)
}


// 0x264280 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_264280(size: usize, max_load: f32) -> usize {
    // IDA 0x264280: floor(size / max_load) + 1 up to the next prime (cf. 0x2642a4..0x264300).
    // was: boost::unordered::detail::table<FunctionDescriptor>::min_buckets_for_size.
    hash_min_buckets_for_size(size, max_load)
}


// 0x264310 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
pub fn stub_264310(map: &mut DescriptorTable, buckets: usize) {
    // IDA 0x264310: create_buckets + re-place every node (cf. 0x264316..0x264334).
    // was: boost::unordered::detail::table_impl<FunctionDescriptor>::rehash_impl.
    map.table.reserve(buckets.saturating_sub(map.table.len()));
}


// 0x26433c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_26433c(map: &mut DescriptorTable, key: &str, value: usize) -> Option<usize> {
    // IDA 0x26433c: hashes the key and links the node into its bucket (cf. 0x264350..0x26438a).
    // was: boost::unordered::detail::table_impl<FunctionDescriptor>::place_in_bucket.
    map.table.insert(key.to_owned(), value)
}


// 0x264394 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEEEEE9constructEv
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>>>::construct(void)")]
pub fn stub_264394(slot: &mut Option<(String, usize)>, key: &str) -> bool {
    // IDA 0x264394: allocates the node on first use, no-op when one is pending (cf. 0x26439a..0x2643c6).
    // was: boost::unordered::detail::node_constructor::construct.
    if slot.is_none() {
        *slot = Some((key.to_owned(), 0));
        return true;
    }
    false
}


// 0x2643cc — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD *, unsigned int, const char **)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
pub fn stub_2643cc(map: &DescriptorTable, key: &str) -> Option<usize> {
    // IDA 0x2643cc: bucket walk with hash + strcmp check (cf. 0x2643d8..0x26443a).
    // was: boost::unordered::detail::table_impl<FunctionDescriptor>::find_node_impl.
    map.table.get(key).copied()
}


// 0x26443c — __ZNSt6vectorIPN3RBX10Reflection18FunctionDescriptorESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::FunctionDescriptor *,std::allocator<RBX::Reflection::FunctionDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::FunctionDescriptor **,std::vector<RBX::Reflection::FunctionDescriptor *,std::allocator<RBX::Reflection::FunctionDescriptor *>>>,RBX::Reflection::FunctionDescriptor * const&)")]
pub fn stub_26443c(vec: &mut Vec<usize>, pos: usize, value: usize) {
    // IDA 0x26443c: realloc-doubling insert with memmove (cf. 0x26444a..0x2644cc).
    // was: std::vector<...>::_M_insert_aux.
    let pos = pos.min(vec.len());
    vec.insert(pos, value);
}


// 0x26451c — __ZNSt12_Vector_baseIPN3RBX10Reflection18FunctionDescriptorESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Reflection::FunctionDescriptor *,std::allocator<RBX::Reflection::FunctionDescriptor *>>::_M_allocate(unsigned long)")]
pub fn stub_26451c(capacity: usize) -> Vec<usize> {
    // IDA 0x26451c: raw allocate for capacity elements.
    // was: std::_Vector_base<...>::_M_allocate.
    Vec::with_capacity(capacity)
}


// 0x264534 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> const&)")]
pub fn stub_264534(capacity: usize) -> DescriptorTable {
    // IDA 0x264534: bucket count rounded to the prime list, size 0, max_load 1.0 (cf. 0x264540..0x26459c).
    // was: boost::unordered::detail::table<FunctionDescriptor>::table.
    DescriptorTable { table: HashMap::with_capacity(hash_min_buckets_for_size(capacity, 1.0)), max_load: 1.0 }
}


// 0x2645c8 — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18PropertyDescriptorEEESaIS5_EE9push_backERKS5_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *>>::push_back(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> * const&)")]
pub fn stub_2645c8(vec: &mut Vec<usize>, value: usize) {
    // IDA 0x2645c8: appends, growing via _M_insert_aux at capacity (cf. 0x2645ca..0x2645ee).
    // was: std::vector<...>::push_back.
    vec.push(value);
}


// 0x264608 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
pub fn stub_264608(map: &mut DescriptorTable) {
    // IDA 0x264608: destroys every node then frees the bucket array (cf. 0x26460e..0x26464e).
    // was: boost::unordered::detail::table<PropertyDescriptor>::delete_buckets.
    map.table.clear();
    map.table.shrink_to_fit();
}


// 0x264654 — __ZNSt6vectorIPN3RBX10Reflection25MemberDescriptorContainerINS1_18PropertyDescriptorEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> **,std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *>>>,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> * const&)")]
pub fn stub_264654(vec: &mut Vec<usize>, pos: usize, value: usize) {
    // IDA 0x264654: realloc-doubling insert with memmove (cf. 0x26444a..0x2644cc).
    // was: std::vector<...>::_M_insert_aux.
    let pos = pos.min(vec.len());
    vec.insert(pos, value);
}


// 0x264734 — __ZNSt12_Vector_baseIPN3RBX10Reflection25MemberDescriptorContainerINS1_18PropertyDescriptorEEESaIS5_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *>>::_M_allocate(unsigned long)")]
pub fn stub_264734(capacity: usize) -> Vec<usize> {
    // IDA 0x264734: raw allocate for capacity elements.
    // was: std::_Vector_base<...>::_M_allocate.
    Vec::with_capacity(capacity)
}


// 0x2648cc — __ZNSt6vectorIPN3RBX10Reflection18PropertyDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int *, _DWORD *, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::PropertyDescriptor *,std::allocator<RBX::Reflection::PropertyDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::PropertyDescriptor **,std::vector<RBX::Reflection::PropertyDescriptor *,std::allocator<RBX::Reflection::PropertyDescriptor *>>>,RBX::Reflection::PropertyDescriptor * const&)")]
pub fn stub_2648cc(vec: &mut Vec<usize>, pos: usize, value: usize) -> usize {
    // IDA 0x2648cc: fast push-back path at end with spare capacity else _M_insert_aux; returns the index (cf. 0x2648d2..0x264902).
    // was: std::vector<...>::insert.
    let pos = pos.min(vec.len());
    vec.insert(pos, value);
    pos
}


// 0x264aec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
// type: char **__fastcall(_DWORD *, char **, int, int, void *, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
pub fn stub_264aec<'a>(map: &'a mut DescriptorTable, key: &str) -> &'a mut usize {
    // IDA 0x264aec: find-or-construct the node for the key (cf. 0x264b0e..0x264b7e).
    // was: boost::unordered::detail::table_impl<PropertyDescriptor>::operator[].
    map.table.entry(key.to_owned()).or_insert(0)
}


// 0x264c70 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
pub fn stub_264c70(map: &mut DescriptorTable, buckets: usize) {
    // IDA 0x264c70: allocates buckets + 1 and recomputes the max-load size (cf. 0x264c9e..0x264d42).
    // was: boost::unordered::detail::table<PropertyDescriptor>::create_buckets.
    map.table.reserve(buckets.saturating_sub(map.table.len()));
}


// 0x264d98 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_264d98(size: usize, max_load: f32) -> usize {
    // IDA 0x264d98: floor(size / max_load) + 1 up to the next prime (cf. 0x2642a4..0x264300).
    // was: boost::unordered::detail::table<PropertyDescriptor>::min_buckets_for_size.
    hash_min_buckets_for_size(size, max_load)
}


// 0x264e28 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
pub fn stub_264e28(map: &mut DescriptorTable, buckets: usize) {
    // IDA 0x264e28: create_buckets + re-place every node (cf. 0x264316..0x264334).
    // was: boost::unordered::detail::table_impl<PropertyDescriptor>::rehash_impl.
    map.table.reserve(buckets.saturating_sub(map.table.len()));
}


// 0x264e54 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_264e54(map: &mut DescriptorTable, key: &str, value: usize) -> Option<usize> {
    // IDA 0x264e54: hashes the key and links the node into its bucket (cf. 0x264350..0x26438a).
    // was: boost::unordered::detail::table_impl<PropertyDescriptor>::place_in_bucket.
    map.table.insert(key.to_owned(), value)
}


// 0x264eac — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEEEEE9constructEv
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>>>::construct(void)")]
pub fn stub_264eac(slot: &mut Option<(String, usize)>, key: &str) -> bool {
    // IDA 0x264eac: allocates the node on first use, no-op when one is pending (cf. 0x26439a..0x2643c6).
    // was: boost::unordered::detail::node_constructor::construct.
    if slot.is_none() {
        *slot = Some((key.to_owned(), 0));
        return true;
    }
    false
}


// 0x264ee4 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD *, unsigned int, const char **)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
pub fn stub_264ee4(map: &DescriptorTable, key: &str) -> Option<usize> {
    // IDA 0x264ee4: bucket walk with hash + strcmp check (cf. 0x2643d8..0x26443a).
    // was: boost::unordered::detail::table_impl<PropertyDescriptor>::find_node_impl.
    map.table.get(key).copied()
}


// 0x264f54 — __ZNSt6vectorIPN3RBX10Reflection18PropertyDescriptorESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::PropertyDescriptor *,std::allocator<RBX::Reflection::PropertyDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::PropertyDescriptor **,std::vector<RBX::Reflection::PropertyDescriptor *,std::allocator<RBX::Reflection::PropertyDescriptor *>>>,RBX::Reflection::PropertyDescriptor * const&)")]
pub fn stub_264f54(vec: &mut Vec<usize>, pos: usize, value: usize) {
    // IDA 0x264f54: realloc-doubling insert with memmove (cf. 0x26444a..0x2644cc).
    // was: std::vector<...>::_M_insert_aux.
    let pos = pos.min(vec.len());
    vec.insert(pos, value);
}


// 0x265034 — __ZNSt12_Vector_baseIPN3RBX10Reflection18PropertyDescriptorESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Reflection::PropertyDescriptor *,std::allocator<RBX::Reflection::PropertyDescriptor *>>::_M_allocate(unsigned long)")]
pub fn stub_265034(capacity: usize) -> Vec<usize> {
    // IDA 0x265034: raw allocate for capacity elements.
    // was: std::_Vector_base<...>::_M_allocate.
    Vec::with_capacity(capacity)
}


// 0x26504c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> const&)")]
pub fn stub_26504c(capacity: usize) -> DescriptorTable {
    // IDA 0x26504c: bucket count rounded to the prime list, size 0, max_load 1.0 (cf. 0x264540..0x26459c).
    // was: boost::unordered::detail::table<PropertyDescriptor>::table.
    DescriptorTable { table: HashMap::with_capacity(hash_min_buckets_for_size(capacity, 1.0)), max_load: 1.0 }
}


// 0x2650b8 — __ZN5boost21thread_resource_errorD0Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::thread_resource_error::~thread_resource_error()")]
pub fn stub_2650b8(error: BoostSyncError, free: &mut dyn FnMut()) {
    // IDA 0x2650b8: vtable reset + message destroy + base destroy + delete (cf. 0x2650d0..0x2650e4).
    drop(error);
    free();
}


// 0x2650e8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE7rethrowEv
// type: void __fastcall __noreturn(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::rethrow(void)const")]
pub fn stub_2650e8(error: BoostSyncError) -> ! {
    // IDA 0x2650e8: allocates + copies the exception then __cxa_throw (cf. 0x265120..0x26526e); noreturn.
    // was: boost::exception_detail::clone_impl<...>::rethrow → C++ throw; Rust panic carries it out.
    panic!("stub_2650e8: {error:?}");
}


// 0x2652b0 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE7rethrowEv
// type: void __fastcall __noreturn(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::rethrow(void)const")]
pub fn stub_2652b0(error: BoostSyncError) -> ! {
    // IDA 0x2652b0: allocates + copies the exception then __cxa_throw (cf. 0x265120..0x26526e); noreturn.
    // was: boost::exception_detail::clone_impl<...>::rethrow → C++ throw; Rust panic carries it out.
    panic!("stub_2652b0: {error:?}");
}


// 0x2652c0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
pub fn stub_2652c0(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x2652c0: vtable-offset adjust then tail-call the primary dtor (cf. 0x2652ca..0x2652ce shape).
    destroy_at(this);
}


// 0x2652e0 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
pub fn stub_2652e0(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x2652e0: this-20 adjust then tail-call the primary dtor.
    destroy_at(this - 20);
}


// 0x2652f8 — __ZN5boost16exception_detail10clone_baseD1Ev
// type: void __fastcall(boost::exception_detail::clone_base *__hidden this)
#[doc(alias = "boost::exception_detail::clone_base::~clone_base()")]
pub fn stub_2652f8() {
    // IDA 0x2652f8: empty base dtor (0x2652f8).
    // was: boost::exception_detail::clone_base::~clone_base.
}


// 0x265300 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS5_NS5_9clone_tagE
// type: int __fastcall(int, int, int, int, std::exception *, std::string *, int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_tag)")]
pub fn stub_265300(error: &BoostSyncError) -> BoostSyncError {
    // IDA 0x265300: copies vtable, message strings and error-info (cf. 0x265346..0x265436).
    // was: boost::exception_detail::clone_impl<...>::clone_impl copy ctor.
    error.clone()
}


// 0x2654d8 — __ZN5boost16exception_detail14bad_exception_D2Ev
// type: void __fastcall(std::bad_exception *this)
#[doc(alias = "boost::exception_detail::bad_exception_::~bad_exception_()")]
pub fn stub_2654d8(error: BoostSyncError) {
    // IDA 0x2654d8: bad_exception destroy chain.
    drop(error);
}


// 0x265590 — __ZThn20_N5boost16exception_detail14bad_exception_D1Ev
// type: void __fastcall(boost::exception_detail::bad_exception_ *__hidden this)
#[doc(alias = "non-virtual thunk toboost::exception_detail::bad_exception_::~bad_exception_()")]
pub fn stub_265590(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x265590: this-20 adjust then tail-call the primary dtor.
    destroy_at(this - 20);
}


// 0x265598 — __ZThn20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED1Ev
// type: void __fastcall(int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()")]
pub fn stub_265598(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x265598: this-20 adjust then tail-call the primary dtor.
    destroy_at(this - 20);
}


// 0x2655a0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()")]
pub fn stub_2655a0(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x2655a0: vtable-offset adjust then tail-call the primary dtor (cf. 0x2652ca..0x2652ce shape).
    destroy_at(this);
}


// 0x2655b0 — __ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_14bad_exception_EEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> *)")]
pub fn stub_2655b0(error: BoostSyncError) -> SharedPtr<BoostSyncError> {
    // IDA 0x2655b0: operator new the counted impl with use/weak counts 1,1 (cf. 0x2655dc..0x265624).
    // was: boost::detail::shared_count<...> ctor → SharedPtr (Arc).
    SharedPtr::new(error)
}


// 0x2656a8 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>::~sp_counted_impl_p()")]
pub fn stub_2656a8(error: BoostSyncError) {
    // IDA 0x2656a8: counted-impl destroy (cf. operator delete at end).
    // was: boost::detail::sp_counted_impl_p<...>::~sp_counted_impl_p; Arc drop glue runs here.
    drop(error);
}


// 0x2656b0 — __ZN5boost16exception_detail10bad_alloc_D2Ev
// type: void __fastcall(std::bad_alloc *this)
#[doc(alias = "boost::exception_detail::bad_alloc_::~bad_alloc_()")]
pub fn stub_2656b0(error: BoostSyncError) {
    // IDA 0x2656b0: bad_alloc destroy chain.
    drop(error);
}


// 0x265768 — __ZThn20_N5boost16exception_detail10bad_alloc_D1Ev
// type: void __fastcall(boost::exception_detail::bad_alloc_ *__hidden this)
#[doc(alias = "non-virtual thunk toboost::exception_detail::bad_alloc_::~bad_alloc_()")]
pub fn stub_265768(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x265768: this-20 adjust then tail-call the primary dtor.
    destroy_at(this - 20);
}


// 0x265770 — __ZThn20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()")]
pub fn stub_265770(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x265770: this-20 adjust then tail-call the primary dtor.
    destroy_at(this - 20);
}


// 0x265778 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED1Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()")]
pub fn stub_265778(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x265778: vtable-offset adjust then tail-call the primary dtor (cf. 0x2652ca..0x2652ce shape).
    destroy_at(this);
}


// 0x265788 — __ZN5boost16exception_detail10bad_alloc_D0Ev
// type: void __fastcall(boost::exception_detail::bad_alloc_ *__hidden this)
#[doc(alias = "boost::exception_detail::bad_alloc_::~bad_alloc_()")]
pub fn stub_265788(error: BoostSyncError, free: &mut dyn FnMut()) {
    // IDA 0x265788: bad_alloc destroy chain + delete.
    drop(error);
    free();
}


// 0x2657a0 — __ZN3RBX10Reflection10DescriptorD1Ev
// type: void __fastcall(RBX::Reflection::Descriptor *__hidden this)
#[doc(alias = "RBX::Reflection::Descriptor::~Descriptor()")]
pub fn stub_2657a0(destroy: &mut dyn FnMut()) {
    // IDA 0x2657a0: reflection base destroy body.
    destroy();
}


// 0x2658d4 — __ZN3RBX10Reflection10hash_valueERKNS0_13ConstPropertyE
// type: int __fastcall(__int64 *)
#[doc(alias = "RBX::Reflection::hash_value(RBX::Reflection::ConstProperty const&)")]
pub fn stub_2658d4(value: i64) -> i32 {
    // IDA 0x2658d4: (v + (v>>3)) ^ (hi + (hi>>3) + ((v+(v>>3))<<6) + ((v+(v>>3))>>2) - 1640531527), 32-bit wrap (0x2658f4).
    let lo = value as u32;
    let hi = (value as u64 >> 32) as u32;
    let a64 = value.wrapping_add(value >> 3);
    let a32 = lo.wrapping_add(lo >> 3);
    let rhs = (u64::from(hi.wrapping_add(hi >> 3)))
        .wrapping_add(u64::from(a32.wrapping_shl(6)))
        .wrapping_add(u64::from(a32 >> 2))
        .wrapping_sub(1_640_531_527);
    (a64 ^ rhs as i64) as i32
}


// 0x2659dc — __ZN3RBX10Reflection14EnumDescriptor18allEnumsNameLookupEv
// type: void *__fastcall(RBX::Reflection::EnumDescriptor *this)
#[doc(alias = "RBX::Reflection::EnumDescriptor::allEnumsNameLookup(void)")]
pub fn stub_2659dc() -> &'static Mutex<BTreeMap<usize, usize>> {
    // IDA 0x2659dc: guard-once map init, returns the static (cf. 0x2659f8..0x265a3c).
    enum_name_table()
}


// 0x265a40 — __ZN3RBX10Reflection14EnumDescriptor18allEnumsTypeLookupEv
// type: void *__fastcall(RBX::Reflection::EnumDescriptor *this)
#[doc(alias = "RBX::Reflection::EnumDescriptor::allEnumsTypeLookup(void)")]
pub fn stub_265a40() -> &'static Mutex<HashMap<usize, usize>> {
    // IDA 0x265a40: guard-once table init, returns the static (cf. 0x265a9e..0x265b08).
    enum_type_table()
}


// 0x265b34 — __ZN3RBX10Reflection14EnumDescriptor8allEnumsEv
// type: int *__fastcall(RBX::Reflection::EnumDescriptor *this)
#[doc(alias = "RBX::Reflection::EnumDescriptor::allEnums(void)")]
pub fn stub_265b34() -> &'static Mutex<Vec<usize>> {
    // IDA 0x265b34: guard-once vector init, returns the static (cf. 0x265b50..0x265b88).
    all_enums()
}


// 0x265cd4 — __ZN3RBX10Reflection14EnumDescriptorD0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *__hidden this)
#[doc(alias = "RBX::Reflection::EnumDescriptor::~EnumDescriptor()")]
pub fn stub_265cd4(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x265cd4: full destroy then operator delete (cf. 0x265d24..0x265d2a).
    destroy();
    free();
}


// 0x265d74 — __ZN3RBX10Reflection14EnumDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *__hidden this)
#[doc(alias = "RBX::Reflection::EnumDescriptor::~EnumDescriptor()")]
pub fn stub_265d74(destroy: &mut dyn FnMut()) {
    // IDA 0x265d74: EnumDescriptor destroy body.
    destroy();
}


// 0x265d78 — __ZN3RBX10Reflection14EnumDescriptorD2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *__hidden this)
#[doc(alias = "RBX::Reflection::EnumDescriptor::~EnumDescriptor()")]
pub fn stub_265d78(destroy: &mut dyn FnMut()) {
    // IDA 0x265d78: EnumDescriptor destroy body.
    destroy();
}


// 0x266338 — __ZN3RBX10Reflection5TTypeIPKNS0_18PropertyDescriptorEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<RBX::Reflection::PropertyDescriptor const*>::~TType()")]
pub fn stub_266338() {
    // IDA 0x266338: TType vtable reset + base destroy; static-type teardown, no per-instance work.
    // was: RBX::Reflection::TType<...>::~TType.
}


// 0x26633c — __ZNSt3mapIPKN3RBX4NameEPKNS0_10Reflection14EnumDescriptorESt4lessIS3_ESaISt4pairIKS3_S7_EEED1Ev
// type: int __fastcall(int)
#[doc(alias = "std::map<RBX::Name const*,RBX::Reflection::EnumDescriptor const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::~map()")]
pub fn stub_26633c(map: &mut NameEnumTable) {
    // IDA 0x26633c: erases every node.
    // was: std::map<Name const*, EnumDescriptor const*>::~map.
    map.table.clear();
}


// 0x26634c — __ZN5boost9unordered13unordered_mapIPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorENS7_8TypeHashENS7_9TypeEqualESaISt4pairIKS4_S9_EEED1Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::unordered_map<std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual,std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>>::~unordered_map()")]
pub fn stub_26634c(map: &mut TypeDescriptorTable) {
    // IDA 0x26634c: destroys nodes + buckets (cf. 0x265a2e shape via __cxa_atexit).
    // was: boost::unordered::unordered_map<...>::~unordered_map.
    map.table.clear();
    map.table.shrink_to_fit();
}


// 0x26635c — __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptorESaIS4_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>::~vector()")]
pub fn stub_26635c(vec: &mut Vec<usize>) {
    // IDA 0x26635c: destroys elements + frees storage.
    // was: std::vector<...>::~vector.
    vec.clear();
    vec.shrink_to_fit();
}


// 0x266408 — __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptorESaIS4_EE9push_backERKS4_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>::push_back(RBX::Reflection::EnumDescriptor const* const&)")]
pub fn stub_266408(vec: &mut Vec<usize>, value: usize) {
    // IDA 0x266408: appends, growing via _M_insert_aux at capacity (cf. 0x2645ca..0x2645ee).
    // was: std::vector<...>::push_back.
    vec.push(value);
}


// 0x266434 — __ZNSt3mapIPKN3RBX4NameEPKNS0_10Reflection14EnumDescriptorESt4lessIS3_ESaISt4pairIKS3_S7_EEEixERSB_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::Reflection::EnumDescriptor const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::operator[](RBX::Name const* const&)")]
pub fn stub_266434(map: &mut NameEnumTable, key: usize) -> &mut usize {
    // IDA 0x266434: lower-bound walk then insert-unique on miss (cf. 0x26643e..0x266488).
    // was: std::map<Name const*, EnumDescriptor const*>::operator[].
    map.table.entry(key).or_insert(0)
}


// 0x266500 — __ZN3RBX10Reflection18PropertyDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::PropertyDescriptor *__hidden this)
#[doc(alias = "RBX::Reflection::PropertyDescriptor::~PropertyDescriptor()")]
pub fn stub_266500(destroy: &mut dyn FnMut()) {
    // IDA 0x266500: PropertyDescriptor destroy body.
    destroy();
}


// 0x266504 — __ZN3RBX10Reflection18PropertyDescriptorD0Ev
// type: void __fastcall(RBX::Reflection::PropertyDescriptor *__hidden this)
#[doc(alias = "RBX::Reflection::PropertyDescriptor::~PropertyDescriptor()")]
pub fn stub_266504(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x266504: PropertyDescriptor destroy + delete.
    destroy();
    free();
}


// 0x2665b8 — __ZN5boost10lock_errorD1Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "boost::lock_error::~lock_error()")]
pub fn stub_2665b8(error: BoostSyncError) {
    // IDA 0x2665b8: runtime_error-base destroy chain.
    drop(error);
}


// 0x2665e8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
pub fn stub_2665e8(error: BoostSyncError) {
    // IDA 0x2665e8: runtime_error-base destroy chain.
    drop(error);
}


// 0x2665f8 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
pub fn stub_2665f8(this: usize, destroy_at: &mut dyn FnMut(usize)) {
    // IDA 0x2665f8: this-20 adjust then tail-call the primary dtor.
    destroy_at(this - 20);
}


// 0x266748 — __ZNSt6vectorIPFbvESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::_M_insert_aux(__gnu_cxx::__normal_iterator<bool (**)(void),std::vector<bool (*)(void),std::allocator<bool (*)(void)>>>,bool (* const&)(void))")]
pub fn stub_266748(vec: &mut Vec<usize>, pos: usize, value: usize) {
    // IDA 0x266748: realloc-doubling insert with memmove (cf. 0x26444a..0x2644cc).
    // was: std::vector<...>::_M_insert_aux.
    let pos = pos.min(vec.len());
    vec.insert(pos, value);
}


// 0x266828 — __ZNSt12_Vector_baseIPFbvESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<bool (*)(void),std::allocator<bool (*)(void)>>::_M_allocate(unsigned long)")]
pub fn stub_266828(capacity: usize) -> Vec<usize> {
    // IDA 0x266828: raw allocate for capacity elements.
    // was: std::_Vector_base<...>::_M_allocate.
    Vec::with_capacity(capacity)
}


// 0x266840 — __ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int()
#[doc(alias = "boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_266840(free_blocks: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x266840: walks freelists releasing empty blocks (cf. 0x26687e..0x266946).
    // was: boost::pool/singleton_pool::release_memory; backing allocator stays engine-side.
    free_blocks()
}


// 0x266870 — __ZN5boost4poolINS_34default_user_allocator_malloc_freeEE14release_memoryEv
// type: int __fastcall(int)
#[doc(alias = "boost::pool<boost::default_user_allocator_malloc_free>::release_memory(void)")]
pub fn stub_266870(free_blocks: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x266870: walks freelists releasing empty blocks (cf. 0x26687e..0x266946).
    // was: boost::pool/singleton_pool::release_memory; backing allocator stays engine-side.
    free_blocks()
}


// 0x266960 — __ZN5boost25simple_segregated_storageImE9segregateEPvmmS2_
// type: char **__fastcall(char **, int, unsigned int, char *)
#[doc(alias = "boost::simple_segregated_storage<unsigned long>::segregate(void *,unsigned long,unsigned long,void *)")]
pub fn stub_266960(base: usize, chunk: usize, align: usize) -> usize {
    // IDA 0x266960: carves chunks into a freelist, returns the count (cf. 0x26696c..0x2669c0).
    // was: boost::simple_segregated_storage::segregate.
    if align == 0 {
        return 0;
    }
    chunk / align
}


// 0x2669c8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
pub fn stub_2669c8(map: &mut DescriptorTable, additional: usize) {
    // IDA 0x2669c8: grow-to max(need, len + len/2) then min_buckets + rehash when needed (cf. 0x2669ce..0x266a14).
    // was: boost::unordered::detail::table<PropertyDescriptor>::reserve_for_insert.
    hash_reserve_for_insert(&mut map.table, additional);
}


// 0x266a1c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPKN3RBX10Reflection14EnumDescriptorESt6vectorIS6_SaIS6_EEEEPS4_ET_SD_SD_RKT0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int *)
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**,std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**,std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>>,RBX::Reflection::EnumDescriptor*>(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**,std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**,std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>>,RBX::Reflection::EnumDescriptor* const&,std::random_access_iterator_tag)")]
pub fn stub_266a1c(vec: &[usize], value: usize) -> Option<usize> {
    // IDA 0x266a1c: 4-wide unrolled linear search (cf. 0x266a22..0x266aa6).
    // was: std::__find over vector<EnumDescriptor const*>.
    vec.iter().position(|v| *v == value)
}


// 0x266aac — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEEixERS8_
// type: _DWORD *__fastcall(_DWORD *, int *, int, int, void *, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::operator[](std::type_info const* const&)")]
pub fn stub_266aac(map: &mut TypeDescriptorTable, key: usize) -> &mut usize {
    // IDA 0x266aac: find-or-construct the node for the key (cf. 0x264b0e..0x264b7e).
    // was: boost::unordered::detail::table_impl<EnumDescriptor>::operator[].
    map.table.entry(key).or_insert(0)
}


// 0x266c30 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::reserve_for_insert(unsigned long)")]
pub fn stub_266c30(map: &mut DescriptorTable, additional: usize) {
    // IDA 0x266c30: grow-to max(need, len + len/2) then min_buckets + rehash when needed (cf. 0x2669ce..0x266a14).
    // was: boost::unordered::detail::table<EnumDescriptor>::reserve_for_insert.
    hash_reserve_for_insert(&mut map.table, additional);
}


// 0x266c80 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::create_buckets(unsigned long)")]
pub fn stub_266c80(map: &mut DescriptorTable, buckets: usize) {
    // IDA 0x266c80: allocates buckets + 1 and recomputes the max-load size (cf. 0x264c9e..0x264d42).
    // was: boost::unordered::detail::table<EnumDescriptor>::create_buckets.
    map.table.reserve(buckets.saturating_sub(map.table.len()));
}


// 0x266da8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_266da8(size: usize, max_load: f32) -> usize {
    // IDA 0x266da8: floor(size / max_load) + 1 up to the next prime (cf. 0x2642a4..0x264300).
    // was: boost::unordered::detail::table<EnumDescriptor>::min_buckets_for_size.
    hash_min_buckets_for_size(size, max_load)
}


// 0x266e38 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::rehash_impl(unsigned long)")]
pub fn stub_266e38(map: &mut DescriptorTable, buckets: usize) {
    // IDA 0x266e38: create_buckets + re-place every node (cf. 0x264316..0x264334).
    // was: boost::unordered::detail::table_impl<EnumDescriptor>::rehash_impl.
    map.table.reserve(buckets.saturating_sub(map.table.len()));
}


// 0x266e64 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE15place_in_bucketERNS1_5tableISI_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_266e64(map: &mut DescriptorTable, key: &str, value: usize) -> Option<usize> {
    // IDA 0x266e64: hashes the key and links the node into its bucket (cf. 0x264350..0x26438a).
    // was: boost::unordered::detail::table_impl<EnumDescriptor>::place_in_bucket.
    map.table.insert(key.to_owned(), value)
}


// 0x266ebc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEEEEE9constructEv
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>>>::construct(void)")]
pub fn stub_266ebc(slot: &mut Option<(String, usize)>, key: &str) -> bool {
    // IDA 0x266ebc: allocates the node on first use, no-op when one is pending (cf. 0x26439a..0x2643c6).
    // was: boost::unordered::detail::node_constructor::construct.
    if slot.is_none() {
        *slot = Some((key.to_owned(), 0));
        return true;
    }
    false
}


// 0x266ef4 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE14find_node_implIS7_SH_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD *, unsigned int, int)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::find_node_impl<std::type_info const*,RBX::Reflection::EnumDescriptor::TypeEqual>(unsigned long,std::type_info const* const&,RBX::Reflection::EnumDescriptor::TypeEqual const&)const")]
pub fn stub_266ef4(map: &TypeDescriptorTable, key: usize) -> Option<usize> {
    // IDA 0x266ef4: bucket walk with hash + equality check (cf. 0x2643d8..0x26443a).
    // was: boost::unordered::detail::table_impl<EnumDescriptor>::find_node_impl.
    map.table.get(&key).copied()
}


// 0x266f6c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_10Reflection14EnumDescriptorEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*> const&)")]
pub fn stub_266f6c(map: &mut NameEnumTable, key: usize, value: usize) -> bool {
    // IDA 0x266f6c: inserts unless the key exists (cf. 0x266480 shape).
    // was: std::_Rb_tree<...>::_M_insert_unique.
    if map.table.contains_key(&key) {
        return false;
    }
    map.table.insert(key, value);
    true
}


// 0x267020 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_10Reflection14EnumDescriptorEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*> const&)")]
pub fn stub_267020(map: &mut NameEnumTable, key: usize, value: usize) {
    // IDA 0x267020: node alloc + rebalance + size++ (cf. 0x267050..0x267076).
    // was: std::_Rb_tree<...>::_M_insert.
    map.table.insert(key, value);
}


// 0x267078 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_10Reflection14EnumDescriptorEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE16_M_insert_uniqueERKSA_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*> const&)")]
pub fn stub_267078(map: &mut NameEnumTable, key: usize, value: usize) -> bool {
    // IDA 0x267078: inserts unless the key exists (cf. 0x266480 shape).
    // was: std::_Rb_tree<...>::_M_insert_unique.
    if map.table.contains_key(&key) {
        return false;
    }
    map.table.insert(key, value);
    true
}


// 0x2670e0 — __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptorESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**,std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>>,RBX::Reflection::EnumDescriptor const* const&)")]
pub fn stub_2670e0(vec: &mut Vec<usize>, pos: usize, value: usize) {
    // IDA 0x2670e0: realloc-doubling insert with memmove (cf. 0x26444a..0x2644cc).
    // was: std::vector<...>::_M_insert_aux.
    let pos = pos.min(vec.len());
    vec.insert(pos, value);
}


// 0x2671c0 — __ZNSt12_Vector_baseIPKN3RBX10Reflection14EnumDescriptorESaIS4_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>::_M_allocate(unsigned long)")]
pub fn stub_2671c0(capacity: usize) -> Vec<usize> {
    // IDA 0x2671c0: raw allocate for capacity elements.
    // was: std::_Vector_base<...>::_M_allocate.
    Vec::with_capacity(capacity)
}


// 0x2671d8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::delete_buckets(void)")]
pub fn stub_2671d8(map: &mut DescriptorTable) {
    // IDA 0x2671d8: destroys every node then frees the bucket array (cf. 0x26460e..0x26464e).
    // was: boost::unordered::detail::table<EnumDescriptor>::delete_buckets.
    map.table.clear();
    map.table.shrink_to_fit();
}


// 0x267224 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKSt9type_infoPKN3RBX10Reflection14EnumDescriptorEEES7_SD_NSB_8TypeHashENSB_9TypeEqualEEEEC2EmRKSG_RKSH_RKSaINS1_8ptr_nodeISE_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::table(unsigned long,RBX::Reflection::EnumDescriptor::TypeHash const&,RBX::Reflection::EnumDescriptor::TypeEqual const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>> const&)")]
pub fn stub_267224(capacity: usize) -> TypeDescriptorTable {
    // IDA 0x267224: bucket count rounded to the prime list, size 0, max_load 1.0 (cf. 0x264540..0x26459c).
    // was: boost::unordered::detail::table<EnumDescriptor>::table.
    TypeDescriptorTable { table: HashMap::with_capacity(hash_min_buckets_for_size(capacity, 1.0)), max_load: 1.0 }
}


// 0x26733c — __ZN3RBX10Reflection5TTypeIPKNS0_18PropertyDescriptorEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TType<RBX::Reflection::PropertyDescriptor const*>::~TType()")]
pub fn stub_26733c(free: &mut dyn FnMut()) {
    // IDA 0x26733c: TType teardown then operator delete.
    // was: RBX::Reflection::TType<...>::~TType D0.
    free();
}


// 0x267340 — __ZN3RBX10Reflection4TypeD0Ev
// type: void __fastcall(RBX::Reflection::Type *__hidden this)
#[doc(alias = "RBX::Reflection::Type::~Type()")]
pub fn stub_267340(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x267340: reflection base destroy + delete.
    destroy();
    free();
}


// 0x267348 — __ZN3RBX10Reflection10DescriptorD0Ev
// type: void __fastcall(RBX::Reflection::Descriptor *__hidden this)
#[doc(alias = "RBX::Reflection::Descriptor::~Descriptor()")]
pub fn stub_267348(destroy: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x267348: reflection base destroy + delete.
    destroy();
    free();
}


// 0x267488 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_10Reflection14EnumDescriptorEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>> *)")]
pub fn stub_267488(map: &mut NameEnumTable, key: usize) -> bool {
    // IDA 0x267488: erases the subtree rooted at the node (cf. 0x26748a..0x2674aa).
    // was: std::_Rb_tree<...>::_M_erase.
    map.table.remove(&key).is_some()
}


// 0x2675e0 — __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEEERKS1_v
// type: int *()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(void)")]
pub fn stub_2675e0() -> &'static str {
    // IDA 0x2675e0: guard-once Type("Array") init (cf. 0x26763e..0x267698).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    reflection_type_name(&CELL, "Array")
}


// 0x2677ac — __ZN3RBX10Reflection7Variant7convertIN5boost10shared_ptrIKSt6vectorIS1_SaIS1_EEEEEERT_v
// type: int __fastcall(int)
#[doc(alias = "boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> & RBX::Reflection::Variant::convert<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(void)")]
pub fn stub_2677ac(payload: Option<SharedPtr<()>>) -> Option<SharedPtr<()>> {
    // IDA 0x2677ac: any_cast to shared_ptr<Array> then copy-construct (cf. 0x2677c8..0x2677d4); mismatch throws bad_cast in C++.
    // was: boost::shared_ptr<Array> retained copy.
    payload.clone()
}


// 0x26796c — __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEEERKS1_v
// type: int *()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(void)")]
pub fn stub_26796c() -> &'static str {
    // IDA 0x26796c: guard-once Type("Dictionary") init (cf. 0x2679ca..0x267a24).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    reflection_type_name(&CELL, "Dictionary")
}


// 0x267a50 — __ZN3RBX10Reflection7Variant7convertIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsS1_NS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS1_EEEEEEEERT_v
// type: int __fastcall(int)
#[doc(alias = "boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> & RBX::Reflection::Variant::convert<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(void)")]
pub fn stub_267a50(payload: Option<SharedPtr<()>>) -> Option<SharedPtr<()>> {
    // IDA 0x267a50: any_cast to shared_ptr<Dictionary> then copy-construct (cf. 0x2677c8..0x2677d4); mismatch throws bad_cast in C++.
    // was: boost::shared_ptr<Dictionary> retained copy.
    payload.clone()
}


// 0x267c30 — __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS6_EEEEEEERKS1_v
// type: int *()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(void)")]
pub fn stub_267c30() -> &'static str {
    // IDA 0x267c30: guard-once Type("Map") init (cf. 0x267c8e..0x267cea).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    reflection_type_name(&CELL, "Map")
}


// 0x267d18 — __ZN3RBX10Reflection7Variant7convertIN5boost10shared_ptrIKSt3mapISsS1_St4lessISsESaISt4pairIKSsS1_EEEEEEERT_v
// type: int __fastcall(int)
#[doc(alias = "boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> & RBX::Reflection::Variant::convert<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(void)")]
pub fn stub_267d18(payload: Option<SharedPtr<()>>) -> Option<SharedPtr<()>> {
    // IDA 0x267d18: any_cast to shared_ptr<Map> then copy-construct (cf. 0x2677c8..0x2677d4); mismatch throws bad_cast in C++.
    // was: boost::shared_ptr<Map> retained copy.
    payload.clone()
}


// 0x267e5c — __ZN3RBX10Reflection4Type12getSingletonIvEERKS1_v
// type: int *()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<void>(void)")]
pub fn stub_267e5c() -> &'static str {
    // IDA 0x267e5c: guard-once Type<void>("void") init (cf. 0x267eba..0x267f16).
    static CELL: OnceLock<&'static str> = OnceLock::new();
    reflection_type_name(&CELL, "void")
}


// 0x267f44 — __ZN3RBX10Reflection4Type13addToAllTypesEv
// type: int __fastcall(RBX::Reflection::Type *this)
#[doc(alias = "RBX::Reflection::Type::addToAllTypes(void)")]
pub fn stub_267f44(type_id: usize) {
    // IDA 0x267f44: guard-once vector init then push_back (cf. 0x267f66..0x267fae).
    all_types().lock().unwrap().push(type_id);
}


// 0x267fb0 — __ZN3RBX10Reflection4Type11getAllTypesEv
// type: int __fastcall(RBX::Reflection::Type *this)
#[doc(alias = "RBX::Reflection::Type::getAllTypes(void)")]
pub fn stub_267fb0() -> &'static Mutex<Vec<usize>> {
    // IDA 0x267fb0: returns the all-types vector (cf. 0x267fbc).
    all_types()
}


// 0x267fc0 — __ZN3RBX10Reflection19SignatureDescriptorC1Ev
// type: RBX::Reflection::SignatureDescriptor *__fastcall(RBX::Reflection::SignatureDescriptor *this)
#[doc(alias = "RBX::Reflection::SignatureDescriptor::SignatureDescriptor(void)")]
pub fn stub_267fc0(set_void_type: &mut dyn FnMut()) {
    // IDA 0x267fc0: return type = getSingleton<void>(), empty item list (cf. 0x267fc6..0x267fe8).
    set_void_type();
}


// 0x268484 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS5_EEEEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::~TType()")]
pub fn stub_268484() {
    // IDA 0x268484: TType vtable reset + base destroy; static-type teardown, no per-instance work.
    // was: RBX::Reflection::TType<...>::~TType.
}


// 0x268488 — __ZN3rbx8any_castIN5boost10shared_ptrIKNS1_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEENS5_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> * rbx::any_cast<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_268488(type_matches: bool, payload: usize) -> Option<usize> {
    // IDA 0x268488: returns payload + 1 on typeinfo match (cf. 0x26848a..0x2684bc), else the bad_cast path.
    type_matches.then_some(payload + 1)
}


// 0x2684e0 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsNS0_7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEEEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::~TType()")]
pub fn stub_2684e0() {
    // IDA 0x2684e0: TType vtable reset + base destroy; static-type teardown, no per-instance work.
    // was: RBX::Reflection::TType<...>::~TType.
}


// 0x2684e4 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::TType<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::~TType()")]
pub fn stub_2684e4() {
    // IDA 0x2684e4: TType vtable reset + base destroy; static-type teardown, no per-instance work.
    // was: RBX::Reflection::TType<...>::~TType.
}

