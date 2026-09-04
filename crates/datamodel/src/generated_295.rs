// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|RBX::Workspace|RBX::Part|RBX::Model|RBX::Humanoid|RBX::Script|RBX::Players|RBX::Lighting (EA-sorted asc, NOT in global /tmp/global_eas.txt)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 36 stubs | range 0xf5de44..0xf660c4 | total filtered 13623, remaining 36 (0 after batch)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_b::HumanoidTarget;
use rbx_core::WeakPtr;
use std::collections::HashMap;
use crate::instance::PartInstance;

/// Rust model of `boost::unordered_map<RBX::PartInstance *, boost::weak_ptr<RBX::PartInstance>>`
/// (IDA `0xf5de44` family): pointer identity is the hash and equality (as with
/// `boost::hash<RBX::PartInstance *>` / `equal_to`), so the table is a plain `HashMap`.
pub type PartWeakMap = HashMap<*const PartInstance, WeakPtr<PartInstance>>;

/// Rust model of `RBX::Network::TopNErrorsPhysicsSender` for the `mf1` / `for_each`
/// binds below (IDA `0xf5e2f4` / `0xf5e324`): only pointer identity is used here.
/// Mirrors the unit struct in `rbx-network` (`crates/network/src/physics.rs`), which
/// `rbx-datamodel` cannot depend on (see the crate DAG in AGENTS.md).
pub struct TopNErrorsPhysicsSender;

/// Rust model of `boost::bind(mf1<void, TopNErrorsPhysicsSender, PartInstance&>,
/// value<Sender*>, _1)` (IDA `0xf5e2f4` / `0xf5e324`): the member entry point plus
/// the bound sender; the late arg is the part (cf. `BindPredicate` in generated_05).
pub struct TopNErrorsBind {
    pub func: fn(*mut TopNErrorsPhysicsSender, *const PartInstance),
    pub target: *mut TopNErrorsPhysicsSender,
}

/// Rust model of `RBX::Network::ErrorCompPhysicsSender` for the `mf1` bind below
/// (IDA `0xf5f804`): same treatment as `TopNErrorsPhysicsSender`.
pub struct ErrorCompPhysicsSender;

/// Rust model of `boost::bind(mf1<void, ErrorCompPhysicsSender, PartInstance&>,
/// value<Sender*>, _1)` (IDA `0xf5f804` / `0xf5f864`): twin of `TopNErrorsBind`.
pub struct ErrorCompBind {
    pub func: fn(*mut ErrorCompPhysicsSender, *const PartInstance),
    pub target: *mut ErrorCompPhysicsSender,
}

/// Rust model of `RBX::Network::TopNErrorsPhysicsSender::Nugget`: layout is unknown
/// (the senders are still unit structs in `rbx-network`); opaque until the sender
/// batch lands. Only moved into table entries by the emplace suite (IDA `0xf5e2a4`).
#[derive(Clone, Copy, Default)]
pub struct TopNErrorsNugget;

/// Rust model of `RBX::Network::ErrorCompPhysicsSender::Nugget`: same treatment as
/// `TopNErrorsNugget`. Cloned by the pair suite (IDA `0xf5f834`..`0xf5f854`).
#[derive(Clone, Copy, Default)]
pub struct ErrorCompNugget;

/// Rust model of `RBX::FastClusterMeshGenerator` for the C2 below (IDA
/// `0xf64c24`): the layout lives on the rendering side and is unrecovered;
/// opaque carrier like `MegaClusterInstance` in `instance.rs`.
#[derive(Default)]
pub struct FastClusterMeshGenerator {
    _opaque: (),
}

/// Rust model of `boost::unordered_map<shared_ptr<const PartInstance>, ...::Nugget>`
/// (IDA `0xf5e2a4` / `0xf5f794` families): `boost::hash<shared_ptr>` hashes the stored
/// pointer and `equal_to` compares it, so the key collapses to the raw pointer.
pub type TopNErrorsMap = HashMap<*const PartInstance, TopNErrorsNugget>;
/// Rust model of the `ErrorCompPhysicsSender` sibling table (IDA `0xf5f794` family).
pub type ErrorCompMap = HashMap<*const PartInstance, ErrorCompNugget>;

/// Rust model of `std::pair<shared_ptr<const PartInstance> const, ErrorComp Nugget>`
/// (IDA `0xf5f834`): `const` collapses in Rust; the retain on copy is the `Arc` clone.
pub type ErrorCompEntry = (SharedPtr<PartInstance>, ErrorCompNugget);

