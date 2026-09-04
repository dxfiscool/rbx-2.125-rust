//! core shard ky — 26 IDA-grounded ports 0x77e930-0x77f5b0.
//! Continuation after ku (ku took 0x77a5ac-0x77e8e0): the Script-to-Debugger
//! unordered map bucket table (IDA 0x77e930-0x77ecd4 plus 0x77f214/0x77f244),
//! the shared_ptr<vector<shared_ptr<Instance>>> control-block family
//! (IDA 0x77ed18-0x77f210), and the DebuggerManager / DebuggerWatch
//! reflection descriptors (IDA 0x77f2b0-0x77f5b0).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)].
//! AGENTS.md section 4: shared/intrusive_ptr -> crate::SharedPtr (Arc),
//! weak_ptr -> Weak, unordered_map -> arena + bucket chains with the IDA
//! prime-list growth policy, bind/function -> closures (n/a here),
//! signals/slots -> crate::signal::Signal (n/a here), thread -> std (n/a).
//! Carriers in generated_core_shard_f.rs / generated_core_shard_jl.rs are
//! untouched; these ports live under new idiomatic names.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
use std::sync::Weak;

/// was: `map<RBX::Script const*, RBX::Scripting::ScriptDebugger*>` bucket
/// table — `boost::unordered::detail::{table, table_impl,
/// node_constructor}` over `pair<Script const* const, ScriptDebugger*>`
/// (IDA 0x77e930-0x77ecd4, 0x77f214, 0x77f244). Nodes live in an arena;
/// `buckets` holds head links plus the IDA extra tail slot at `buckets[size]`.
pub mod script_map_table {
    /// was: `RBX::Script const*` — map key (identity hash).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct ScriptKey(pub usize);

    /// was: `RBX::Scripting::ScriptDebugger*` — map value.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct DebuggerPtr(pub usize);

    /// was: `ptr_node<pair<Script const* const, ScriptDebugger*>>` — 16
    /// bytes on ARM (IDA 0x77eb8a `operator new(0x10)`): next link, stored
    /// hash, key, value.
    #[derive(Debug, Default, Clone, Copy)]
    pub struct MapNode {
        pub next: Option<usize>,
        pub hash: u32,
        pub key: usize,
        pub debugger: usize,
    }

    /// was: `table<map<...>>` — +4 bucket count, +8 live count, +12 max
    /// load factor (f32), +16 max count, +20 bucket array (IDA 0x77f244).
    #[derive(Debug, Default)]
    pub struct ScriptDebuggerTable {
        pub buckets: Option<Vec<Option<usize>>>,
        pub nodes: Vec<MapNode>,
        pub live: Vec<bool>,
        pub bucket_count: usize,
        pub size: usize,
        pub max_count: u32,
        pub mlf: f32,
    }

    /// was: `prime_list_template<unsigned long>::value` — the boost prime
    /// list the IDA binary searches at 0x77ea9e-0x77ead8 and 0x77f250-0x77f29a
    /// (v4 starts at 38; the end-clamp at 0x77ead2/0x77f29a against
    /// `unk_FA77F8` is mirrored by clamping to the last entry).
    pub const PRIME_LIST: [usize; 37] = [
        17, 29, 37, 53, 67, 79, 97, 131, 193, 257, 389, 521, 769, 1031, 1543,
        3079, 6151, 12289, 24593, 49157, 98317, 196613, 393241, 786433,
        1572869, 3145739, 6291469, 12582917, 25165843, 50331653, 100663319,
        201326611, 402653189, 805306457, 1610612741, 3221225473, 4294967291,
    ];

    /// First prime >= `need`; clamps to the last entry when `need` runs past
    /// the list (IDA 0x77ead2 `v5 == unk_FA77F8` → `v5 -= 4`).
    pub fn prime_at_least(need: u64) -> usize {
        let mut lo = 0usize;
        let mut hi = PRIME_LIST.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if (PRIME_LIST[mid] as u64) < need {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        PRIME_LIST[lo.min(PRIME_LIST.len() - 1)]
    }

    /// was: `node_constructor<allocator<ptr_node<...>>>` — +4 node slot,
    /// +8 occupied hook, +9 constructed flag (IDA 0x77eb6c).
    #[derive(Debug, Default)]
    pub struct NodeCtor {
        pub node: Option<MapNode>,
        pub occupied: bool,
        pub constructed: bool,
    }

    impl ScriptDebuggerTable {
        /// IDA hash placement: `stored_hash % bucket_count` (___umodsi3 at
        /// 0x77eb2c/0x77ec32/0x77ebbe); None when no buckets are allocated.
        pub fn bucket_of(&self, hash: u32) -> Option<usize> {
            if self.bucket_count == 0 {
                return None;
            }
            Some((hash as usize) % self.bucket_count)
        }

        fn alloc_node(&mut self, node: MapNode) -> usize {
            for (i, live) in self.live.iter_mut().enumerate() {
                if !*live {
                    *live = true;
                    self.nodes[i] = node;
                    return i;
                }
            }
            self.nodes.push(node);
            self.live.push(true);
            self.nodes.len() - 1
        }
    }
}

/// was: `vector<shared_ptr<Instance>>` plus the `sp_counted_impl_pd` control
/// block with an `rbx::detail::sp_ms_deleter` (IDA 0x77ed18-0x77f210).
/// `shared_ptr`/`intrusive_ptr` -> `crate::SharedPtr` (Arc),
/// `weak_ptr` -> `Weak`.
pub mod instance_vec_block {
    use super::*;

