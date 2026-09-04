//! core shard GI — 100 core stubs EA-sorted, 0xf4e584..0xf4ebf4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf4e574).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf4e574.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator new(unsigned long)")]
// 0xf4e584 — j___ZN3RBX9AllocatorINS_15BallCellContactEEnwEm
pub fn stub_f4e584() {
    // IDA 0xf4e584: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::operator new(unsigned long)")]
// 0xf4e594 — j___ZN3RBX9AllocatorINS_15BallPolyContactEEnwEm
pub fn stub_f4e594() {
    // IDA 0xf4e594: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator delete(void *)")]
// 0xf4e5a4 — j___ZN3RBX9AllocatorINS_15PolyCellContactEEdlEPv
pub fn stub_f4e5a4() {
    // IDA 0xf4e5a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator new(unsigned long)")]
// 0xf4e5b4 — j___ZN3RBX9AllocatorINS_15PolyCellContactEEnwEm
pub fn stub_f4e5b4() {
    // IDA 0xf4e5b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator delete(void *)")]
// 0xf4e5c4 — j___ZN3RBX9AllocatorINS_15PolyPolyContactEEdlEPv
pub fn stub_f4e5c4() {
    // IDA 0xf4e5c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator new(unsigned long)")]
// 0xf4e5d4 — j___ZN3RBX9AllocatorINS_15PolyPolyContactEEnwEm
pub fn stub_f4e5d4() {
    // IDA 0xf4e5d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::Allocator(void)")]
// 0xf4e5e4 — j___ZN3RBX9AllocatorINS_16BallBlockContactEEC2Ev
pub fn stub_f4e5e4() {
    // IDA 0xf4e5e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator delete(void *)")]
// 0xf4e5f4 — j___ZN3RBX9AllocatorINS_16BallBlockContactEEdlEPv
pub fn stub_f4e5f4() {
    // IDA 0xf4e5f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator new(unsigned long)")]
// 0xf4e604 — j___ZN3RBX9AllocatorINS_16BallBlockContactEEnwEm
pub fn stub_f4e604() {
    // IDA 0xf4e604: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::operator new(unsigned long)")]
// 0xf4e614 — j___ZN3RBX9AllocatorINS_17BlockBlockContactEEnwEm
pub fn stub_f4e614() {
    // IDA 0xf4e614: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode* boost::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::construct<int,int,RBX::Vector3int32>(int &,int &,RBX::Vector3int32 const&)")]
// 0xf4e624 — j___ZN5boost11object_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeENS1_16roblox_allocatorEE9constructIiiNS1_12Vector3int32EEEPS7_RT_RT0_RKT1_
pub fn stub_f4e624() {
    // IDA 0xf4e624: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::~object_pool()")]
// 0xf4e634 — j___ZN5boost11object_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeENS1_16roblox_allocatorEED2Ev
pub fn stub_f4e634() {
    // IDA 0xf4e634: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::construct(void)")]
// 0xf4e644 — j___ZN5boost11object_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEE9constructEv
pub fn stub_f4e644() {
    // IDA 0xf4e644: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::~object_pool()")]
// 0xf4e654 — j___ZN5boost11object_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEED2Ev
pub fn stub_f4e654() {
    // IDA 0xf4e654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4e664 — j___ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4e664() {
    // IDA 0xf4e664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e674 — j___ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e674() {
    // IDA 0xf4e674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4e684 — j___ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4e684() {
    // IDA 0xf4e684: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4e694 — j___ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4e694() {
    // IDA 0xf4e694: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e6a4 — j___ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e6a4() {
    // IDA 0xf4e6a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4e6b4 — j___ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4e6b4() {
    // IDA 0xf4e6b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4e6c4 — j___ZN5boost14singleton_poolIN3RBX15BallBallContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4e6c4() {
    // IDA 0xf4e6c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e6d4 — j___ZN5boost14singleton_poolIN3RBX15BallBallContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e6d4() {
    // IDA 0xf4e6d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e6e4 — j___ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e6e4() {
    // IDA 0xf4e6e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4e6f4 — j___ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4e6f4() {
    // IDA 0xf4e6f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallPolyContact,212u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e704 — j___ZN5boost14singleton_poolIN3RBX15BallPolyContactELj212ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e704() {
    // IDA 0xf4e704: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallPolyContact,212u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4e714 — j___ZN5boost14singleton_poolIN3RBX15BallPolyContactELj212ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4e714() {
    // IDA 0xf4e714: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e724 — j___ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e724() {
    // IDA 0xf4e724: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4e734 — j___ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4e734() {
    // IDA 0xf4e734: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::PolyPolyContact,216u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e744 — j___ZN5boost14singleton_poolIN3RBX15PolyPolyContactELj216ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e744() {
    // IDA 0xf4e744: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::PolyPolyContact,216u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4e754 — j___ZN5boost14singleton_poolIN3RBX15PolyPolyContactELj216ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4e754() {
    // IDA 0xf4e754: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4e764 — j___ZN5boost14singleton_poolIN3RBX16BallBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4e764() {
    // IDA 0xf4e764: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e774 — j___ZN5boost14singleton_poolIN3RBX16BallBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e774() {
    // IDA 0xf4e774: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BlockBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e784 — j___ZN5boost14singleton_poolIN3RBX17BlockBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e784() {
    // IDA 0xf4e784: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BlockBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4e794 — j___ZN5boost14singleton_poolIN3RBX17BlockBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4e794() {
    // IDA 0xf4e794: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::MegaClusterMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4e7a4 — j___ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4e7a4() {
    // IDA 0xf4e7a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::simple_segregated_storage<unsigned long>::add_ordered_block(void *,unsigned long,unsigned long)")]
// 0xf4e7b4 — j___ZN5boost25simple_segregated_storageImE17add_ordered_blockEPvmm
pub fn stub_f4e7b4() {
    // IDA 0xf4e7b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::ContactManager *>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::operator()<boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list1<RBX::Primitive * const&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool> &,boost::_bi::list1<RBX::Primitive * const&> &,int)")]
// 0xf4e7c4 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX14ContactManagerEEENS_3argILi1EEENS2_IbEES9_S9_EclINS_4_mfi3mf4IvS4_PNS3_9PrimitiveEbbbEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f4e7c4() {
    // IDA 0xf4e7c4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::pool<RBX::roblox_allocator>::purge_memory(void)")]
// 0xf4e7d4 — j___ZN5boost4poolIN3RBX16roblox_allocatorEE12purge_memoryEv
pub fn stub_f4e7d4() {
    // IDA 0xf4e7d4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::pool<RBX::roblox_allocator>::ordered_malloc_need_resize(void)")]
// 0xf4e7e4 — j___ZN5boost4poolIN3RBX16roblox_allocatorEE26ordered_malloc_need_resizeEv
pub fn stub_f4e7e4() {
    // IDA 0xf4e7e4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::rehash_impl(unsigned long)")]
// 0xf4e7f4 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE11rehash_implEm
pub fn stub_f4e7f4() {
    // IDA 0xf4e7f4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::SpatialRegion::Id>>(RBX::SpatialRegion::Id const&,boost::unordered::detail::emplace_args1<RBX::SpatialRegion::Id> const&)")]
// 0xf4e804 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
pub fn stub_f4e804() {
    // IDA 0xf4e804: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf4e814 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE15place_in_bucketERNS1_5tableISB_EEPNS1_10ptr_bucketE
pub fn stub_f4e814() {
    // IDA 0xf4e814: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::rehash_impl(unsigned long)")]
// 0xf4e824 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11rehash_implEm
pub fn stub_f4e824() {
    // IDA 0xf4e824: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Primitive *>>(RBX::Primitive * const&,boost::unordered::detail::emplace_args1<RBX::Primitive *> const&)")]
// 0xf4e834 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
pub fn stub_f4e834() {
    // IDA 0xf4e834: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf4e844 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE15place_in_bucketERNS1_5tableISC_EEPNS1_10ptr_bucketE
pub fn stub_f4e844() {
    // IDA 0xf4e844: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>>::construct(void)")]
// 0xf4e854 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIN3RBX13SpatialRegion2IdEEEEE9constructEv
pub fn stub_f4e854() {
    // IDA 0xf4e854: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive *>>>::construct(void)")]
// 0xf4e864 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPN3RBX9PrimitiveEEEEE9constructEv
pub fn stub_f4e864() {
    // IDA 0xf4e864: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
// 0xf4e874 — j___ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12delete_nodesEPNS1_10ptr_bucketESE_
pub fn stub_f4e874() {
    // IDA 0xf4e874: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::create_buckets(unsigned long)")]
// 0xf4e884 — j___ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm
pub fn stub_f4e884() {
    // IDA 0xf4e884: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::delete_buckets(void)")]
// 0xf4e894 — j___ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14delete_bucketsEv
pub fn stub_f4e894() {
    // IDA 0xf4e894: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::reserve_for_insert(unsigned long)")]
// 0xf4e8a4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE18reserve_for_insertEm
pub fn stub_f4e8a4() {
    // IDA 0xf4e8a4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::clear(void)")]
// 0xf4e8b4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE5clearEv
pub fn stub_f4e8b4() {
    // IDA 0xf4e8b4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::table(unsigned long,RBX::SpatialRegion::Id::boost_compatible_hash_value const&,std::equal_to<RBX::SpatialRegion::Id> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>> const&)")]
// 0xf4e8c4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEEC2EmRKS8_RKSA_RKSaINS1_8ptr_nodeIS6_EEE
pub fn stub_f4e8c4() {
    // IDA 0xf4e8c4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
// 0xf4e8d4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12delete_nodesEPNS1_10ptr_bucketESF_
pub fn stub_f4e8d4() {
    // IDA 0xf4e8d4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::create_buckets(unsigned long)")]
// 0xf4e8e4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
pub fn stub_f4e8e4() {
    // IDA 0xf4e8e4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::delete_buckets(void)")]
// 0xf4e8f4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14delete_bucketsEv
pub fn stub_f4e8f4() {
    // IDA 0xf4e8f4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::reserve_for_insert(unsigned long)")]
// 0xf4e904 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
pub fn stub_f4e904() {
    // IDA 0xf4e904: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::clear(void)")]
// 0xf4e914 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE5clearEv
pub fn stub_f4e914() {
    // IDA 0xf4e914: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::table(unsigned long,boost::hash<RBX::Primitive *> const&,std::equal_to<RBX::Primitive *> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive *>> const&)")]
// 0xf4e924 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
pub fn stub_f4e924() {
    // IDA 0xf4e924: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::begin(void)const")]
// 0xf4e934 — j___ZNK3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE5beginEv
pub fn stub_f4e934() {
    // IDA 0xf4e934: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ExtentsInt32::contains(RBX::Vector3int32 const&)const")]
// 0xf4e944 — j___ZNK3RBX12ExtentsInt328containsERKNS_12Vector3int32E
pub fn stub_f4e944() {
    // IDA 0xf4e944: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "bool RBX::ContactManager::anyExtentsOverlapsOrTouchesPrimitives<RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>>(RBX::Extents const&,RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>> const&)const")]
// 0xf4e954 — j___ZNK3RBX14ContactManager37anyExtentsOverlapsOrTouchesPrimitivesINS_12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS4_EESaIS4_EEEEEbRKNS_7ExtentsERKT_
pub fn stub_f4e954() {
    // IDA 0xf4e954: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Extents::overlapsOrTouches(RBX::Extents const&)const")]
// 0xf4e974 — j___ZNK3RBX7Extents17overlapsOrTouchesERKS0_
pub fn stub_f4e974() {
    // IDA 0xf4e974: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::find_node_impl<RBX::SpatialRegion::Id,std::equal_to<RBX::SpatialRegion::Id>>(unsigned long,RBX::SpatialRegion::Id const&,std::equal_to<RBX::SpatialRegion::Id> const&)const")]
// 0xf4e984 — j___ZNK5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14find_node_implIS6_SA_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
pub fn stub_f4e984() {
    // IDA 0xf4e984: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::find_node_impl<RBX::Primitive *,std::equal_to<RBX::Primitive *>>(unsigned long,RBX::Primitive * const&,std::equal_to<RBX::Primitive *> const&)const")]
// 0xf4e994 — j___ZNK5boost9unordered6detail10table_implINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14find_node_implIS6_SB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
pub fn stub_f4e994() {
    // IDA 0xf4e994: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::min_buckets_for_size(unsigned long)const")]
// 0xf4e9a4 — j___ZNK5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE20min_buckets_for_sizeEm
pub fn stub_f4e9a4() {
    // IDA 0xf4e9a4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::min_buckets_for_size(unsigned long)const")]
// 0xf4e9b4 — j___ZNK5boost9unordered6detail5tableINS1_3setISaIPN3RBX9PrimitiveEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE20min_buckets_for_sizeEm
pub fn stub_f4e9b4() {
    // IDA 0xf4e9b4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::find(RBX::Primitive * const&)const")]
// 0xf4e9c4 — j___ZNKSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE4findERKS2_
pub fn stub_f4e9c4() {
    // IDA 0xf4e9c4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::_M_allocate(unsigned long)")]
// 0xf4e9d4 — j___ZNSt12_Vector_baseIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE11_M_allocateEm
pub fn stub_f4e9d4() {
    // IDA 0xf4e9d4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Primitive *,std::allocator<RBX::Primitive *>>::_M_allocate(unsigned long)")]
// 0xf4e9e4 — j___ZNSt12_Vector_baseIPN3RBX9PrimitiveESaIS2_EE11_M_allocateEm
pub fn stub_f4e9e4() {
    // IDA 0xf4e9e4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *>(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *)")]
// 0xf4e9f4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SpatialHashINS3_9PrimitiveENS3_7ContactENS3_14ContactManagerELi4EE21SpatialHashTableEntryESA_EET0_T_SC_SB_
pub fn stub_f4e9f4() {
    // IDA 0xf4e9f4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::set<RBX::Primitive * const*>(RBX::Primitive * const*,RBX::Primitive * const*)")]
// 0xf4ea04 — j___ZNSt3setIPN3RBX9PrimitiveESt4lessIS2_ESaIS2_EEC2IPKS2_EET_SA_
pub fn stub_f4ea04() {
    // IDA 0xf4ea04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::resize(unsigned long,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry)")]
// 0xf4ea14 — j___ZNSt6vectorIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE6resizeEmS6_
pub fn stub_f4ea14() {
    // IDA 0xf4ea14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_insert_unique(RBX::Primitive * const&)")]
// 0xf4ea24 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_f4ea24() {
    // IDA 0xf4ea24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_insert_unique(std::_Rb_tree_iterator<RBX::Primitive *>,RBX::Primitive * const&)")]
// 0xf4ea34 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_f4ea34() {
    // IDA 0xf4ea34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_erase(std::_Rb_tree_node<RBX::Primitive *> *)")]
// 0xf4ea44 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_f4ea44() {
    // IDA 0xf4ea44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Primitive * const&)")]
// 0xf4ea54 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_f4ea54() {
    // IDA 0xf4ea54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>> std::for_each<boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>)")]
// 0xf4ea74 — j___ZSt8for_eachIN5boost9unordered15iterator_detail10c_iteratorINS1_6detail8ptr_nodeIPN3RBX9PrimitiveEEEPKS9_EENS0_3_bi6bind_tIvNS0_4_mfi3mf4IvNS6_14ContactManagerES8_bbbEENSD_5list5INSD_5valueIPSH_EENS0_3argILi1EEENSK_IbEESP_SP_EEEEET0_T_ST_SS_
pub fn stub_f4ea74() {
    // IDA 0xf4ea74: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::returnNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// 0xf4ea94 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE10returnNodeEPNS4_11SpatialNodeE
pub fn stub_f4ea94() {
    // IDA 0xf4ea94: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::destroyNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// 0xf4eaa4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11destroyNodeEPNS4_11SpatialNodeE
pub fn stub_f4eaa4() {
    // IDA 0xf4eaa4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findTreeNode(int,int,RBX::Vector3int32 const&)")]
// 0xf4eab4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12findTreeNodeEiiRKNS_12Vector3int32E
pub fn stub_f4eab4() {
    // IDA 0xf4eab4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::retireTreeNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
// 0xf4eac4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14retireTreeNodeEPNS4_8TreeNodeE
pub fn stub_f4eac4() {
    // IDA 0xf4eac4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAssembled(RBX::Primitive*)")]
// 0xf4ead4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE20onPrimitiveAssembledEPS1_
pub fn stub_f4ead4() {
    // IDA 0xf4ead4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesTouchingGrids(RBX::Extents const&,RBX::Primitive const*,unsigned long,boost::unordered::unordered_set<RBX::Primitive*,boost::hash<RBX::Primitive*>,std::equal_to<RBX::Primitive*>,std::allocator<RBX::Primitive*>> &)")]
// 0xf4eae4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26getPrimitivesTouchingGridsERKNS_7ExtentsEPKS1_mRN5boost9unordered13unordered_setIPS1_NSA_4hashISD_EESt8equal_toISD_ESaISD_EEE
pub fn stub_f4eae4() {
    // IDA 0xf4eae4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHash(RBX::World *,RBX::ContactManager*,int)")]
// 0xf4eaf4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EEC2EPNS_5WorldEPS3_i
pub fn stub_f4eaf4() {
    // IDA 0xf4eaf4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Extents::zero(void)")]
// 0xf4eb04 — j___ZN3RBX7Extents4zeroEv
pub fn stub_f4eb04() {
    // IDA 0xf4eb04: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>,RBX::Primitive*>,boost::_bi::list2<boost::_bi::value<RBX::ContactManagerSpatialHash *>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>,RBX::Primitive*>,boost::_bi::list2<boost::_bi::value<RBX::ContactManagerSpatialHash *>,boost::arg<1>>>,RBX::Primitive*)")]
// 0xf4eb14 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EEEPS8_EENS3_5list2INS3_5valueIPNS_25ContactManagerSpatialHashEEENS2_3argILi1EEEEEEEEEvT_SC_
pub fn stub_f4eb14() {
    // IDA 0xf4eb14: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::operator delete(void *)")]
// 0xf4eb24 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEdlEPv
pub fn stub_f4eb24() {
    // IDA 0xf4eb24: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::simple_segregated_storage<unsigned long>::ordered_free(void *)")]
// 0xf4eb34 — j___ZN5boost25simple_segregated_storageImE12ordered_freeEPv
pub fn stub_f4eb34() {
    // IDA 0xf4eb34: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::ExtentsInt32::overlapsOrTouches(RBX::ExtentsInt32 const&)const")]
// 0xf4eb44 — j___ZNK3RBX12ExtentsInt3217overlapsOrTouchesERKS0_
pub fn stub_f4eb44() {
    // IDA 0xf4eb44: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::BasicSpatialHashPrimitive::getSpatialNodeLevel(void)const")]
// 0xf4eb54 — j___ZNK3RBX25BasicSpatialHashPrimitive19getSpatialNodeLevelEv
pub fn stub_f4eb54() {
    // IDA 0xf4eb54: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::Extents::operator==(RBX::Extents const&)const")]
// 0xf4eb64 — j___ZNK3RBX7ExtentseqERKS0_
pub fn stub_f4eb64() {
    // IDA 0xf4eb64: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "std::_Vector_base<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::_M_allocate(unsigned long)")]
// 0xf4eb74 — j___ZNSt12_Vector_baseIN3RBX8NodeInfoESaIS1_EE11_M_allocateEm
pub fn stub_f4eb74() {
    // IDA 0xf4eb74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::NodeInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::NodeInfo *,RBX::NodeInfo *>(RBX::NodeInfo *,RBX::NodeInfo *,RBX::NodeInfo *)")]
// 0xf4eb84 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NodeInfoES5_EET0_T_S7_S6_
pub fn stub_f4eb84() {
    // IDA 0xf4eb84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry*,std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>>,unsigned long,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry const&)")]
// 0xf4eb94 — j___ZNSt6vectorIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS6_S8_EEmRKS6_
pub fn stub_f4eb94() {
    // IDA 0xf4eb94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::NodeInfo*,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,RBX::NodeInfo const&)")]
// 0xf4eba4 — j___ZNSt6vectorIN3RBX8NodeInfoESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_f4eba4() {
    // IDA 0xf4eba4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::NodeInfo *,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,int,RBX::NodeInfo,std::less<RBX::NodeInfo>>(__gnu_cxx::__normal_iterator<RBX::NodeInfo *,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,int,int,RBX::NodeInfo,std::less<RBX::NodeInfo>)")]
// 0xf4ebb4 — j___ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX8NodeInfoESt6vectorIS3_SaIS3_EEEEiS3_St4lessIS3_EEvT_T0_SC_T1_T2_
pub fn stub_f4ebb4() {
    // IDA 0xf4ebb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::NodeInfo *,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,int,RBX::NodeInfo,std::less<RBX::NodeInfo>>(__gnu_cxx::__normal_iterator<RBX::NodeInfo *,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,int,int,RBX::NodeInfo,std::less<RBX::NodeInfo>)")]
// 0xf4ebc4 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX8NodeInfoESt6vectorIS3_SaIS3_EEEEiS3_St4lessIS3_EEvT_T0_SC_T1_T2_
pub fn stub_f4ebc4() {
    // IDA 0xf4ebc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::_M_allocate(unsigned long)")]
// 0xf4ebe4 — j___ZNSt12_Vector_baseIN3RBX16LegacyController9InputTypeESaIS2_EE11_M_allocateEm
pub fn stub_f4ebe4() {
    // IDA 0xf4ebe4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::LegacyController::InputType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::LegacyController::InputType *,RBX::LegacyController::InputType *>(RBX::LegacyController::InputType *,RBX::LegacyController::InputType *,RBX::LegacyController::InputType *)")]
// 0xf4ebf4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16LegacyController9InputTypeES6_EET0_T_S8_S7_
pub fn stub_f4ebf4() {
    // IDA 0xf4ebf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