// 0xf5de44 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISH_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeISB_EEEERSM_RT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>> &)
pub fn stub_0xf5de44() {
    // IDA 0xf5de44: __picsymbolstub4 (LDR R12,=ptr; ADD R12,PC; LDR PC,[R12])
    // into table::fill_buckets — seats nodes into fresh buckets during rehash.
    // Bucket placement is HashMap-internal; monomorph artifact, no-op carrier.
}

// 0xf5de54 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::operator[](RBX::PartInstance * const&)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::operator[](RBX::PartInstance * const&)
pub fn stub_0xf5de54(map: &mut PartWeakMap, key: *const PartInstance) -> &mut WeakPtr<PartInstance> {
    // IDA 0xf5de54: __picsymbolstub4 into table_impl::operator[] — finds the key
    // (`boost::hash<PartInstance*>` hashes px, `equal_to` compares px) or inserts
    // a default weak. `entry().or_insert_with()` is the same lookup.
    map.entry(key).or_insert_with(WeakPtr::new)
}

// 0xf5df44 — j___ZN5boost9unordered6detail11node_holderISaINS1_8ptr_nodeISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEEEEED2Ev
#[doc(alias = "boost::unordered::detail::node_holder<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>>::~node_holder()")]
// was: boost::unordered::detail::node_holder<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>>>::~node_holder()
pub fn stub_0xf5df44() {
    // IDA 0xf5df44: __picsymbolstub4 into node_holder::~node_holder — frees the
    // half-built node unless released into the table. Allocation and insertion are
    // one `entry()` step in the HashMap model, so no held node can exist;
    // monomorph artifact, no-op carrier.
}

// 0xf5df64 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>>::construct(void)")]
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>>>::construct(void)
pub fn stub_0xf5df64() {
    // IDA 0xf5df64: __picsymbolstub4 into node_constructor::construct — allocates
    // and value-initializes one node from the stored value. Node allocation is
    // HashMap-internal; monomorph artifact, no-op carrier.
}

// 0xf5df94 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::create_buckets(unsigned long)
pub fn stub_0xf5df94() {
    // IDA 0xf5df94: __picsymbolstub4 into table::create_buckets — allocates the
    // bucket array during table construction. Construction-time-only helper;
    // capacity lives inside `HashMap`; monomorph artifact, no-op carrier.
}

// 0xf5dfa4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::reserve_for_insert(unsigned long)
pub fn stub_0xf5dfa4(map: &mut PartWeakMap, n: usize) {
    // IDA 0xf5dfa4: __picsymbolstub4 into table::reserve_for_insert — grows the
    // bucket array for `n` further inserts (rehashing as needed). `reserve` keeps
    // the same guarantee; the bucket poking is HashMap-internal.
    map.reserve(n);
}

// 0xf5dfb4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE4initERKSI_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::init(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>> const&)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::init(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>> const&)
pub fn stub_0xf5dfb4(dst: &mut PartWeakMap, src: &PartWeakMap) {
    // IDA 0xf5dfb4: __picsymbolstub4 into table::init — copy-initializes the table
    // (buckets plus nodes) from `src`. `clone` copies the same entries; the bucket
    // array itself is HashMap-internal.
    *dst = src.clone();
}

// 0xf5dfc4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE6assignERKSI_NS1_17integral_constantIbLb0EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>> const&,boost::unordered::detail::integral_constant<bool,false>)
pub fn stub_0xf5dfc4(dst: &mut PartWeakMap, src: &PartWeakMap) {
    // IDA 0xf5dfc4: __picsymbolstub4 into table::assign — whole-table copy. The
    // `integral_constant<false>` (don't preserve buckets) tag collapses: there is
    // no reusable bucket array in the HashMap model, so this is `init` (0xf5dfb4).
    *dst = src.clone();
}

// 0xf5dfd4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEED2Ev
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::~table()")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::~table()
pub fn stub_0xf5dfd4() {
    // IDA 0xf5dfd4: __picsymbolstub4 into table::~table — releases buckets and
    // nodes. Drop glue (compiler-managed here); monomorph artifact, no-op carrier.
}

// 0xf5e294 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *)
pub fn stub_0xf5e294() {
    // IDA 0xf5e294: __picsymbolstub4 (LDR R12,=ptr; ADD R12,PC; LDR PC,[R12])
    // into table_impl::erase_nodes (TopNErrors map) — unlinks and frees the node
    // range `[first, last)`. Ranges over node pointers have no model once nodes
    // collapse into `HashMap` entries; no-op carrier.
}