    /// was: `RBX::Instance` — opaque handle; identity word only.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct InstanceSlot(pub usize);

    /// was: `RBX::BaseScript` — opaque handle for the weak-lock port.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct BaseScriptSlot(pub usize);

    /// was: `std::vector<shared_ptr<RBX::Instance>>` — each element is 8
    /// bytes on ARM (px + pi), hence IDA 0x77eee0 `end = start + 8 * n`.
    pub type InstanceVec = Vec<SharedPtr<InstanceSlot>>;

    /// was: `std::_Vector_base<shared_ptr<Instance>>` — the begin/end/cap
    /// triple, collapsed (IDA 0x77eeb8 zeroes +0/+4/+8, then `_M_allocate`).
    #[derive(Debug, Default)]
    pub struct InstanceVecBase {
        pub storage: InstanceVec,
    }

    /// was: `rbx::detail::sp_ms_deleter<vector<shared_ptr<Instance>>>` —
    /// stateless deleter living at control-block +16 (IDA 0x77f212 returns
    /// `a1 + 16`).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct MsDeleter;

    /// was: `typeinfo for rbx::detail::sp_ms_deleter<...>` name probed at
    /// IDA 0x77f20a.
    pub const SP_MS_DELETER_TI_NAME: &str =
        "N3rbx6detail13sp_ms_deleterISt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS7_EEEE";

    /// was: `sp_counted_impl_pd<vector<...>* , sp_ms_deleter<...>>` — the
    /// 0x20-byte block from IDA 0x77f04a `operator new(0x20)`: vtable +0,
    /// use count +4 = 1, weak count +8 = 1, payload +12, owned flag +16 = 0
    /// (IDA 0x77f05e-0x77f06c).
    #[derive(Debug, Default)]
    pub struct CountedBlock {
        pub uses: u32,
        pub weaks: u32,
        pub vec: Option<InstanceVec>,
        pub deleter: MsDeleter,
        pub owned: bool,
    }

    /// was: `shared_ptr<vector<shared_ptr<Instance>>>` holder for the
    /// `get_deleter` port (IDA 0x77ed94 reads pi at +4).
    #[derive(Debug, Default, Clone)]
    pub struct SharedInstanceVec {
        pub ptr: Option<SharedPtr<InstanceVec>>,
    }
}

/// was: `Described<DebuggerManager>` / `BoundFuncDesc<DebuggerWatch>`
/// reflection glue (IDA 0x77f2b0-0x77f5b0). Dtor thunks delegate to the
/// `Instance` teardown; the func descriptor stores the `mf0` member-pointer
/// pair plus a void return-type token.
pub mod debugger_reflection {
    /// was: `Described<Scripting::DebuggerManager, ...>` instance side —
    /// teardown flag stands in for the `Instance` base subobject.
    #[derive(Debug, Default)]
    pub struct DebuggerManagerDesc {
        pub torn_down: bool,
    }

    /// was: `void (DebuggerWatch::*)(void)` member-pointer pair —
    /// `__PAIR64__(a3, a2)` at IDA 0x77f4ce, stored at +40 (IDA 0x77f51a).
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct MemberFnVoid {
        pub ptr: usize,
        pub adjust: usize,
    }

    /// was: `BoundFuncDesc<DebuggerWatch, void()(void), 0>` — vtable at +0,
    /// signature list at +8, member pair at +40, void return token at +28.
    #[derive(Debug, Default)]
    pub struct BoundFuncDescWatch {
        /// Image vtable token (`off_12A04A8` while live, `off_1222248`
        /// after the base reset at IDA 0x77f5ee).
        pub vtable: usize,
        /// was: `describedClassDescriptor` static chained at IDA 0x77f4f2.
        pub class_descriptor: usize,
        /// Signature items cleared by `_M_clear` at IDA 0x77f614.
        pub signatures: Vec<usize>,
        /// Member-function pair at +40 (IDA 0x77f51a).
        pub member_fn: MemberFnVoid,
        /// Bound name (char const* at IDA 0x77f4ac a4).
        pub name: String,
        /// was: `RBX::Security::Permissions`.
        pub permissions: u32,
        /// Set from `Type::getSingleton<void>()` at IDA 0x77f542.
        pub returns_void: bool,
    }

