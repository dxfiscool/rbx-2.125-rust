//! core shard E — 100 boost core stubs EA-sorted, continuation after shard D (0x6dea08).
//! Source: ida/export.json filtered where mangled/demangled contains "boost" (and crate core, Reflection/Instance/Ogre/RakNet/Network/Lua excluded), EA-sorted, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::Kernel>,boost::_bi::list1<boost::_bi::value<RBX::Kernel const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x6dea24 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS3_5list1INS3_5valueIPKS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::Kernel>,boost::_bi::list1<boost::_bi::value<RBX::Kernel const*>>>>::manage(boost::detail::function::function_
pub fn stub_0x6dea24() {
    // IDA 0x6dea24: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::Kernel>,boost::_bi::list1<boost::_bi::value<RBX::Kernel const*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
// 0x6dea84 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS3_5list1INS3_5valueIPKS8_EEEEEEiE6invokeERNS1_15function_bufferE — boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::Kernel>,boost::_bi::list1<boost::_bi::value<RBX::Kernel const*>>>,int>::invoke(boost::detail::function:
pub fn stub_0x6dea84() {
    // IDA 0x6dea84: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::Kernel>,boost::_bi::list1<boost::_bi::value<RBX::Kernel const*>>>::operator()(void)")]
// 0x6dea88 — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX6KernelEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv — boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::Kernel>,boost::_bi::list1<boost::_bi::value<RBX::Kernel const*>>>::operator()(void)
pub fn stub_0x6dea88() {
    // IDA 0x6dea88: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf1<int,RBX::World,RBX::IWorldStage::MetricType>,boost::_bi::list2<boost::_bi::value<RBX::World const*>,boost::_bi::value<RBX::IWorldStage::MetricType>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x6deaa0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf1IiN3RBX5WorldENS7_11IWorldStage10MetricTypeEEENS3_5list2INS3_5valueIPKS8_EENSD_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf1<int,RBX::World,RBX::IWorldStage::MetricType>,boost::_bi::list2<boost::_bi::value<RBX::World const*>,boost::_bi::value<
pub fn stub_0x6deaa0() {
    // IDA 0x6deaa0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf1<int,RBX::World,RBX::IWorldStage::MetricType>,boost::_bi::list2<boost::_bi::value<RBX::World const*>,boost::_bi::value<RBX::IWorldStage::MetricType>>>,int>::invoke(boost::detail::function::function_buffer &)")]
// 0x6deb18 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf1IiN3RBX5WorldENS7_11IWorldStage10MetricTypeEEENS3_5list2INS3_5valueIPKS8_EENSD_ISA_EEEEEEiE6invokeERNS1_15function_bufferE — boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf1<int,RBX::World,RBX::IWorldStage::MetricType>,boost::_bi::list2<boost::_bi::value<RBX::World const*>,boost::_bi::
pub fn stub_0x6deb18() {
    // IDA 0x6deb18: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf1<int,RBX::World,RBX::IWorldStage::MetricType>,boost::_bi::list2<boost::_bi::value<RBX::World const*>,boost::_bi::value<RBX::IWorldStage::MetricType>>>::operator()(void)")]
// 0x6deb20 — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf1IiN3RBX5WorldENS4_11IWorldStage10MetricTypeEEENS0_5list2INS0_5valueIPKS5_EENSA_IS7_EEEEEclEv — boost::_bi::bind_t<int,boost::_mfi::cmf1<int,RBX::World,RBX::IWorldStage::MetricType>,boost::_bi::list2<boost::_bi::value<RBX::World const*>,boost::_bi::value<RBX::IWorldStage::MetricType>>>::operator
pub fn stub_0x6deb20() {
    // IDA 0x6deb20: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function0<int>::clear(void)")]
// 0x6deb38 — __ZN5boost9function0IiE5clearEv — boost::function0<int>::clear(void)
pub fn stub_0x6deb38() {
    // IDA 0x6deb38: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x6deb64 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX5WorldEEENS3_5list1INS3_5valueIPKS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>>::manage(boost::detail::function::function_bu
pub fn stub_0x6deb64() {
    // IDA 0x6deb64: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
// 0x6debc4 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX5WorldEEENS3_5list1INS3_5valueIPKS8_EEEEEEiE6invokeERNS1_15function_bufferE — boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>,int>::invoke(boost::detail::function::f
pub fn stub_0x6debc4() {
    // IDA 0x6debc4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>::operator()(void)")]
// 0x6debc8 — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX5WorldEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv — boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>::operator()(void)
pub fn stub_0x6debc8() {
    // IDA 0x6debc8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function0<int>::operator()(void)const")]
// 0x6df2a0 — __ZNK5boost9function0IiEclEv — boost::function0<int>::operator()(void)const
pub fn stub_0x6df2a0() {
    // IDA 0x6df2a0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<float,boost::_mfi::cmf0<float,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x6df524 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIfNS_4_mfi4cmf0IfN3RBX5WorldEEENS3_5list1INS3_5valueIPKS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<boost::_bi::bind_t<float,boost::_mfi::cmf0<float,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>>::manage(boost::detail::function::functio
pub fn stub_0x6df524() {
    // IDA 0x6df524: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<float,boost::_mfi::cmf0<float,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>,float>::invoke(boost::detail::function::function_buffer &)")]
// 0x6df584 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIfNS_4_mfi4cmf0IfN3RBX5WorldEEENS3_5list1INS3_5valueIPKS8_EEEEEEfE6invokeERNS1_15function_bufferE — boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<float,boost::_mfi::cmf0<float,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>,float>::invoke(boost::detail::funct
pub fn stub_0x6df584() {
    // IDA 0x6df584: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<float,boost::_mfi::cmf0<float,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>::operator()(void)")]
// 0x6df588 — __ZN5boost3_bi6bind_tIfNS_4_mfi4cmf0IfN3RBX5WorldEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv — boost::_bi::bind_t<float,boost::_mfi::cmf0<float,RBX::World>,boost::_bi::list1<boost::_bi::value<RBX::World const*>>>::operator()(void)
pub fn stub_0x6df588() {
    // IDA 0x6df588: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function0<float>::assign_to_own(boost::function0<float> const&)")]
// 0x6df6ac — __ZN5boost9function0IfE13assign_to_ownERKS1_ — boost::function0<float>::assign_to_own(boost::function0<float> const&)
pub fn stub_0x6df6ac() {
    // IDA 0x6df6ac: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RBX::Stats::TypedStatsItem<float>::TypedStatsItem(boost::function0<float>)")]
// 0x6df6dc — __ZN3RBX5Stats14TypedStatsItemIfEC2EN5boost9function0IfEE — RBX::Stats::TypedStatsItem<float>::TypedStatsItem(boost::function0<float>)
pub fn stub_0x6df6dc() {
    // IDA 0x6df6dc: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function0<double>::clear(void)")]
// 0x6dfb8c — __ZN5boost9function0IdE5clearEv — boost::function0<double>::clear(void)
pub fn stub_0x6dfb8c() {
    // IDA 0x6dfb8c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::RunService>,boost::_bi::list1<boost::_bi::value<RBX::RunService const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x6dfbb8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX10RunServiceEEENS3_5list1INS3_5valueIPKS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::RunService>,boost::_bi::list1<boost::_bi::value<RBX::RunService const*>>>>::manage(boost::detail::funct
pub fn stub_0x6dfbb8() {
    // IDA 0x6dfbb8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::RunService>,boost::_bi::list1<boost::_bi::value<RBX::RunService const*>>>,double>::invoke(boost::detail::function::function_buffer &)")]
// 0x6dfc18 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX10RunServiceEEENS3_5list1INS3_5valueIPKS8_EEEEEEdE6invokeERNS1_15function_bufferE — boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::RunService>,boost::_bi::list1<boost::_bi::value<RBX::RunService const*>>>,double>::invoke(boost::
pub fn stub_0x6dfc18() {
    // IDA 0x6dfc18: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::RunService>,boost::_bi::list1<boost::_bi::value<RBX::RunService const*>>>::operator()(void)")]
// 0x6dfc1c — __ZN5boost3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX10RunServiceEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv — boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::RunService>,boost::_bi::list1<boost::_bi::value<RBX::RunService const*>>>::operator()(void)
pub fn stub_0x6dfc1c() {
    // IDA 0x6dfc1c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function0<double>::assign_to_own(boost::function0<double> const&)")]
// 0x6dfd40 — __ZN5boost9function0IdE13assign_to_ownERKS1_ — boost::function0<double>::assign_to_own(boost::function0<double> const&)
pub fn stub_0x6dfd40() {
    // IDA 0x6dfd40: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RBX::Stats::TypedStatsItem<double>::TypedStatsItem(boost::function0<double>)")]
// 0x6dfd70 — __ZN3RBX5Stats14TypedStatsItemIdEC2EN5boost9function0IdEE — RBX::Stats::TypedStatsItem<double>::TypedStatsItem(boost::function0<double>)
pub fn stub_0x6dfd70() {
    // IDA 0x6dfd70: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::disconnectAll(void)")]
// 0x6e1148 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE13disconnectAllEv — rbx::signals::signal<void ()(RBX::TouchPair const&)>::disconnectAll(void)
pub fn stub_0x6e1148() {
    // IDA 0x6e1148: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::Cofm,60u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x6e3a98 — __ZN5boost14singleton_poolIN3RBX4CofmELj60ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::Cofm,60u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x6e3a98() {
    // IDA 0x6e3a98: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::Cofm,60u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x6e3ad0 — __ZN5boost14singleton_poolIN3RBX4CofmELj60ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::Cofm,60u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_0x6e3ad0() {
    // IDA 0x6e3ad0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::SimBody,308u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x6e3b1c — __ZN5boost14singleton_poolIN3RBX7SimBodyELj308ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::SimBody,308u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_0x6e3b1c() {
    // IDA 0x6e3b1c: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::SimBody,308u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x6e3b6c — __ZN5boost14singleton_poolIN3RBX7SimBodyELj308ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::SimBody,308u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x6e3b6c() {
    // IDA 0x6e3b6c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::Body,276u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x6e3bc0 — __ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::Body,276u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x6e3bc0() {
    // IDA 0x6e3bc0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::Cofm,60u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x6e43f0 — __ZN5boost14singleton_poolIN3RBX4CofmELj60ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::Cofm,60u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x6e43f0() {
    // IDA 0x6e43f0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::SimBody,308u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x6f2698 — __ZN5boost14singleton_poolIN3RBX7SimBodyELj308ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::SimBody,308u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x6f2698() {
    // IDA 0x6f2698: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET__0")]
// 0x702f60 — __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET__0 — __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrINS_8InstanceEEEEEvNS2_8functionIFvNS0_7VariantEEEET__0
pub fn stub_0x702f60() {
    // IDA 0x702f60: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::FWInstance,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x704e78 — __ZN5boost14singleton_poolIN3RBX10FWInstanceELj28ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::FWInstance,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x704e78() {
    // IDA 0x704e78: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x704eac — __ZN5boost14singleton_poolIN3RBX16OnDemandInstanceELj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x704eac() {
    // IDA 0x704eac: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::FWInstance,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x705634 — __ZN5boost14singleton_poolIN3RBX10FWInstanceELj28ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::FWInstance,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x705634() {
    // IDA 0x705634: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x706260 — __ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x706260() {
    // IDA 0x706260: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvPKN3RBX10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS9_5list2INS9_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// 0x706d04 — __ZN5boost8functionIFvPKN3RBX10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS9_5list2INS9_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE — __ZN5boost8functionIFvPKN3RBX10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS9_5list2INS9_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_1
pub fn stub_0x706d04() {
    // IDA 0x706d04: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0x706de8 — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE — __ZN5boost9function1IvPKN3RBX10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11
pub fn stub_0x706de8() {
    // IDA 0x706de8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEES4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SF_EENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// 0x70850c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEES4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SF_EENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEES4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SF_EENS8_5list3INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSL_ILi2EEEE
pub fn stub_0x70850c() {
    // IDA 0x70850c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0x7085f0 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEES4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_SE_EENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSK_ILi2EEEEE
pub fn stub_0x7085f0() {
    // IDA 0x7085f0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS2_10Reflection7VariantEEEES4_ENS8_5list2INS8_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0x70b534 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS2_10Reflection7VariantEEEES4_ENS8_5list2INS8_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS2_10Reflection7VariantEEEES4_ENS8_5list2INS8_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_
pub fn stub_0x70b534() {
    // IDA 0x70b534: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS2_10Reflection7VariantEEEES4_ENS7_5list2INS7_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0x70b608 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS2_10Reflection7VariantEEEES4_ENS7_5list2INS7_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS2_10Reflection7VariantEEEES4_ENS7_5list2INS7_5valueISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost
pub fn stub_0x70b608() {
    // IDA 0x70b608: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::function1<void,std::string>::assign_to_own(boost::function1<void,std::string> const&)")]
// 0x70e698 — __ZN5boost9function1IvSsE13assign_to_ownERKS1_ — boost::function1<void,std::string>::assign_to_own(boost::function1<void,std::string> const&)
pub fn stub_0x70e698() {
    // IDA 0x70e698: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<XmlElement,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x70e6c8 — __ZN5boost14singleton_poolI10XmlElementLj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<XmlElement,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x70e6c8() {
    // IDA 0x70e6c8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FWBase>(RBX::FWBase *)")]
// 0x70e8e8 — __ZN5boost6detail12shared_countC2IN3RBX6FWBaseEEEPT_ — boost::detail::shared_count::shared_count<RBX::FWBase>(RBX::FWBase *)
pub fn stub_0x70e8e8() {
    // IDA 0x70e8e8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::FWBase>::~sp_counted_impl_p()")]
// 0x70e9e0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6FWBaseEED1Ev — boost::detail::sp_counted_impl_p<RBX::FWBase>::~sp_counted_impl_p()
pub fn stub_0x70e9e0() {
    // IDA 0x70e9e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::FWBase>::~sp_counted_impl_p()")]
// 0x70e9e4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6FWBaseEED0Ev — boost::detail::sp_counted_impl_p<RBX::FWBase>::~sp_counted_impl_p()
pub fn stub_0x70e9e4() {
    // IDA 0x70e9e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::FWBase>::dispose(void)")]
// 0x70e9e8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6FWBaseEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::FWBase>::dispose(void)
pub fn stub_0x70e9e8() {
    // IDA 0x70e9e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::FWBase>::get_deleter(std::type_info const&)")]
// 0x70e9f8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6FWBaseEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<RBX::FWBase>::get_deleter(std::type_info const&)
pub fn stub_0x70e9f8() {
    // IDA 0x70e9f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,bool),boost::_bi::list2<boost::arg<1>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,bool),boost::_bi::list2<boost::arg<1>,boost::_bi::value<bool>>>,RBX::Primitive *)")]
// 0x717cac — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveEbENS3_5list2INS2_3argILi1EEENS3_5valueIbEEEEEEEEvT_S6_ — void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,bool),boost::_bi::list2<boost::arg<1>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primiti
pub fn stub_0x717cac() {
    // IDA 0x717cac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Vector3 const&,float &),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<G3D::Vector3 const>,boost::reference_wrapper<float>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Vector3 const&,float &),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<G3D::Vector3 const>,boost::reference_wrapper<float>>>,RBX::Primitive *)")]
// 0x717d04 — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERKN3G3D7Vector3ERfENS3_5list3INS2_3argILi1EEENS2_17reference_wrapperIS9_EENSH_IfEEEEEEEEvT_S6_ — void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Vector3 const&,float &),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<G3D::Vector3 const>,boos
pub fn stub_0x717d04() {
    // IDA 0x717d04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Assembly,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Assembly*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Assembly,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Assembly*>,boost::arg<1>>>,RBX::Primitive *)")]
// 0x717d64 — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvS0_PNS_9PrimitiveEEENS3_5list2INS3_5valueIPS0_EENS2_3argILi1EEEEEEEEEvT_S8_ — void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Assembly,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Assembly*>,boost::arg<1>>>>(boost::_bi::b
pub fn stub_0x717d64() {
    // IDA 0x717d64: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::BallPolyContact,212u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x71b054 — __ZN5boost14singleton_poolIN3RBX15BallPolyContactELj212ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::BallPolyContact,212u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x71b054() {
    // IDA 0x71b054: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token> const&)")]
// 0x71c0b8 — __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY9BlockMeshENS1_15Vector3ComparerEE5TokenEEaSERKSA_ — rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Com
pub fn stub_0x71c0b8() {
    // IDA 0x71c0b8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token> const&)")]
// 0x71c254 — __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY12BlockCornersENS1_15Vector3ComparerEE5TokenEEaSERKSA_ — rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vect
pub fn stub_0x71c254() {
    // IDA 0x71c254: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token *)")]
// 0x71c5ec — __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY12BlockCornersENS1_15Vector3ComparerEE5TokenEEC2IS9_EEPT_ — rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token
pub fn stub_0x71c5ec() {
    // IDA 0x71c5ec: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token *)")]
// 0x71c6c0 — __ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolIN3G3D7Vector3ENS3_4POLY12BlockCornersENS3_15Vector3ComparerEE5TokenEEEPT_ — boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Compar
pub fn stub_0x71c6c0() {
    // IDA 0x71c6c0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()")]
// 0x71cbb8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY12BlockCornersENS2_15Vector3ComparerEE5TokenEED1Ev — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()
pub fn stub_0x71cbb8() {
    // IDA 0x71cbb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()")]
// 0x71cbbc — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY12BlockCornersENS2_15Vector3ComparerEE5TokenEED0Ev — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()
pub fn stub_0x71cbbc() {
    // IDA 0x71cbbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::dispose(void)")]
// 0x71cbc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY12BlockCornersENS2_15Vector3ComparerEE5TokenEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::dispose(void)
pub fn stub_0x71cbc0() {
    // IDA 0x71cbc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::get_deleter(std::type_info const&)")]
// 0x71cc68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY12BlockCornersENS2_15Vector3ComparerEE5TokenEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::get_deleter(std::type_info const&)
pub fn stub_0x71cc68() {
    // IDA 0x71cc68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::get_untyped_deleter(void)")]
// 0x71cc6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY12BlockCornersENS2_15Vector3ComparerEE5TokenEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::Token>::get_untyped_deleter(void)
pub fn stub_0x71cc6c() {
    // IDA 0x71cc6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x71d1cc — __ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x71d1cc() {
    // IDA 0x71d1cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x71d1fc — __ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x71d1fc() {
    // IDA 0x71d1fc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token *)")]
// 0x71d510 — __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY9BlockMeshENS1_15Vector3ComparerEE5TokenEEC2IS9_EEPT_ — rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>(RBX:
pub fn stub_0x71d510() {
    // IDA 0x71d510: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token *)")]
// 0x71d5e4 — __ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolIN3G3D7Vector3ENS3_4POLY9BlockMeshENS3_15Vector3ComparerEE5TokenEEEPT_ — boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::T
pub fn stub_0x71d5e4() {
    // IDA 0x71d5e4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()")]
// 0x71dc18 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY9BlockMeshENS2_15Vector3ComparerEE5TokenEED1Ev — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()
pub fn stub_0x71dc18() {
    // IDA 0x71dc18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()")]
// 0x71dc1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY9BlockMeshENS2_15Vector3ComparerEE5TokenEED0Ev — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()
pub fn stub_0x71dc1c() {
    // IDA 0x71dc1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::dispose(void)")]
// 0x71dc20 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY9BlockMeshENS2_15Vector3ComparerEE5TokenEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::dispose(void)
pub fn stub_0x71dc20() {
    // IDA 0x71dc20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::get_deleter(std::type_info const&)")]
// 0x71dcc8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY9BlockMeshENS2_15Vector3ComparerEE5TokenEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::get_deleter(std::type_info const&)
pub fn stub_0x71dcc8() {
    // IDA 0x71dcc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::get_untyped_deleter(void)")]
// 0x71dccc — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY9BlockMeshENS2_15Vector3ComparerEE5TokenEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::Token>::get_untyped_deleter(void)
pub fn stub_0x71dccc() {
    // IDA 0x71dccc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x71e2b0 — __ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x71e2b0() {
    // IDA 0x71e2b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x71e2e0 — __ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x71e2e0() {
    // IDA 0x71e2e0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::BlockBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x723c68 — __ZN5boost14singleton_poolIN3RBX17BlockBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::BlockBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x723c68() {
    // IDA 0x723c68: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x723c98 — __ZN5boost14singleton_poolIN3RBX16GeoPairConnectorELj264ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x723c98() {
    // IDA 0x723c98: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x723cd0 — __ZN5boost14singleton_poolIN3RBX18BallBlockConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x723cd0() {
    // IDA 0x723cd0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x723d08 — __ZN5boost14singleton_poolIN3RBX17BallBallConnectorELj272ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x723d08() {
    // IDA 0x723d08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x724034 — __ZN5boost14singleton_poolIN3RBX16GeoPairConnectorELj264ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x724034() {
    // IDA 0x724034: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x7240e4 — __ZN5boost14singleton_poolIN3RBX18BallBlockConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x7240e4() {
    // IDA 0x7240e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x724194 — __ZN5boost14singleton_poolIN3RBX17BallBallConnectorELj272ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv — boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_0x724194() {
    // IDA 0x724194: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::getPrimitivesTouchingExtentsIgnoreAncestor(RBX::Extents const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,int,G3D::Array<RBX::Primitive*,10,32ul> &)")]
// 0x72520c — __ZN3RBX14ContactManager42getPrimitivesTouchingExtentsIgnoreAncestorERKNS_7ExtentsERKN5boost9unordered13unordered_setIPKNS_9PrimitiveENS4_4hashIS9_EESt8equal_toIS9_ESaIS9_EEEiRN3G3D5ArrayIPS7_Li10ELm32EEE — RBX::ContactManager::getPrimitivesTouchingExtentsIgnoreAncestor(RBX::Extents const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitiv
pub fn stub_0x72520c() {
    // IDA 0x72520c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::begin(void)const")]
// 0x727a94 — __ZNK3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE5beginEv — RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::begin(void)const
pub fn stub_0x727a94() {
    // IDA 0x727a94: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::const_iterator::operator++(void)")]
// 0x727acc — __ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE14const_iteratorppEv — RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::const_iterator::operator++(void)
pub fn stub_0x727acc() {
    // IDA 0x727acc: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Primitive *,RBX::Extents const*,G3D::Array<RBX::Primitive *,10,32ul> *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::Extents const*>,boost::_bi::value<G3D::Array<RBX::Primitive *,10,32ul> *>>> std::for_each<boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::_bi::bind_t<void,void (*)(RBX::Primitive *,RBX::Extents const*,G3D::Array<RBX::Primitive *,10,32ul> *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::Extents const*>,boost::_bi::value<G3D::Array<RBX::Primitive *,10,32ul> *>>>>(boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::_bi::bind_t<void,void (*)(RBX::Primitive *,RBX::Extents const*,G3D::Array<RBX::Primitive *,10,32ul> *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::Extents const*>,boost::_bi::value<G3D::Array<RBX::Primitive *,10,32ul> *>>>)")]
// 0x727b04 — __ZSt8for_eachIN5boost9unordered15iterator_detail10c_iteratorINS1_6detail8ptr_nodeIPN3RBX9PrimitiveEEEPKS9_EENS0_3_bi6bind_tIvPFvS8_PKNS6_7ExtentsEPN3G3D5ArrayIS8_Li10ELm32EEEENSD_5list3INS0_3argILi1EEENSD_5valueISH_EENSR_ISL_EEEEEEET0_T_SX_SW_ — boost::_bi::bind_t<void,void (*)(RBX::Primitive *,RBX::Extents const*,G3D::Array<RBX::Primitive *,10,32ul> *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::Extents const*>,boost::_bi::value<G
pub fn stub_0x727b04() {
    // IDA 0x727b04: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesTouchingGrids(RBX::Extents const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,unsigned long,boost::unordered::unordered_set<RBX::Primitive*,boost::hash<RBX::Primitive*>,std::equal_to<RBX::Primitive*>,std::allocator<RBX::Primitive*>> &)")]
// 0x727b44 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26getPrimitivesTouchingGridsERKNS_7ExtentsERKN5boost9unordered13unordered_setIPKS1_NS8_4hashISC_EESt8equal_toISC_ESaISC_EEEmRNSA_IPS1_NSD_ISL_EESF_ISL_ESaISL_EEE — RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesTouchingGrids(RBX::Extents const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive con
pub fn stub_0x727b44() {
    // IDA 0x727b44: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "bool RBX::ContactManager::anyExtentsOverlapsOrTouchesPrimitives<RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>>(RBX::Extents const&,RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>> const&)const")]
// 0x7283d4 — __ZNK3RBX14ContactManager37anyExtentsOverlapsOrTouchesPrimitivesINS_12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS4_EESaIS4_EEEEEbRKNS_7ExtentsERKT_ — bool RBX::ContactManager::anyExtentsOverlapsOrTouchesPrimitives<RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>>(RBX::Extents const&,RBX::DenseHashSe
pub fn stub_0x7283d4() {
    // IDA 0x7283d4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>> std::for_each<boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>)")]
// 0x729bd8 — __ZSt8for_eachIN5boost9unordered15iterator_detail10c_iteratorINS1_6detail8ptr_nodeIPN3RBX9PrimitiveEEEPKS9_EENS0_3_bi6bind_tIvNS0_4_mfi3mf4IvNS6_14ContactManagerES8_bbbEENSD_5list5INSD_5valueIPSH_EENS0_3argILi1EEENSK_IbEESP_SP_EEEEET0_T_ST_SS_ — boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost
pub fn stub_0x729bd8() {
    // IDA 0x729bd8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::clear(void)")]
// 0x729c3c — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE5clearEv — boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::Spati
pub fn stub_0x729c3c() {
    // IDA 0x729c3c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
// 0x729c6c — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12delete_nodesEPNS1_10ptr_bucketESE_ — boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::Spati
pub fn stub_0x729c6c() {
    // IDA 0x729c6c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::ContactManager *>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::operator()<boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list1<RBX::Primitive * const&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool> &,boost::_bi::list1<RBX::Primitive * const&> &,int)")]
// 0x729d8c — __ZN5boost3_bi5list5INS0_5valueIPN3RBX14ContactManagerEEENS_3argILi1EEENS2_IbEES9_S9_EclINS_4_mfi3mf4IvS4_PNS3_9PrimitiveEbbbEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i — void boost::_bi::list5<boost::_bi::value<RBX::ContactManager *>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::operator()<boost::_mfi::mf4<void,RBX::ContactMan
pub fn stub_0x729d8c() {
    // IDA 0x729d8c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::SpatialRegion::Id>>(RBX::SpatialRegion::Id const&,boost::unordered::detail::emplace_args1<RBX::SpatialRegion::Id> const&)")]
// 0x729dd8 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_ — std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocato
pub fn stub_0x729dd8() {
    // IDA 0x729dd8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::reserve_for_insert(unsigned long)")]
// 0x729f7c — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE18reserve_for_insertEm — boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::Spati
pub fn stub_0x729f7c() {
    // IDA 0x729f7c: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::create_buckets(unsigned long)")]
// 0x729fd0 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm — boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::Spati
pub fn stub_0x729fd0() {
    // IDA 0x729fd0: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::min_buckets_for_size(unsigned long)const")]
// 0x72a0f8 — __ZNK5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE20min_buckets_for_sizeEm — boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::Spati
pub fn stub_0x72a0f8() {
    // IDA 0x72a0f8: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::rehash_impl(unsigned long)")]
// 0x72a188 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE11rehash_implEm — boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::
pub fn stub_0x72a188() {
    // IDA 0x72a188: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x72a1b4 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE15place_in_bucketERNS1_5tableISB_EEPNS1_10ptr_bucketE — boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::
pub fn stub_0x72a1b4() {
    // IDA 0x72a1b4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>>::construct(void)")]
// 0x72a20c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIN3RBX13SpatialRegion2IdEEEEE9constructEv — boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>>::construct(void)
pub fn stub_0x72a20c() {
    // IDA 0x72a20c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::find_node_impl<RBX::SpatialRegion::Id,std::equal_to<RBX::SpatialRegion::Id>>(unsigned long,RBX::SpatialRegion::Id const&,std::equal_to<RBX::SpatialRegion::Id> const&)const")]
// 0x72a244 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14find_node_implIS6_SA_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_ — boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRe
pub fn stub_0x72a244() {
    // IDA 0x72a244: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x72a2c0 — __ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_0x72a2c0() {
    // IDA 0x72a2c0: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x72a30c — __ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv — boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_0x72a30c() {
    // IDA 0x72a30c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x72a344 — __ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv — boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)
pub fn stub_0x72a344() {
    // IDA 0x72a344: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}