// 0xf5e2a4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERSA_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(rbx_core::SharedPtr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")]
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)
pub fn stub_0xf5e2a4(map: &mut TopNErrorsMap, key: *const PartInstance, value: TopNErrorsNugget) -> bool {
    // IDA 0xf5e2a4: __picsymbolstub4 into table_impl::emplace_impl (TopNErrors map) —
    // constructs the pair from `emplace_args1` and inserts it unless the key exists.
    // The returned node iterator collapses (positions are only observable through
    // node pointers, which have no model); the `inserted` half is the result.
    map.insert(key, value).is_none()
}

// 0xf5e2b4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")]
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)
pub fn stub_0xf5e2b4() {
    // IDA 0xf5e2b4: __picsymbolstub4 into node_constructor::construct_with_value
    // (TopNErrors map) — allocates one node and copy-constructs the pair from
    // `emplace_args1`. Node allocation is HashMap-internal; monomorph artifact,
    // no-op carrier.
}

// 0xf5e2c4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)
pub fn stub_0xf5e2c4() {
    // IDA 0xf5e2c4: __picsymbolstub4 into table::create_buckets (TopNErrors map) —
    // construction-time bucket-array allocation; capacity lives inside `HashMap`;
    // monomorph artifact, no-op carrier.
}

// 0xf5e2d4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)
pub fn stub_0xf5e2d4(map: &mut TopNErrorsMap, n: usize) {
    // IDA 0xf5e2d4: __picsymbolstub4 into table::reserve_for_insert (TopNErrors map).
    // Same guarantee as 0xf5dfa4: `reserve` for `n` further inserts.
    map.reserve(n);
}

// 0xf5e2e4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEED2Ev
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::~table()")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::~table()
pub fn stub_0xf5e2e4() {
    // IDA 0xf5e2e4: __picsymbolstub4 into table::~table (TopNErrors map) — releases
    // buckets and nodes. Drop glue (compiler-managed here); monomorph artifact,
    // no-op carrier.
}

// 0xf5e2f4 — j___ZNK5boost4_mfi3mf1IvN3RBX7Network23TopNErrorsPhysicsSenderENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,rbx_core::SharedPtr<RBX::PartInstance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,rbx_core::SharedPtr<RBX::PartInstance>)const")]
// was: boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,boost::shared_ptr<RBX::PartInstance>)const
pub fn stub_0xf5e2f4(bound: &TopNErrorsBind, arg: *const PartInstance) {
    // IDA 0xf5e2f4: __picsymbolstub4 into mf1::operator() — loads the member fn
    // pointer plus the bound `value<TopNErrorsPhysicsSender*>` and calls it with
    // the part arg. Collapses to the direct call (cf. generated_05::stub_0x7057e0).
    (bound.func)(bound.target, arg);
}

// 0xf5e324 — j___ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network23TopNErrorsPhysicsSenderERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>)")]
pub fn stub_0xf5e324(items: &[*const PartInstance], bound: &TopNErrorsBind) {
    // IDA 0xf5e324: __picsymbolstub4 into for_each over `Intrusive::Set<PartInstance>`
    // iterators applying the TopNErrors mf1 bind. The intrusive links collapse into
    // the slice; each element rides the same direct call as 0xf5e2f4
    // (cf. generated_05::stub_0x704748).
    for item in items {
        (bound.func)(bound.target, *item);
    }
}

// 0xf5ebe4 — j___ZN3RBX11shared_fromIKNS_12PartInstanceEEEN5boost10shared_ptrIT_EEPS5_
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance const> RBX::shared_from<RBX::PartInstance const>(RBX::PartInstance const*)")]
// was: boost::shared_ptr<RBX::PartInstance const> RBX::shared_from<RBX::PartInstance const>(RBX::PartInstance const*)
pub fn stub_0xf5ebe4(out: *mut Option<SharedPtr<PartInstance>>, this: *const PartInstance) {
    // IDA 0xf5ebe4: __picsymbolstub4 into shared_from<PartInstance const> — null
    // `this` yields an empty out; else the embedded `enable_shared_from_this` weak
    // (`this + 40`, same discipline as `Instance`, IDA 0x7039e4) is upgraded into the
    // out slot. `Weak::upgrade` is the same locked copy; an expired weak yields `None`
    // where the original throws `bad_weak_ptr`.
    // SAFETY: `out` must be writable `Option<SharedPtr>` storage; non-null `this`
    // must point to a valid `PartInstance`.
    unsafe {
        *out = if this.is_null() { None } else { (*this).weak_owner.upgrade() };
    }
}