    /// Live vtable image token from IDA 0x77f50e (`off_12A04A8`).
    pub const VTABLE_BOUND_FUNC_WATCH: usize = 0x12A04A8;
    /// Base vtable image token from IDA 0x77f5ee (`off_1222248`).
    pub const VTABLE_FUNC_BASE: usize = 0x1222248;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm")]
// 0x77e930 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm
// was: boost::unordered::detail::table<...map<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>...>::create_buckets(unsigned long)
pub fn stub_0x77e930(table: &mut script_map_table::ScriptDebuggerTable, n: usize) {
    // IDA 0x77e994: array_constructor allocates n+1 buckets (v17 = a1+3
    // allocator hook, v16 out-pointer).
    let mut buckets: Vec<Option<usize>> = vec![None; n + 1];
    // IDA 0x77e9a0-0x77e9b2: if old buckets exist, splice the old tail
    // (`new[n] = old[size]`) then `operator delete` the old array.
    if let Some(old) = table.buckets.take() {
        if table.bucket_count < old.len() {
            buckets[n] = old[table.bucket_count];
        }
    }
    table.buckets = Some(buckets);
    // IDA 0x77e9be/0x77e9c4: size = n, buckets_ = new array.
    table.bucket_count = n;
    // IDA 0x77e9c6-0x77ea02: max = ceil(mlf * n), clamped to u32::MAX
    // (v12 starts -1; kept unless ceil < 4294967300.0).
    let grown = (table.mlf as f64 * n as f64).ceil();
    table.max_count = if grown < 4294967300.0 {
        grown as u32
    } else {
        u32::MAX
    };
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE20min_buckets_for_sizeEm")]
// 0x77ea58 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE20min_buckets_for_sizeEm
// was: boost::unordered::detail::table<...map<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>...>::min_buckets_for_size(unsigned long)const
pub fn stub_0x77ea58(table: &script_map_table::ScriptDebuggerTable, size: usize) -> usize {
    // IDA 0x77ea7c: rooms = floor(size / mlf).
    let rooms = (size as f64 / table.mlf as f64).floor();
    // IDA 0x77ea80-0x77ea96: need = 0 unless rooms < 4294967300.0, then
    // need = (u32)rooms + 1.
    let need: u64 = if rooms < 4294967300.0 {
        rooms as u64 + 1
    } else {
        0
    };
    // IDA 0x77ea9e-0x77ead8: binary search of prime_list_template for the
    // first prime >= need (v4 starts at 38, halving).
    script_map_table::prime_at_least(need)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::rehash_impl(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE11rehash_implEm")]
// 0x77eae8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE11rehash_implEm
// was: boost::unordered::detail::table_impl<...map<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>...>::rehash_impl(unsigned long)
pub fn stub_0x77eae8(table: &mut script_map_table::ScriptDebuggerTable) {
    // IDA 0x77eaee: create_buckets() grows/shrinks the array first.
    let n = table.bucket_count;
    stub_0x77e930(table, n);
    // IDA 0x77eaf2-0x77eb0c: drain the tail chain at buckets[size],
    // re-placing each node via place_in_bucket until the link is null.
    let mut link = table
        .buckets
        .as_ref()
        .and_then(|b| b.get(table.bucket_count).copied().flatten());
    while let Some(idx) = link {
        link = stub_0x77eb14(table, idx);
    }
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE")]
// 0x77eb14 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table_impl<...map<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>...>::place_in_bucket(table&, ptr_bucket*)
pub fn stub_0x77eb14(
    table: &mut script_map_table::ScriptDebuggerTable,
    idx: usize,
) -> Option<usize> {
    // Onward link: IDA unlinks the node from its old chain (`*v2 = next`,
    // 0x77eb3e) and returns it so rehash_impl keeps draining.
    let onward = table.nodes.get(idx).and_then(|n| n.next);
    // IDA 0x77eb2c: group = stored_hash % bucket_count (___umodsi3).
    let bucket = table.bucket_of(table.nodes.get(idx)?.hash)?;
    let buckets = table.buckets.as_mut()?;
    if buckets.get(bucket).copied().flatten().is_some() {
        // IDA 0x77eb30-0x77eb54: bucket occupied — splice the node to the
        // head of its hash group (`node.next = head.next`,
        // `head.next = node-or-null`).
        let head = buckets[bucket].unwrap();
        table.nodes[idx].next = table.nodes[head].next;
        table.nodes[head].next = Some(idx);
    } else {
        // IDA 0x77eb58-0x77eb62: empty bucket takes the node; the rehash
        // drain continues from the unlinked successor.
        buckets[bucket] = Some(idx);
        table.nodes[idx].next = None;
    }
    onward
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEEEEE9constructEv")]
// 0x77eb6c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEEEEE9constructEv
// was: boost::unordered::detail::node_constructor<allocator<ptr_node<pair<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>>>>::construct(void)
pub fn stub_0x77eb6c(ct: &mut script_map_table::NodeCtor) -> u8 {
    // IDA 0x77eb72: node slot already occupied — return the flag byte and
    // clear it when set (both paths yield 0: `result = flag; if (flag)
    // { result = 0; flag = 0; }`).
    if ct.occupied {
        ct.constructed = false;
        return 0;
    }
    // IDA 0x77eb86-0x77eb9e: fresh 0x10-byte node, hash/key words zeroed
    // (`*(v3+8) = 0` QWORD), occupied hook set, report 1.
    ct.node = Some(script_map_table::MapNode::default());
    ct.occupied = true;
    ct.constructed = true;
    1
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::find_node_impl<RBX::Script const*,std::equal_to<RBX::Script const*>>(unsigned long,RBX::Script const* const&,std::equal_to<RBX::Script const*> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14find_node_implIS8_SI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_")]
// 0x77eba4 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14find_node_implIS8_SI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
// was: boost::unordered::detail::table_impl<...map<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>...>::find_node_impl<...>(hash, key&, eq&)const
pub fn stub_0x77eba4(
    table: &script_map_table::ScriptDebuggerTable,
    hash: u32,
    key: usize,
) -> Option<usize> {
    // IDA 0x77ebc2/0x77ebbe: bucket = hash % size (___umodsi3); empty table
    // (0x77ebc6 CBZ) yields null.
    let bucket = table.bucket_of(hash)?;
    if table.size == 0 {
        return None;
    }
    // IDA 0x77ebcc: head = buckets[bucket]; null head yields null.
    let mut link = table
        .buckets
        .as_ref()?
        .get(bucket)
        .copied()
        .flatten();
    // IDA 0x77ebd4-0x77ec08: walk the group; node = link - 8 (0x77ebde).
    // The `i == 8` break (0x77ebe6) is the null-link sentinel, folded here.
    while let Some(idx) = link {
        let node = table.nodes.get(idx)?;
        if node.hash == hash {
            // IDA 0x77ebec-0x77ebf8: hash hit — return node on key match.
            if node.key == key {
                return Some(idx);
            }
        } else {
            // IDA 0x77ec02-0x77ec08: hash miss whose group differs ends the
            // group — return null.
            let group = (node.hash as usize) % table.bucket_count.max(1);
            if group != bucket {
                return None;
            }
        }
        link = node.next;
    }
    None
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::erase_key(RBX::Script const* const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_")]
// 0x77ec10 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_
// was: boost::unordered::detail::table_impl<...map<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>...>::erase_key(Script const* const&)
pub fn stub_0x77ec10(table: &mut script_map_table::ScriptDebuggerTable, key: usize) -> usize {
    // IDA 0x77ec1a: empty table (size at +8 zero) erases nothing.
    if table.size == 0 || table.bucket_count == 0 {
        return 0;
    }
    // IDA 0x77ec28/0x77ec32 (disasm ADD R6,R1,R1,LSR#3; BLX ___umodsi3):
    // h = key + (key >> 3); bucket = h % count.
    let h = (key.wrapping_add(key >> 3) as u32) as u32;
    let bucket = (h as usize) % table.bucket_count;
    let mut prev_link: Option<usize> = None;
    let mut link = table
        .buckets
        .as_ref()
        .and_then(|b| b.get(bucket).copied().flatten());
    // IDA 0x77ec46-0x77ec6e: walk the group while the stored hash stays in
    // this bucket (0x77ec56 BNE exits with 0).
    while let Some(idx) = link {
        let node = match table.nodes.get(idx) {
            Some(n) => *n,
            None => return 0,
        };
        if (node.hash as usize) % table.bucket_count != bucket {
            break;
        }
        // IDA 0x77ec58-0x77ec64: hash AND key match — delete + fix, return
        // the freed count.
        if node.hash == h && node.key == key {
            // IDA 0x77ec88/0x77ec90: delete_nodes(head, stop) frees the
            // matched node, then fix_bucket repairs the bucket head.
            // Head match: drain one node via delete_nodes (head != stop,
            // then head == stop), mirroring `delete_nodes(v6, *v4)`.
            // Mid-chain match: unlink exactly one node — the one-node
            // range the IDA call would free.
            let next = node.next;
            let freed = if prev_link.is_none() {
                stub_0x77ec98(table, bucket, next)
            } else {
                let p = prev_link.unwrap();
                table.nodes[p].next = next;
                if let Some(live) = table.live.get_mut(idx) {
                    *live = false;
                }
                table.size = table.size.saturating_sub(1);
                1
            };
            let head = table
                .buckets
                .as_ref()
                .and_then(|b| b.get(bucket).copied().flatten());
            stub_0x77ecd4(table, bucket, head);
            return freed;
        }
        prev_link = Some(idx);
        link = node.next;
    }
    // IDA 0x77ec70/0x77ec78: miss — return 0.
    0
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12delete_nodesEPNS1_10ptr_bucketESM_")]
// 0x77ec98 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12delete_nodesEPNS1_10ptr_bucketESM_
// was: boost::unordered::detail::table<...map<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>...>::delete_nodes(ptr_bucket*, ptr_bucket*)
pub fn stub_0x77ec98(
    table: &mut script_map_table::ScriptDebuggerTable,
    slot: usize,
    stop: Option<usize>,
) -> usize {
    // IDA 0x77eca4-0x77ecc8: unlink `*head` (`*a2 = node.next`, 0x77ecb6),
    // `operator delete` the node (0x77ecb8), decrement size (0x77ecc2),
    // until `*head == stop`.
    let mut freed = 0usize;
    loop {
        let cur = table
            .buckets
            .as_ref()
            .and_then(|b| b.get(slot).copied().flatten());
        if cur == stop {
            break;
        }
        let idx = match cur {
            Some(i) => i,
            None => break,
        };
        let next = table.nodes.get(idx).and_then(|n| n.next);
        if let Some(buckets) = table.buckets.as_mut() {
            if let Some(head) = buckets.get_mut(slot) {
                *head = next;
            }
        }
        if let Some(live) = table.live.get_mut(idx) {
            *live = false;
        }
        table.size = table.size.saturating_sub(1);
        freed += 1;
    }
    // IDA 0x77ecd0: return the freed count.
    freed
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE10fix_bucketEmPNS1_10ptr_bucketE")]
// 0x77ecd4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE10fix_bucketEmPNS1_10ptr_bucketE
// was: boost::unordered::detail::table<...map<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>...>::fix_bucket(unsigned long, ptr_bucket*)
pub fn stub_0x77ecd4(
    table: &mut script_map_table::ScriptDebuggerTable,
    bucket: usize,
    node: Option<usize>,
) -> usize {
    // IDA 0x77ecda-0x77ecf8: non-null head — recompute its group; same
    // group returns the bucket, otherwise buckets[group] takes the node.
    let result = if let Some(idx) = node {
        let group = match table.nodes.get(idx) {
            Some(n) if table.bucket_count > 0 => {
                (n.hash as usize) % table.bucket_count
            }
            _ => return bucket,
        };
        if group == bucket {
            return bucket;
        }
        if let Some(buckets) = table.buckets.as_mut() {
            if let Some(slot) = buckets.get_mut(group) {
                *slot = node;
            }
        }
        group
    } else {
        // IDA 0x77ecfe-0x77ed00: null head — result is the bucket itself.
        bucket
    };
    // IDA 0x77ed0a-0x77ed0e: clear buckets[bucket] if it still points here.
    if let Some(buckets) = table.buckets.as_mut() {
        if buckets.get(bucket).copied().flatten() == node {
            if let Some(slot) = buckets.get_mut(bucket) {
                *slot = None;
            }
        }
    }
    result
}

#[doc(alias = "boost::shared_ptr<RBX::BaseScript>::shared_ptr<RBX::BaseScript>(boost::weak_ptr<RBX::BaseScript> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10BaseScriptEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
// 0x77ed18 — __ZN5boost10shared_ptrIN3RBX10BaseScriptEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::BaseScript>::shared_ptr<RBX::BaseScript>(boost::weak_ptr<RBX::BaseScript> const&, boost::detail::sp_nothrow_tag)
pub fn stub_0x77ed18(
    weak: &Weak<instance_vec_block::BaseScriptSlot>,
) -> Option<SharedPtr<instance_vec_block::BaseScriptSlot>> {
    // IDA 0x77ed26-0x77ed2c: px = 0, pi = weak.pi; null pi stays empty.
    // IDA 0x77ed56-0x77ed6e: lock spinlock_pool<1> slot
    // `44 * ((pi + 4) % 0x29)` (pthread_mutex_lock), bump use count
    // (0x77ed68) when non-zero, copy px (0x77ed7c); on zero uses unlock and
    // null the out pointer (0x77ed82/0x77ed88). `Weak::upgrade` is the same
    // locked check-and-bump, atomically.
    weak.upgrade()
}

#[doc(alias = "rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> const&)")]
#[doc(alias = "__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS8_EEEESA_EEPT_RKNS5_IT0_EE")]
// 0x77ed94 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS8_EEEESA_EEPT_RKNS5_IT0_EE
// was: boost::get_deleter<rbx::detail::sp_ms_deleter<vector<shared_ptr<Instance>>>, vector<shared_ptr<Instance>>>(shared_ptr<vector<shared_ptr<Instance>>> const&)
pub fn stub_0x77ed94(
    shared: &instance_vec_block::SharedInstanceVec,
) -> Option<instance_vec_block::MsDeleter> {
    // IDA 0x77ed98-0x77ed9e: null control block (pi at +4) yields null.
    let live = shared.ptr.as_ref()?;
    // IDA 0x77edb4: vtable probe for the sp_ms_deleter typeinfo. Our Arc
    // always carries make-shared deleter semantics, so the direct probe
    // hits whenever the pointer is live ...
    let _ = live;
    Some(instance_vec_block::MsDeleter)
    // ... and the IDA 0x77edd4/0x77edec esft2-wrapper fallback (probe the
    // wrapper, then its inner deleter) has no live-model counterpart:
    // there is no wrapper state to unwrap, so a live block reports the
    // deleter on the first probe.
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::vector(unsigned long,boost::shared_ptr<RBX::Instance> const&,std::allocator<boost::shared_ptr<RBX::Instance>> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS4_RKS5_")]
// 0x77edf0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS4_RKS5_
// was: std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<RBX::Instance>>>::vector(unsigned long, shared_ptr const&, allocator const&)
pub fn stub_0x77edf0(
    base: &mut instance_vec_block::InstanceVecBase,
    n: usize,
    value: &SharedPtr<instance_vec_block::InstanceSlot>,
) {
    // IDA 0x77ee18: _Vector_base(n) allocates the backing store ...
    stub_0x77eeb8(base, n);
    // ... then 0x77ee54 __uninitialized_fill_n_aux clones `value` n times
    // and 0x77ee64 sets finish = start + 8 * n (8-byte shared_ptr cells).
    base.storage.clear();
    base.storage.reserve(n);
    for _ in 0..n {
        base.storage.push(SharedPtr::clone(value));
    }
}

#[doc(alias = "std::_Vector_base<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_Vector_base(unsigned long,std::allocator<boost::shared_ptr<RBX::Instance>> const&)")]
#[doc(alias = "__ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS5_")]
// 0x77eeb8 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EEC2EmRKS5_
// was: std::_Vector_base<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<RBX::Instance>>>::_Vector_base(unsigned long, allocator const&)
pub fn stub_0x77eeb8(base: &mut instance_vec_block::InstanceVecBase, n: usize) {
    // IDA 0x77eec8-0x77eecc: zero the begin/end/cap triple ...
    base.storage = instance_vec_block::InstanceVec::new();
    // ... and 0x77eece-0x77eee0: on n != 0, `_M_allocate` reserves n cells
    // (end = start + 8 * n for 8-byte shared_ptr slots).
    if n != 0 {
        base.storage.reserve(n);
    }
}

#[doc(alias = "boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")]
#[doc(alias = "__ZN5boost10shared_ptrISt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_")]
// 0x77eeec — __ZN5boost10shared_ptrISt6vectorINS0_IN3RBX8InstanceEEESaIS4_EEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_
// was: boost::shared_ptr<vector<shared_ptr<Instance>>>::shared_ptr<vector<shared_ptr<Instance>>, rbx::detail::sp_ms_deleter<vector<shared_ptr<Instance>>>>(vector*, deleter)
pub fn stub_0x77eeec(
    vec: instance_vec_block::InstanceVec,
) -> SharedPtr<instance_vec_block::InstanceVec> {
    // IDA 0x77ef34: px = raw vector; 0x77ef5e builds the shared_count
    // (sp_counted_impl_pd, see 0x77eff4). The landing-pad path
    // (0x77ef66 flag check + 0x77ef72 vector dtor) destroys the vector if
    // the control-block allocation throws — infallible under Arc, so the
    // flag has no live counterpart.
    SharedPtr::new(vec)
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEN3rbx6detail13sp_ms_deleterIS9_EEEET_T0_")]
// 0x77eff4 — __ZN5boost6detail12shared_countC2IPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEN3rbx6detail13sp_ms_deleterIS9_EEEET_T0_
// was: boost::detail::shared_count::shared_count<vector<shared_ptr<Instance>>*, rbx::detail::sp_ms_deleter<vector<shared_ptr<Instance>>>>(vector*, deleter)
pub fn stub_0x77eff4(
    vec: instance_vec_block::InstanceVec,
) -> instance_vec_block::CountedBlock {
    // IDA 0x77f022: pi = 0, then 0x77f04a `operator new(0x20)` with
    // 0x77f05e-0x77f06c: vtable, use = 1, weak = 1, payload = px,
    // owned flag (+16) = 0.
    instance_vec_block::CountedBlock {
        uses: 1,
        weaks: 1,
        vec: Some(vec),
        deleter: instance_vec_block::MsDeleter,
        owned: false,
    }
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED1Ev")]
// 0x77f0f8 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED1Ev
// was: boost::detail::sp_counted_impl_pd<vector<shared_ptr<Instance>>*, rbx::detail::sp_ms_deleter<vector<shared_ptr<Instance>>>>::~sp_counted_impl_pd()
pub fn stub_0x77f0f8(cb: &mut instance_vec_block::CountedBlock) {
    // IDA 0x77f10c: reset vtable to the sp_counted_impl_pd image (fixed
    // layout in Rust — the drop below is the reset) ...
    // IDA 0x77f10e-0x77f11c: if the owned flag (+16) is set, run the
    // vector dtor and clear the flag.
    if cb.owned {
        cb.vec = None;
        cb.owned = false;
    }
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED0Ev")]
// 0x77f124 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEED0Ev
// was: boost::detail::sp_counted_impl_pd<vector<shared_ptr<Instance>>*, rbx::detail::sp_ms_deleter<vector<shared_ptr<Instance>>>>::~sp_counted_impl_pd() [deleting]
pub fn stub_0x77f124(cb: instance_vec_block::CountedBlock) {
    // IDA 0x77f15c-0x77f166: D1 body (vtable reset + conditional vector
    // dtor) ...
    let mut cb = cb;
    stub_0x77f0f8(&mut cb);
    // ... then 0x77f192 `operator delete` — the by-value drop here.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE7disposeEv")]
// 0x77f1dc — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<vector<shared_ptr<Instance>>*, rbx::detail::sp_ms_deleter<vector<shared_ptr<Instance>>>>::dispose(void)
pub fn stub_0x77f1dc(cb: &mut instance_vec_block::CountedBlock) -> u8 {
    // IDA 0x77f1e2-0x77f1f4: `result = owned; if (owned) { vector dtor;
    // owned = 0; return 0; } return result` — both paths yield 0.
    if cb.owned {
        cb.vec = None;
        cb.owned = false;
    }
    0
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE11get_deleterERKSt9type_info")]
// 0x77f1f8 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<vector<shared_ptr<Instance>>*, rbx::detail::sp_ms_deleter<vector<shared_ptr<Instance>>>>::get_deleter(type_info const&)
pub fn stub_0x77f1f8<'a>(
    cb: &'a instance_vec_block::CountedBlock,
    ti_name: &str,
) -> Option<&'a instance_vec_block::MsDeleter> {
    // IDA 0x77f1fc-0x77f20e: return a1+16 (the deleter slot) iff the
    // type_info name matches the sp_ms_deleter instantiation, else null.
    if ti_name == instance_vec_block::SP_MS_DELETER_TI_NAME {
        Some(&cb.deleter)
    } else {
        None
    }
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,rbx::detail::sp_ms_deleter<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE19get_untyped_deleterEv")]
// 0x77f210 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<vector<shared_ptr<Instance>>*, rbx::detail::sp_ms_deleter<vector<shared_ptr<Instance>>>>::get_untyped_deleter(void)
pub fn stub_0x77f210(
    cb: &instance_vec_block::CountedBlock,
) -> &instance_vec_block::MsDeleter {
    // IDA 0x77f212: unconditional `return a1 + 16`.
    &cb.deleter
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14delete_bucketsEv")]
// 0x77f214 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14delete_bucketsEv
// was: boost::unordered::detail::table<...map<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>...>::delete_buckets(void)
pub fn stub_0x77f214(table: &mut script_map_table::ScriptDebuggerTable) {
    // IDA 0x77f21a-0x77f21e: null bucket array — nothing to do.
    if table.buckets.is_none() {
        return;
    }
    // IDA 0x77f222-0x77f230: non-zero size — delete_nodes over the chain at
    // buckets[size] down to null. The arena model keeps one chain per
    // bucket instead of a single master chain, so each slot is drained in
    // turn; the observable outcome is identical (every node freed,
    // size driven to 0).
    if table.size != 0 {
        let slots = table.bucket_count + 1;
        for slot in 0..slots {
            stub_0x77ec98(table, slot, None);
            if table.size == 0 {
                break;
            }
        }
    }
    // ... then 0x77f236 `operator delete` the array and 0x77f23e zero the
    // +16 QWORD (max count + bucket pointer).
    table.buckets = None;
    table.max_count = 0;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::table(unsigned long,boost::hash<RBX::Script const*> const&,std::equal_to<RBX::Script const*> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE")]
// 0x77f244 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX6ScriptEPNS5_9Scripting14ScriptDebuggerEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
// was: boost::unordered::detail::table<...map<RBX::Script const* const, RBX::Scripting::ScriptDebugger*>...>::table(unsigned long, hash const&, equal const&, allocator const&)
pub fn stub_0x77f244(table: &mut script_map_table::ScriptDebuggerTable, hint: usize) {
    // IDA 0x77f258: tag byte = 0 ...
    // IDA 0x77f25a-0x77f2a0: prime search for the first prime >= hint
    // (same 38-entry binary search as min_buckets_for_size, end-clamped).
    let prime = script_map_table::prime_at_least(hint as u64);
    // IDA 0x77f2a4-0x77f2a8: QWORD +16 = 0 (no buckets yet), count = prime,
    // size = 0, mlf = 1.0f (1065353216).
    table.buckets = None;
    table.bucket_count = prime;
    table.size = 0;
    table.max_count = 0;
    table.mlf = 1.0;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// 0x77f2b0 — __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// was: RBX::Reflection::Described<RBX::Scripting::DebuggerManager, ...>::~Described()
pub fn stub_0x77f2b0(desc: &mut debugger_reflection::DebuggerManagerDesc) {
    // IDA 0x77f2b0: pure thunk — tail-calls `RBX::Instance::~Instance`.
    // The Instance base teardown is the whole body; no member cleanup.
    desc.torn_down = true;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// 0x77f2b4 — __ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// was: RBX::Reflection::Described<RBX::Scripting::DebuggerManager, ...>::~Described() [deleting]
pub fn stub_0x77f2b4(desc: Box<debugger_reflection::DebuggerManagerDesc>) {
    // IDA 0x77f304: D1 body (Instance dtor thunk) ...
    let mut desc = desc;
    stub_0x77f2b0(&mut desc);
    // ... then 0x77f30a `operator delete` — the by-value drop here.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::DebuggerWatch::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// 0x77f4ac — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch, void ()(void), 0>::BoundFuncDesc(void (DebuggerWatch::*)(void), char const*, Permissions, Attributes)
pub fn stub_0x77f4ac(
    desc: &mut debugger_reflection::BoundFuncDescWatch,
    member_ptr: usize,
    member_adjust: usize,
    name: &str,
    permissions: u32,
) {
    // IDA 0x77f4d2: chain `Described<DebuggerWatch>::classDescriptor()` —
    // the descriptor token is captured before the base ctor runs.
    desc.class_descriptor = 0;
    // IDA 0x77f4f2: `FunctionDescriptor` base ctor over the
    // `describedClassDescriptor` static — name/permissions recorded here.
    desc.name = name.to_string();
    desc.permissions = permissions;
    // IDA 0x77f50e: vtable = off_12A04A8 (BoundFuncDesc image).
    desc.vtable = debugger_reflection::VTABLE_BOUND_FUNC_WATCH;
    // IDA 0x77f4ce/0x77f51a: member pair `__PAIR64__(a3, a2)` stored at +40.
    desc.member_fn =
        debugger_reflection::MemberFnVoid { ptr: member_ptr, adjust: member_adjust };
    // IDA 0x77f542: return type = `Type::getSingleton<void>()` at +28.
    desc.returns_void = true;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED0Ev")]
// 0x77f5b0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch, void ()(void), 0>::~BoundFuncDesc() [deleting]
pub fn stub_0x77f5b0(desc: Box<debugger_reflection::BoundFuncDescWatch>) {
    // IDA 0x77f5ee: vtable reset to the FunctionDescriptor base image
    // (off_1222248) ...
    let mut desc = desc;
    desc.vtable = debugger_reflection::VTABLE_FUNC_BASE;
    // ... 0x77f614 `_M_clear` the signature list ...
    desc.signatures.clear();
    // ... then 0x77f61a `operator delete` — the by-value drop here.
}

#[cfg(test)]
mod ky_tests {
    use super::debugger_reflection::*;
    use super::instance_vec_block::*;
    use super::script_map_table::*;
    use super::*;

    fn live_table() -> ScriptDebuggerTable {
        let mut t = ScriptDebuggerTable::default();
        stub_0x77f244(&mut t, 3);
        let n = t.bucket_count;
        stub_0x77e930(&mut t, n);
        // Insert two nodes: (key 0x1000), (key 0x2000), head-linked per bucket.
        for (key, dbg) in [(0x1000usize, 0xAAusize), (0x2000usize, 0xBBusize)] {
            let h = key.wrapping_add(key >> 3) as u32;
            let mut ct = NodeCtor::default();
            assert_eq!(stub_0x77eb6c(&mut ct), 1, "ctor must allocate once");
            assert_eq!(stub_0x77eb6c(&mut ct), 0, "second construct is a no-op");
            let bucket = (h as usize) % t.bucket_count;
            let head = t.buckets.as_ref().unwrap()[bucket];
            let idx = t.nodes.len();
            let mut node = ct.node.unwrap();
            node.hash = h;
            node.key = key;
            node.debugger = dbg;
            node.next = head;
            t.nodes.push(node);
            t.live.push(true);
            t.buckets.as_mut().unwrap()[bucket] = Some(idx);
            t.size += 1;
        }
        t
    }

    #[test]
    fn find_round_trip() {
        let t = live_table();
        for key in [0x1000usize, 0x2000usize] {
            let h = key.wrapping_add(key >> 3) as u32;
            let idx = stub_0x77eba4(&t, h, key).expect("inserted key must be found");
            assert_eq!(t.nodes[idx].key, key, "find must return the matching node");
        }
    }

    #[test]
    fn find_miss_returns_none() {
        let t = live_table();
        assert!(stub_0x77eba4(&t, 0xdead, 0xbeef).is_none(), "unknown key must miss");
        let empty = ScriptDebuggerTable::default();
        assert!(
            stub_0x77eba4(&empty, 1, 1).is_none(),
            "empty table must miss without dividing by zero"
        );
    }

    #[test]
    fn erase_removes_and_counts() {
        let mut t = live_table();
        assert_eq!(stub_0x77ec10(&mut t, 0x9999), 0, "missing key erases nothing");
        assert_eq!(stub_0x77ec10(&mut t, 0x1000), 1, "present key erases one node");
        let h = 0x1000usize.wrapping_add(0x1000usize >> 3) as u32;
        assert!(stub_0x77eba4(&t, h, 0x1000).is_none(), "erased key must not be found");
        assert_eq!(t.size, 1, "size must track the deletion");
    }

    #[test]
    fn min_buckets_grows_with_size() {
        let mut t = ScriptDebuggerTable::default();
        t.mlf = 1.0;
        let small = stub_0x77ea58(&t, 1);
        let big = stub_0x77ea58(&t, 10_000);
        assert!(big >= small, "larger size needs at least as many buckets");
        assert!(PRIME_LIST.contains(&small), "result must come from the prime list");
        assert!(PRIME_LIST.contains(&big), "result must come from the prime list");
    }

    #[test]
    fn delete_buckets_drains_all() {
        let mut t = live_table();
        assert!(t.buckets.is_some());
        stub_0x77f214(&mut t);
        assert!(t.buckets.is_none(), "bucket array must be released");
        assert_eq!(t.max_count, 0, "max count must be zeroed");
        assert_eq!(t.size, 0, "all nodes must be deleted");
    }

    #[test]
    fn weak_lock_upgrades_and_expires() {
        let live: SharedPtr<BaseScriptSlot> = SharedPtr::new(BaseScriptSlot(7));
        let weak = SharedPtr::downgrade(&live);
        {
            let locked = stub_0x77ed18(&weak).expect("live weak must lock");
            assert_eq!(locked.0, 7, "locked pointer must carry the payload");
        }
        drop(live);
        assert!(stub_0x77ed18(&weak).is_none(), "expired weak must yield null");
    }
    #[test]
    fn deleter_probe_matches_name() {
        let cb = stub_0x77eff4(Vec::new());
        assert_eq!((cb.uses, cb.weaks), (1, 1), "control block starts 1/1");
        assert!(!cb.owned, "owned flag starts clear per IDA +16 = 0");
        assert!(
            stub_0x77f1f8(&cb, SP_MS_DELETER_TI_NAME).is_some(),
            "matching typeinfo must return the deleter slot"
        );
        assert!(
            stub_0x77f1f8(&cb, "something-else").is_none(),
            "foreign typeinfo must yield null"
        );
        assert!(
            stub_0x77f210(&cb).eq(&MsDeleter),
            "untyped deleter must return the +16 slot"
        );
    }

    #[test]
    fn dispose_and_d1_clear_owned() {
        let mut cb = stub_0x77eff4(vec![SharedPtr::new(InstanceSlot(1))]);
        cb.owned = true;
        assert_eq!(stub_0x77f1dc(&mut cb), 0, "dispose must return 0");
        assert!(!cb.owned && cb.vec.is_none(), "dispose must destroy the payload");
        let mut cb2 = stub_0x77eff4(Vec::new());
        cb2.owned = true;
        stub_0x77f0f8(&mut cb2);
        assert!(!cb2.owned, "D1 must clear the owned flag");
    }

    #[test]
    fn vec_fill_clones_value() {
        let mut base = InstanceVecBase::default();
        let v = SharedPtr::new(InstanceSlot(3));
        stub_0x77edf0(&mut base, 4, &v);
        assert_eq!(base.storage.len(), 4, "fill must set finish = start + n");
        assert!(
            base.storage.iter().all(|e| e.0 == 3),
            "every cell must clone the fill value"
        );
    }

    #[test]
    fn reflection_descriptors() {
        let mut m = DebuggerManagerDesc::default();
        stub_0x77f2b0(&mut m);
        assert!(m.torn_down, "D1 thunk must run the Instance teardown");
        stub_0x77f2b4(Box::new(DebuggerManagerDesc::default()));
        let mut d = BoundFuncDescWatch::default();
        stub_0x77f4ac(&mut d, 0x11, 0x22, "watch", 2);
        assert_eq!(d.member_fn, MemberFnVoid { ptr: 0x11, adjust: 0x22 });
        assert!(d.returns_void, "return type must be the void singleton");
        assert_eq!(d.vtable, VTABLE_BOUND_FUNC_WATCH);
        d.signatures.push(9);
        stub_0x77f5b0(Box::new(d));
    }
}