// 0xf5f6f4 — j___ZN5boost10shared_ptrIN3RBX12PartInstanceEE5resetEv
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance>::reset(void)")]
// was: boost::shared_ptr<RBX::PartInstance>::reset(void)
pub fn stub_0xf5f6f4(slot: &mut Option<SharedPtr<PartInstance>>) {
    // IDA 0xf5f6f4: __picsymbolstub4 into shared_ptr<PartInstance>::reset — releases
    // the owned reference (`px = 0`). `Option::take` drops the same `Arc`.
    slot.take();
}

// 0xf5f784 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> *)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> *)
pub fn stub_0xf5f784() {
    // IDA 0xf5f784: __picsymbolstub4 (LDR R12,=ptr; ADD R12,PC; LDR PC,[R12])
    // into table_impl::erase_nodes (ErrorComp map) — unlinks and frees the node
    // range `[first, last)`. Ranges over node pointers have no model once nodes
    // collapse into `HashMap` entries; no-op carrier.
}

// 0xf5f794 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERSA_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>(rbx_core::SharedPtr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> const&)")]
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> const&)
pub fn stub_0xf5f794(map: &mut ErrorCompMap, key: *const PartInstance, value: ErrorCompNugget) -> bool {
    // IDA 0xf5f794: __picsymbolstub4 into table_impl::emplace_impl (ErrorComp map).
    // Same shape as 0xf5e2a4: the `inserted` half is the result, the node iterator
    // half collapses.
    map.insert(key, value).is_none()
}

// 0xf5f7a4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> const&)")]
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> const&)
pub fn stub_0xf5f7a4() {
    // IDA 0xf5f7a4: __picsymbolstub4 into node_constructor::construct_with_value
    // (ErrorComp map) — allocates one node and copy-constructs the pair from
    // `emplace_args1`. Node allocation is HashMap-internal; monomorph artifact,
    // no-op carrier.
}

// 0xf5f7b4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)
pub fn stub_0xf5f7b4() {
    // IDA 0xf5f7b4: __picsymbolstub4 into table::create_buckets (ErrorComp map) —
    // construction-time bucket-array allocation; capacity lives inside `HashMap`;
    // monomorph artifact, no-op carrier.
}

// 0xf5f7c4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)
pub fn stub_0xf5f7c4(map: &mut ErrorCompMap, n: usize) {
    // IDA 0xf5f7c4: __picsymbolstub4 into table::reserve_for_insert (ErrorComp map).
    // Same guarantee as 0xf5dfa4: `reserve` for `n` further inserts.
    map.reserve(n);
}

// 0xf5f7d4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEED2Ev
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::~table()")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::~table()
pub fn stub_0xf5f7d4() {
    // IDA 0xf5f7d4: __picsymbolstub4 into table::~table (ErrorComp map) — releases
    // buckets and nodes. Drop glue (compiler-managed here); monomorph artifact,
    // no-op carrier.
}

// 0xf5f804 — j___ZNK5boost4_mfi3mf1IvN3RBX7Network22ErrorCompPhysicsSenderENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,rbx_core::SharedPtr<RBX::PartInstance>)const")]
// was: boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,boost::shared_ptr<RBX::PartInstance>)const
pub fn stub_0xf5f804(bound: &ErrorCompBind, arg: *const PartInstance) {
    // IDA 0xf5f804: __picsymbolstub4 into mf1::operator() (ErrorComp member) — twin
    // of 0xf5e2f4: loads the member fn plus the bound sender and calls it with the
    // part arg; collapses to the direct call.
    (bound.func)(bound.target, arg);
}

// 0xf5f824 — j___ZNSt10_List_baseIN5boost10shared_ptrIN3RBX12PartInstanceEEENS0_19fast_pool_allocatorIS4_NS0_33default_user_allocator_new_deleteENS0_5mutexELj32ELj0EEEE8_M_clearEv
#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::PartInstance>,boost::fast_pool_allocator<rbx_core::SharedPtr<RBX::PartInstance>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::_M_clear(void)")]
// was: std::_List_base<boost::shared_ptr<RBX::PartInstance>,boost::fast_pool_allocator<boost::shared_ptr<RBX::PartInstance>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::_M_clear(void)
pub fn stub_0xf5f824(list: &mut Vec<SharedPtr<PartInstance>>) {
    // IDA 0xf5f824: __picsymbolstub4 into _List_base::_M_clear — unlinks every node
    // and releases each `shared_ptr` (the `fast_pool_allocator` storage collapses).
    // `Vec::clear` drops the same `Arc`s.
    list.clear();
}

// 0xf5f834 — j___ZNSt4pairIKN5boost10shared_ptrIKN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2ERKSA_
#[doc(alias = "std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair(std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget> const&)")]
// was: std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair(std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget> const&)
pub fn stub_0xf5f834(out: *mut ErrorCompEntry, src: &ErrorCompEntry) {
    // IDA 0xf5f834: __picsymbolstub4 into pair copy ctor — retains the key
    // `shared_ptr` and copies the nugget (the `shared_count` copies are the `Arc`
    // clone plus the `Copy` nugget). Clone plus write is the same pair.
    // SAFETY: `out` must be writable entry storage; `src` must be a valid entry.
    unsafe {
        *out = src.clone();
    }
}

// 0xf5f844 — j___ZNSt4pairIKN5boost10shared_ptrIKN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2INS1_IS3_EES9_EERKS_IT_T0_E
#[doc(alias = "std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair<rbx_core::SharedPtr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>(std::pair const&<rbx_core::SharedPtr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>)")]
// was: std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>(std::pair const&<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>)
pub fn stub_0xf5f844(out: *mut ErrorCompEntry, src: &ErrorCompEntry) {
    // IDA 0xf5f844: __picsymbolstub4 into pair converting ctor from
    // `pair<shared_ptr<PartInstance>, Nugget>`. The source key's missing `const`
    // collapses in Rust (`SharedPtr` is already shared), so this is the same clone
    // as 0xf5f834.
    // SAFETY: same contract as `stub_0xf5f834`.
    unsafe {
        *out = src.clone();
    }
}

// 0xf5f854 — j___ZNSt4pairIN5boost10shared_ptrIN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2ERKS4_RKS7_
#[doc(alias = "std::pair<rbx_core::SharedPtr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair(rbx_core::SharedPtr<RBX::PartInstance> const&,RBX::Network::ErrorCompPhysicsSender::Nugget const&)")]
// was: std::pair<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair(boost::shared_ptr<RBX::PartInstance> const&,RBX::Network::ErrorCompPhysicsSender::Nugget const&)
pub fn stub_0xf5f854(out: *mut ErrorCompEntry, part: &SharedPtr<PartInstance>, nugget: &ErrorCompNugget) {
    // IDA 0xf5f854: __picsymbolstub4 into pair ctor from `(shared_ptr const&,
    // Nugget const&)` — retains the key and copies the value into fresh storage
    // (the `shared_count` copy is the `Arc` clone).
    // SAFETY: `out` must be writable entry storage; `part`/`nugget` must be valid.
    unsafe {
        *out = (part.clone(), nugget.clone());
    }
}

// 0xf5f864 — j___ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network22ErrorCompPhysicsSenderERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>)")]
pub fn stub_0xf5f864(items: &[*const PartInstance], bound: &ErrorCompBind) {
    // IDA 0xf5f864: __picsymbolstub4 (LDR R12,=ptr; ADD R12,PC; LDR PC,[R12])
    // into for_each over `Intrusive::Set<PartInstance>` iterators applying the
    // ErrorComp mf1 bind — twin of the TopNErrors loop at 0xf5e324, riding the
    // `ErrorCompBind` direct call from 0xf5f804.
    for item in items {
        (bound.func)(bound.target, *item);
    }
}

// 0xf62234 — j___ZN3RBX11shared_fromINS_8HumanoidEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::Humanoid> RBX::shared_from<RBX::Humanoid>(RBX::Humanoid*)")]
// was: boost::shared_ptr<RBX::Humanoid> RBX::shared_from<RBX::Humanoid>(RBX::Humanoid*)
pub fn stub_0xf62234(ptr: *const HumanoidTarget) -> SharedPtr<HumanoidTarget> {
    // IDA 0xf62234: __picsymbolstub4 into shared_from<Humanoid> — the
    // `Humanoid` target collapses to its `Instance` base (cf. `HumanoidTarget`
    // in generated_b), so this is the same weak-owner lock as
    // shared_from<PartInstance> (0x5e1610): an expired owner throws
    // `bad_weak_ptr`, mapped to a panic.
    // SAFETY: `ptr` must point into a live `SharedPtr<Humanoid>`.
    unsafe {
        if (*ptr).weak_owner.upgrade().is_none() {
            panic!("0xf62234 shared_from<Humanoid>: bad_weak_ptr");
        }
        let owned = SharedPtr::from_raw(ptr);
        let out = owned.clone();
        core::mem::forget(owned);
        out
    }
}

// 0xf64c24 — j___ZN3RBX24FastClusterMeshGeneratorC2EPN4Ogre12VisualEngineEPNS_8HumanoidEjb
#[doc(alias = "RBX::FastClusterMeshGenerator::FastClusterMeshGenerator(Ogre::VisualEngine *,RBX::Humanoid *,unsigned int,bool)")]
pub fn stub_0xf64c24(
    this: *mut FastClusterMeshGenerator,
    engine: *const (),
    humanoid: *const HumanoidTarget,
    count: u32,
    flag: bool,
) -> *mut FastClusterMeshGenerator {
    // IDA 0xf64c24: __picsymbolstub4 into FastClusterMeshGenerator::C2
    // (`Ogre::VisualEngine *`, `Humanoid *`, uint, bool) — vtable install plus
    // the engine/humanoid/count/flag words.
    // // BUG: the generator layout lives on the rendering side and is
    // // unrecovered here; construction has no modeled effect yet.
    let _ = (engine, humanoid, count, flag);
    this
}

// 0xf65d44 — j___ZN3RBX17MegaClusterLegacy14bind_templatedINS0_16VoxelGridOverlayEEEvRKN5boost10shared_ptrINS_12PartInstanceEEE
#[doc(alias = "void RBX::MegaClusterLegacy::bind_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// was: void RBX::MegaClusterLegacy::bind_templated<RBX::MegaClusterLegacy::VoxelGridOverlay>(boost::shared_ptr<RBX::PartInstance> const&)
pub fn stub_0xf65d44(part: &SharedPtr<PartInstance>) {
    // IDA 0xf65d44: __picsymbolstub4 into
    // MegaClusterLegacy::bind_templated<VoxelGridOverlay> (real impl 0xc09ab4):
    // ReleaseAssert part type == MEGACLUSTER_PART (10) (0xc09af2), retain into
    // legacy +4/+8 (0xc09b22-0xc09b70), ReleaseAssert gfx null (0xc09b8e), the
    // part+176 self-link (0xc09bbe), the +64/+76 vector reserves (0xc09bcc-
    // 0xc09c0c), the TerrainCellListener log, then
    // `Voxel::Grid::connectListener` (0xc09c48).
    // // BUG: MegaClusterLegacy layout, the PartInstance gfx word, and the
    // // voxel grid have no model here; none of the stores run yet.
    let _ = part;
}

// 0xf65d54 — j___ZN3RBX17MegaClusterLegacy14bind_templatedINS_19MegaClusterInstanceEEEvRKN5boost10shared_ptrINS_12PartInstanceEEE
#[doc(alias = "void RBX::MegaClusterLegacy::bind_templated<RBX::MegaClusterInstance>(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// was: void RBX::MegaClusterLegacy::bind_templated<RBX::MegaClusterInstance>(boost::shared_ptr<RBX::PartInstance> const&)
pub fn stub_0xf65d54(part: &SharedPtr<PartInstance>) {
    // IDA 0xf65d54: __picsymbolstub4 into
    // MegaClusterLegacy::bind_templated<MegaClusterInstance> (real impl
    // 0xc09c4c): same body as the VoxelGridOverlay twin (0xf65d44) — asserts,
    // retain, self-link, reserves — except the log carries the bound part
    // (`*(a1 + 4)`, 0xc09d82) and the tail is
    // `MegaClusterInstance::connectListener` (0xc09dde).
    // // BUG: same unmodeled stores as 0xf65d44.
    let _ = part;
}

// 0xf660c4 — j___ZN3RBX13DataModelUtil15getSpecialShapeEPNS_12PartInstanceE
#[doc(alias = "RBX::DataModelUtil::getSpecialShape(RBX::PartInstance *)")]
pub fn stub_0xf660c4(_util: *const (), _part: *const PartInstance) -> *const () {
    // IDA 0xf660c4: __picsymbolstub4 into DataModelUtil::getSpecialShape
    // (real impl 0xc248a0): null unless the +45 flag word bit 2 is set and the
    // +14 table holds a DataModelMesh isA match (0xc248f2-0xc24a0e), in which
    // case the matching entry pointer returns; the part arg is unused.
    // // BUG: the DataModelUtil flag/table words and the DataModelMesh
    // // descriptor walk have no model; always the null path.
    core::ptr::null()
}
