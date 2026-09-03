//! Auto-generated refl 31 — 120 stubs EA-sorted asc 0x37d98c..0x385fe4 (global gap filler, RBX::Reflection exhausted 19829/19829, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) global EA asc not yet in crates/reflection/src — next 120 uncovered (RBX::Reflection filter yielded 0 remaining, fallback to global gap)
//! Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x37d98c — __ZN3RBX9CreatableINS_8InstanceEE6createI21SoundServiceStatsItemPKNS_10Soundscape12SoundServiceEEEN5boost10shared_ptrIT_EET0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createI21SoundServiceStatsItemPKNS_10Soundscape12SoundServiceEEEN5boost10shared_ptrIT_EET0_")]
pub fn stub_37d98c() -> ! {
    todo!("0x37d98c boost::shared_ptr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)")
}

// 0x37da40 — __ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_
#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int>(char const*,int const&)")]
#[doc(alias = "__ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_")]
pub fn stub_37da40() -> ! {
    todo!("0x37da40 RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int>(char const*,int const&)")
}

// 0x37dd20 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")]
pub fn stub_37dd20() -> ! {
    todo!("0x37dd20 boost::detail::function::functor_manager<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x37dd80 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEiE6invokeERNS1_15function_bufferE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEiE6invokeERNS1_15function_bufferE")]
pub fn stub_37dd80() -> ! {
    todo!("0x37dd80 boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>,int>::invoke(boost::detail::function::function_buffer &)")
}

// 0x37de98 — __ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE
#[doc(alias = "SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")]
#[doc(alias = "__ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE")]
pub fn stub_37de98() -> ! {
    todo!("0x37de98 SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")
}

// 0x37e05c — __ZN21SoundServiceStatsItemD1Ev
#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
#[doc(alias = "__ZN21SoundServiceStatsItemD1Ev")]
pub fn stub_37e05c() {
    // IDA 0x37e05c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x37e098 — __ZN21SoundServiceStatsItemD0Ev
#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
#[doc(alias = "__ZN21SoundServiceStatsItemD0Ev")]
pub fn stub_37e098() {
    // IDA 0x37e098: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x37e16c — __ZN21SoundServiceStatsItem6updateEv
#[doc(alias = "SoundServiceStatsItem::update(void)")]
#[doc(alias = "__ZN21SoundServiceStatsItem6updateEv")]
pub fn stub_37e16c() -> ! {
    todo!("0x37e16c SoundServiceStatsItem::update(void)")
}

// 0x37e344 — __ZThn32_N21SoundServiceStatsItemD1Ev
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
#[doc(alias = "__ZThn32_N21SoundServiceStatsItemD1Ev")]
pub fn stub_37e344() {
    // IDA 0x37e344: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x37e384 — __ZThn32_N21SoundServiceStatsItemD0Ev
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
#[doc(alias = "__ZThn32_N21SoundServiceStatsItemD0Ev")]
pub fn stub_37e384() {
    // IDA 0x37e384: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x37e458 — __ZThn36_N21SoundServiceStatsItemD1Ev
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
#[doc(alias = "__ZThn36_N21SoundServiceStatsItemD1Ev")]
pub fn stub_37e458() {
    // IDA 0x37e458: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x37e498 — __ZThn36_N21SoundServiceStatsItemD0Ev
#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
#[doc(alias = "__ZThn36_N21SoundServiceStatsItemD0Ev")]
pub fn stub_37e498() {
    // IDA 0x37e498: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x37e56c — __ZN5boost10shared_ptrI21SoundServiceStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrI21SoundServiceStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_37e56c() -> ! {
    todo!("0x37e56c boost::shared_ptr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37e720 — __ZN5boost6detail12shared_countC2IP21SoundServiceStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IP21SoundServiceStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_37e720() -> ! {
    todo!("0x37e720 boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37e828 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev")]
pub fn stub_37e828() {
    // IDA 0x37e828: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x37e82c — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev")]
pub fn stub_37e82c() {
    // IDA 0x37e82c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x37e830 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_37e830() -> ! {
    todo!("0x37e830 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x37e850 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_37e850() -> ! {
    todo!("0x37e850 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x37e868 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_37e868() -> ! {
    todo!("0x37e868 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x37e86c — __ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")]
#[doc(alias = "__ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_")]
pub fn stub_37e86c() -> ! {
    todo!("0x37e86c RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")
}

// 0x37e9c4 — __ZN3RBX10Soundscape12SoundService8SoundJobD1Ev
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
#[doc(alias = "__ZN3RBX10Soundscape12SoundService8SoundJobD1Ev")]
pub fn stub_37e9c4() {
    // IDA 0x37e9c4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x37e9c8 — __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
#[doc(alias = "__ZN3RBX10Soundscape12SoundService8SoundJobD0Ev")]
pub fn stub_37e9c8() {
    // IDA 0x37e9c8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x37ea68 — __ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_37ea68() -> ! {
    todo!("0x37ea68 RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x37ea84 — __ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_37ea84() -> ! {
    todo!("0x37ea84 RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x37eaa0 — __ZN3RBX10Soundscape12SoundService8SoundJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX10Soundscape12SoundService8SoundJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_37eaa0() -> ! {
    todo!("0x37eaa0 RBX::Soundscape::SoundService::SoundJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0x37ead8 — __GLOBAL__I_a_138
#[doc(alias = "global constructor keyed to_a_138")]
#[doc(alias = "__GLOBAL__I_a_138")]
pub fn stub_37ead8() -> ! {
    todo!("0x37ead8 global constructor keyed to_a_138")
}

// 0x38039c — __GLOBAL__I_a_139
#[doc(alias = "global constructor keyed to_a_139")]
#[doc(alias = "__GLOBAL__I_a_139")]
pub fn stub_38039c() -> ! {
    todo!("0x38039c global constructor keyed to_a_139")
}

// 0x380464 — __ZNK3RBX12SpanningEdge25getConstChildSpanningNodeEv
#[doc(alias = "RBX::SpanningEdge::getConstChildSpanningNode(void)const")]
#[doc(alias = "__ZNK3RBX12SpanningEdge25getConstChildSpanningNodeEv")]
pub fn stub_380464() -> ! {
    todo!("0x380464 RBX::SpanningEdge::getConstChildSpanningNode(void)const")
}

// 0x3804e0 — __ZN3RBX12SpanningEdge20getChildSpanningNodeEv
#[doc(alias = "RBX::SpanningEdge::getChildSpanningNode(void)")]
#[doc(alias = "__ZN3RBX12SpanningEdge20getChildSpanningNodeEv")]
pub fn stub_3804e0() -> ! {
    todo!("0x3804e0 RBX::SpanningEdge::getChildSpanningNode(void)")
}

// 0x3804e4 — __ZN3RBX12SpanningEdge21getParentSpanningNodeEv
#[doc(alias = "RBX::SpanningEdge::getParentSpanningNode(void)")]
#[doc(alias = "__ZN3RBX12SpanningEdge21getParentSpanningNodeEv")]
pub fn stub_3804e4() -> ! {
    todo!("0x3804e4 RBX::SpanningEdge::getParentSpanningNode(void)")
}

// 0x3804fc — __ZN3RBX12SpanningEdge22removeFromSpanningTreeEv
#[doc(alias = "RBX::SpanningEdge::removeFromSpanningTree(void)")]
#[doc(alias = "__ZN3RBX12SpanningEdge22removeFromSpanningTreeEv")]
pub fn stub_3804fc() -> ! {
    todo!("0x3804fc RBX::SpanningEdge::removeFromSpanningTree(void)")
}

// 0x380568 — __ZN3RBX12SpanningEdge17addToSpanningTreeEPNS_12SpanningNodeE
#[doc(alias = "RBX::SpanningEdge::addToSpanningTree(RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX12SpanningEdge17addToSpanningTreeEPNS_12SpanningNodeE")]
pub fn stub_380568() -> ! {
    todo!("0x380568 RBX::SpanningEdge::addToSpanningTree(RBX::SpanningNode *)")
}

// 0x3806bc — __ZNK3RBX12SpanningEdge14inSpanningTreeEv
#[doc(alias = "RBX::SpanningEdge::inSpanningTree(void)const")]
#[doc(alias = "__ZNK3RBX12SpanningEdge14inSpanningTreeEv")]
pub fn stub_3806bc() -> ! {
    todo!("0x3806bc RBX::SpanningEdge::inSpanningTree(void)const")
}

// 0x3806e4 — __GLOBAL__I_a_140
#[doc(alias = "global constructor keyed to_a_140")]
#[doc(alias = "__GLOBAL__I_a_140")]
pub fn stub_3806e4() -> ! {
    todo!("0x3806e4 global constructor keyed to_a_140")
}

// 0x3807ac — __ZN3RBX12SpanningNode15setEdgeToParentEPNS_12SpanningEdgeE
#[doc(alias = "RBX::SpanningNode::setEdgeToParent(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX12SpanningNode15setEdgeToParentEPNS_12SpanningEdgeE")]
pub fn stub_3807ac() -> ! {
    todo!("0x3807ac RBX::SpanningNode::setEdgeToParent(RBX::SpanningEdge *)")
}

// 0x3807b0 — __GLOBAL__I_a_141
#[doc(alias = "global constructor keyed to_a_141")]
#[doc(alias = "__GLOBAL__I_a_141")]
pub fn stub_3807b0() -> ! {
    todo!("0x3807b0 global constructor keyed to_a_141")
}

// 0x380878 — __ZN3RBX12SpanningTreeC2Ev
#[doc(alias = "RBX::SpanningTree::SpanningTree(void)")]
#[doc(alias = "__ZN3RBX12SpanningTreeC2Ev")]
pub fn stub_380878() -> ! {
    todo!("0x380878 RBX::SpanningTree::SpanningTree(void)")
}

// 0x38089c — __ZN3RBX12SpanningTreeD2Ev
#[doc(alias = "RBX::SpanningTree::~SpanningTree()")]
#[doc(alias = "__ZN3RBX12SpanningTreeD2Ev")]
pub fn stub_38089c() {
    // IDA 0x38089c: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3809c4 — __ZN3RBX12SpanningTree22insertSpanningTreeEdgeEPNS_12SpanningEdgeE
#[doc(alias = "RBX::SpanningTree::insertSpanningTreeEdge(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX12SpanningTree22insertSpanningTreeEdgeEPNS_12SpanningEdgeE")]
pub fn stub_3809c4() -> ! {
    todo!("0x3809c4 RBX::SpanningTree::insertSpanningTreeEdge(RBX::SpanningEdge *)")
}

// 0x380a6c — __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningEdgeERS2_Ri
#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningEdge *,RBX::SpanningEdge *&,int &)")]
#[doc(alias = "__ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningEdgeERS2_Ri")]
pub fn stub_380a6c() -> ! {
    todo!("0x380a6c RBX::SpanningTree::findLightestUpstream(RBX::SpanningEdge *,RBX::SpanningEdge *&,int &)")
}

// 0x380abc — __ZN3RBX12SpanningTree8swapTreeEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
#[doc(alias = "RBX::SpanningTree::swapTree(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX12SpanningTree8swapTreeEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE")]
pub fn stub_380abc() -> ! {
    todo!("0x380abc RBX::SpanningTree::swapTree(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")
}

// 0x380b30 — __ZN3RBX12SpanningTree22removeSpanningTreeEdgeEPNS_12SpanningEdgeE
#[doc(alias = "RBX::SpanningTree::removeSpanningTreeEdge(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX12SpanningTree22removeSpanningTreeEdgeEPNS_12SpanningEdgeE")]
pub fn stub_380b30() -> ! {
    todo!("0x380b30 RBX::SpanningTree::removeSpanningTreeEdge(RBX::SpanningEdge *)")
}

// 0x380bac — __ZN3RBX12SpanningTree22findHeaviestDownstreamEPNS_12SpanningNodeERS2_
#[doc(alias = "RBX::SpanningTree::findHeaviestDownstream(RBX::SpanningNode *,RBX::SpanningNode *&)")]
#[doc(alias = "__ZN3RBX12SpanningTree22findHeaviestDownstreamEPNS_12SpanningNodeERS2_")]
pub fn stub_380bac() -> ! {
    todo!("0x380bac RBX::SpanningTree::findHeaviestDownstream(RBX::SpanningNode *,RBX::SpanningNode *&)")
}

// 0x380cdc — __ZN3RBX12SpanningTree4swapEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
#[doc(alias = "RBX::SpanningTree::swap(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX12SpanningTree4swapEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE")]
pub fn stub_380cdc() -> ! {
    todo!("0x380cdc RBX::SpanningTree::swap(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")
}

// 0x380d50 — __ZN3RBX12SpanningTree10removeEdgeEPNS_12SpanningEdgeE
#[doc(alias = "RBX::SpanningTree::removeEdge(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX12SpanningTree10removeEdgeEPNS_12SpanningEdgeE")]
pub fn stub_380d50() -> ! {
    todo!("0x380d50 RBX::SpanningTree::removeEdge(RBX::SpanningEdge *)")
}

// 0x380e34 — __ZN3RBX12SpanningTree7addEdgeEPNS_12SpanningEdgeEPNS_12SpanningNodeE
#[doc(alias = "RBX::SpanningTree::addEdge(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX12SpanningTree7addEdgeEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
pub fn stub_380e34() -> ! {
    todo!("0x380e34 RBX::SpanningTree::addEdge(RBX::SpanningEdge *,RBX::SpanningNode *)")
}

// 0x380f1c — __ZN3RBX12SpanningTree22findAndDeactivateEdgesEPNS_12SpanningNodeEPNS_12SpanningEdgeERN3G3D5ArrayIS4_Li10ELm32EEE
#[doc(alias = "RBX::SpanningTree::findAndDeactivateEdges(RBX::SpanningNode *,RBX::SpanningEdge *,G3D::Array<RBX::SpanningEdge *,10,32ul> &)")]
#[doc(alias = "__ZN3RBX12SpanningTree22findAndDeactivateEdgesEPNS_12SpanningNodeEPNS_12SpanningEdgeERN3G3D5ArrayIS4_Li10ELm32EEE")]
pub fn stub_380f1c() -> ! {
    todo!("0x380f1c RBX::SpanningTree::findAndDeactivateEdges(RBX::SpanningNode *,RBX::SpanningEdge *,G3D::Array<RBX::SpanningEdge *,10,32ul> &)")
}

// 0x38103c — __ZN3RBX12SpanningTree13activateEdgesEPNS_12SpanningNodeERKN3G3D5ArrayIPNS_12SpanningEdgeELi10ELm32EEE
#[doc(alias = "RBX::SpanningTree::activateEdges(RBX::SpanningNode *,G3D::Array<RBX::SpanningEdge *,10,32ul> const&)")]
#[doc(alias = "__ZN3RBX12SpanningTree13activateEdgesEPNS_12SpanningNodeERKN3G3D5ArrayIPNS_12SpanningEdgeELi10ELm32EEE")]
pub fn stub_38103c() -> ! {
    todo!("0x38103c RBX::SpanningTree::activateEdges(RBX::SpanningNode *,G3D::Array<RBX::SpanningEdge *,10,32ul> const&)")
}

// 0x381120 — __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningNodeES2_iiRPNS_12SpanningEdgeERi
#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningNode *,RBX::SpanningNode *,int,int,RBX::SpanningEdge *&,int &)")]
#[doc(alias = "__ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningNodeES2_iiRPNS_12SpanningEdgeERi")]
pub fn stub_381120() -> ! {
    todo!("0x381120 RBX::SpanningTree::findLightestUpstream(RBX::SpanningNode *,RBX::SpanningNode *,int,int,RBX::SpanningEdge *&,int &)")
}

// 0x38120c — __ZN3RBX12SpanningTree19buildDownstreamTreeEPNS_12SpanningNodeERSt3setIS2_St4lessIS2_ESaIS2_EE
#[doc(alias = "RBX::SpanningTree::buildDownstreamTree(RBX::SpanningNode *,std::set<RBX::SpanningNode *,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>> &)")]
#[doc(alias = "__ZN3RBX12SpanningTree19buildDownstreamTreeEPNS_12SpanningNodeERSt3setIS2_St4lessIS2_ESaIS2_EE")]
pub fn stub_38120c() -> ! {
    todo!("0x38120c RBX::SpanningTree::buildDownstreamTree(RBX::SpanningNode *,std::set<RBX::SpanningNode *,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>> &)")
}

// 0x3812ac — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::append(RBX::SpanningEdge * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6appendERKS3_")]
pub fn stub_3812ac() -> ! {
    todo!("0x3812ac G3D::Array<RBX::SpanningEdge *,10,32ul>::append(RBX::SpanningEdge * const&)")
}

// 0x381308 — __ZN3RBX12SpanningNode8getDepthEPS0_
#[doc(alias = "RBX::SpanningNode::getDepth(RBX::SpanningNode*)")]
#[doc(alias = "__ZN3RBX12SpanningNode8getDepthEPS0_")]
pub fn stub_381308() -> ! {
    todo!("0x381308 RBX::SpanningNode::getDepth(RBX::SpanningNode*)")
}

// 0x381328 — __ZN3RBX12SpanningTree20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX12SpanningTree20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
pub fn stub_381328() -> ! {
    todo!("0x381328 RBX::SpanningTree::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")
}

// 0x38132c — __ZN3RBX12SpanningTree19onSpanningEdgeAddedEPNS_12SpanningEdgeE
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX12SpanningTree19onSpanningEdgeAddedEPNS_12SpanningEdgeE")]
pub fn stub_38132c() -> ! {
    todo!("0x38132c RBX::SpanningTree::onSpanningEdgeAdded(RBX::SpanningEdge *)")
}

// 0x381330 — __ZN3RBX12SpanningTree22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX12SpanningTree22onSpanningEdgeRemovingEPNS_12SpanningEdgeE")]
pub fn stub_381330() -> ! {
    todo!("0x381330 RBX::SpanningTree::onSpanningEdgeRemoving(RBX::SpanningEdge *)")
}

// 0x381334 — __ZN3RBX12SpanningTree21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX12SpanningTree21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
pub fn stub_381334() -> ! {
    todo!("0x381334 RBX::SpanningTree::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")
}

// 0x381338 — __ZN3RBX12SpanningTree12validateTreeEPNS_12SpanningNodeE
#[doc(alias = "RBX::SpanningTree::validateTree(RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX12SpanningTree12validateTreeEPNS_12SpanningNodeE")]
pub fn stub_381338() -> ! {
    todo!("0x381338 RBX::SpanningTree::validateTree(RBX::SpanningNode *)")
}

// 0x38133c — __ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE
#[doc(alias = "RBX::FindHeaviest::operator()(RBX::SpanningNode *,RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE")]
pub fn stub_38133c() -> ! {
    todo!("0x38133c RBX::FindHeaviest::operator()(RBX::SpanningNode *,RBX::SpanningEdge *)")
}

// 0x3813bc — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert_unique(RBX::SpanningNode * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
pub fn stub_3813bc() -> ! {
    todo!("0x3813bc std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert_unique(RBX::SpanningNode * const&)")
}

// 0x381424 — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::SpanningNode * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
pub fn stub_381424() -> ! {
    todo!("0x381424 std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::SpanningNode * const&)")
}

// 0x38147c — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6resizeEib")]
pub fn stub_38147c() -> ! {
    todo!("0x38147c G3D::Array<RBX::SpanningEdge *,10,32ul>::resize(int,bool)")
}

// 0x381534 — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE7reallocEi")]
pub fn stub_381534() -> ! {
    todo!("0x381534 G3D::Array<RBX::SpanningEdge *,10,32ul>::realloc(int)")
}

// 0x38171c — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EED2Ev")]
pub fn stub_38171c() {
    // IDA 0x38171c: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3817f0 — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EEC2Ev")]
pub fn stub_3817f0() -> ! {
    todo!("0x3817f0 G3D::Array<RBX::SpanningEdge *,10,32ul>::Array(void)")
}

// 0x3818e0 — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_erase(std::_Rb_tree_node<RBX::SpanningNode *> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
pub fn stub_3818e0() -> ! {
    todo!("0x3818e0 std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_erase(std::_Rb_tree_node<RBX::SpanningNode *> *)")
}

// 0x381908 — __GLOBAL__I_a_142
#[doc(alias = "global constructor keyed to_a_142")]
#[doc(alias = "__GLOBAL__I_a_142")]
pub fn stub_381908() -> ! {
    todo!("0x381908 global constructor keyed to_a_142")
}

// 0x3819d0 — __ZN3RBX11StandardOut9singletonEv
#[doc(alias = "RBX::StandardOut::singleton(void)")]
#[doc(alias = "__ZN3RBX11StandardOut9singletonEv")]
pub fn stub_3819d0() -> ! {
    todo!("0x3819d0 RBX::StandardOut::singleton(void)")
}

// 0x3821f0 — __ZN5boost10shared_ptrIN3RBX11StandardOutEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::StandardOut>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11StandardOutEED1Ev")]
pub fn stub_3821f0() {
    // IDA 0x3821f0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x382204 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::operator()(RBX::StandardOutMessage const&)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_")]
pub fn stub_382204() -> ! {
    todo!("0x382204 rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::operator()(RBX::StandardOutMessage const&)")
}

// 0x382710 — __ZN5boost10shared_ptrIN3RBX11StandardOutEEC2ERKS3_
#[doc(alias = "boost::shared_ptr<RBX::StandardOut>::shared_ptr(boost::shared_ptr<RBX::StandardOut> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11StandardOutEEC2ERKS3_")]
pub fn stub_382710() -> ! {
    todo!("0x382710 boost::shared_ptr<RBX::StandardOut>::shared_ptr(boost::shared_ptr<RBX::StandardOut> const&)")
}

// 0x382714 — __ZN5boost10shared_ptrIN3RBX11StandardOutEEaSERKS3_
#[doc(alias = "boost::shared_ptr<RBX::StandardOut>::operator=(boost::shared_ptr<RBX::StandardOut> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11StandardOutEEaSERKS3_")]
pub fn stub_382714() -> ! {
    todo!("0x382714 boost::shared_ptr<RBX::StandardOut>::operator=(boost::shared_ptr<RBX::StandardOut> const&)")
}

// 0x382718 — __ZN5boost10shared_ptrIN3RBX11StandardOutEEC2IS1_EEPT_
#[doc(alias = "boost::shared_ptr<RBX::StandardOut>::shared_ptr<RBX::StandardOut>(RBX::StandardOut *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11StandardOutEEC2IS1_EEPT_")]
pub fn stub_382718() -> ! {
    todo!("0x382718 boost::shared_ptr<RBX::StandardOut>::shared_ptr<RBX::StandardOut>(RBX::StandardOut *)")
}

// 0x382760 — __ZN5boost6detail12shared_countC2IPN3RBX11StandardOutEEET_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StandardOut *>(RBX::StandardOut *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX11StandardOutEEET_")]
pub fn stub_382760() -> ! {
    todo!("0x382760 boost::detail::shared_count::shared_count<RBX::StandardOut *>(RBX::StandardOut *)")
}

// 0x38278c — __ZN5boost6detail18sp_counted_impl_pIPN3RBX11StandardOutEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut *>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pIPN3RBX11StandardOutEED1Ev")]
pub fn stub_38278c() {
    // IDA 0x38278c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x382790 — __ZN5boost6detail18sp_counted_impl_pIPN3RBX11StandardOutEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut *>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pIPN3RBX11StandardOutEED0Ev")]
pub fn stub_382790() {
    // IDA 0x382790: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3827a0 — __ZN5boost6detail18sp_counted_impl_pIPN3RBX11StandardOutEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut *>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pIPN3RBX11StandardOutEE7disposeEv")]
pub fn stub_3827a0() -> ! {
    todo!("0x3827a0 boost::detail::sp_counted_impl_p<RBX::StandardOut *>::dispose(void)")
}

// 0x3827c4 — __ZN5boost6detail18sp_counted_impl_pIPN3RBX11StandardOutEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut *>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pIPN3RBX11StandardOutEE11get_deleterERKSt9type_info")]
pub fn stub_3827c4() -> ! {
    todo!("0x3827c4 boost::detail::sp_counted_impl_p<RBX::StandardOut *>::get_deleter(std::type_info const&)")
}

// 0x3827dc — __ZN5boost6detail18sp_counted_impl_pIPN3RBX11StandardOutEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut *>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pIPN3RBX11StandardOutEE19get_untyped_deleterEv")]
pub fn stub_3827dc() -> ! {
    todo!("0x3827dc boost::detail::sp_counted_impl_p<RBX::StandardOut *>::get_untyped_deleter(void)")
}

// 0x3827e0 — __ZN3RBX11StandardOut6printFEPKcz
#[doc(alias = "RBX::StandardOut::printF(char const*,...)")]
#[doc(alias = "__ZN3RBX11StandardOut6printFEPKcz")]
pub fn stub_3827e0() -> ! {
    todo!("0x3827e0 RBX::StandardOut::printF(char const*,...)")
}

// 0x38289c — __ZN3RBX11StandardOut8printNLEv
#[doc(alias = "RBX::StandardOut::printNL(void)")]
#[doc(alias = "__ZN3RBX11StandardOut8printNLEv")]
pub fn stub_38289c() -> ! {
    todo!("0x38289c RBX::StandardOut::printNL(void)")
}

// 0x3828b4 — __ZN3RBX11StandardOut5printEPKc
#[doc(alias = "RBX::StandardOut::print(char const*)")]
#[doc(alias = "__ZN3RBX11StandardOut5printEPKc")]
pub fn stub_3828b4() -> ! {
    todo!("0x3828b4 RBX::StandardOut::print(char const*)")
}

// 0x38292c — __ZN3RBX18StandardOutMessage14addMessageLineERKN3RBX6System5BlockE
#[doc(alias = "RBX::StandardOutMessage::addMessageLine(RBX::System::Block const&)")]
#[doc(alias = "__ZN3RBX18StandardOutMessage14addMessageLineERKN3RBX6System5BlockE")]
pub fn stub_38292c() -> ! {
    todo!("0x38292c RBX::StandardOutMessage::addMessageLine(RBX::System::Block const&)")
}

// 0x38294c — __ZN3RBX18StandardOutMessageD2Ev
#[doc(alias = "RBX::StandardOutMessage::~StandardOutMessage()")]
#[doc(alias = "__ZN3RBX18StandardOutMessageD2Ev")]
pub fn stub_38294c() {
    // IDA 0x38294c: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3829cc — __ZN3RBX18StandardOutMessageC2Ev
#[doc(alias = "RBX::StandardOutMessage::StandardOutMessage(void)")]
#[doc(alias = "__ZN3RBX18StandardOutMessageC2Ev")]
pub fn stub_3829cc() -> ! {
    todo!("0x3829cc RBX::StandardOutMessage::StandardOutMessage(void)")
}

// 0x3829dc — __ZN3RBX11StandardOut11printSystemERKN3RBX6System5BlockE
#[doc(alias = "RBX::StandardOut::printSystem(RBX::System::Block const&)")]
#[doc(alias = "__ZN3RBX11StandardOut11printSystemERKN3RBX6System5BlockE")]
pub fn stub_3829dc() -> ! {
    todo!("0x3829dc RBX::StandardOut::printSystem(RBX::System::Block const&)")
}

// 0x3829fc — __ZN3RBX10Reflection17BoundFuncDescriptor33checkSecurityAndDescribeInstanceEPSsRKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Reflection::BoundFuncDescriptor::checkSecurityAndDescribeInstance(std::string *,boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection17BoundFuncDescriptor33checkSecurityAndDescribeInstanceEPSsRKN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_3829fc() -> ! {
    todo!("0x3829fc RBX::Reflection::BoundFuncDescriptor::checkSecurityAndDescribeInstance(std::string *,boost::shared_ptr<RBX::Instance> const&)")
}

// 0x382ac8 — __ZN3RBX11StandardOut12printMessageERKSs
#[doc(alias = "RBX::StandardOut::printMessage(std::string const&)")]
#[doc(alias = "__ZN3RBX11StandardOut12printMessageERKSs")]
pub fn stub_382ac8() -> ! {
    todo!("0x382ac8 RBX::StandardOut::printMessage(std::string const&)")
}

// 0x382b0c — __ZN3RBX11StandardOut13printCriticalERKSs
#[doc(alias = "RBX::StandardOut::printCritical(std::string const&)")]
#[doc(alias = "__ZN3RBX11StandardOut13printCriticalERKSs")]
pub fn stub_382b0c() -> ! {
    todo!("0x382b0c RBX::StandardOut::printCritical(std::string const&)")
}

// 0x382b50 — __ZN3RBX11StandardOut11printWarningERKSs
#[doc(alias = "RBX::StandardOut::printWarning(std::string const&)")]
#[doc(alias = "__ZN3RBX11StandardOut11printWarningERKSs")]
pub fn stub_382b50() -> ! {
    todo!("0x382b50 RBX::StandardOut::printWarning(std::string const&)")
}

// 0x382b80 — __ZNK3RBX5G3DUtil31getOrCreateInOutlinePassShaderEv
#[doc(alias = "RBX::G3DUtil::getOrCreateInOutlinePassShader(void)const")]
#[doc(alias = "__ZNK3RBX5G3DUtil31getOrCreateInOutlinePassShaderEv")]
pub fn stub_382b80() -> ! {
    todo!("0x382b80 RBX::G3DUtil::getOrCreateInOutlinePassShader(void)const")
}

// 0x382b94 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEE7connectEPFvS5_E
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::connect(void (*)(RBX::StandardOutMessage const&))")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEE7connectEPFvS5_E")]
pub fn stub_382b94() -> ! {
    todo!("0x382b94 rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::connect(void (*)(RBX::StandardOutMessage const&))")
}

// 0x3831a8 — __GLOBAL__I_a_143
#[doc(alias = "global constructor keyed to_a_143")]
#[doc(alias = "__GLOBAL__I_a_143")]
pub fn stub_3831a8() -> ! {
    todo!("0x3831a8 global constructor keyed to_a_143")
}

// 0x383474 — __ZN3RBX6System5BlockD2Ev
#[doc(alias = "RBX::System::Block::~Block()")]
#[doc(alias = "__ZN3RBX6System5BlockD2Ev")]
pub fn stub_383474() {
    // IDA 0x383474: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x38348c — __ZN3RBX6System5Block12setTextColorERKN3G3D7Color3fE
#[doc(alias = "RBX::System::Block::setTextColor(G3D::Color3f const&)")]
#[doc(alias = "__ZN3RBX6System5Block12setTextColorERKN3G3D7Color3fE")]
pub fn stub_38348c() -> ! {
    todo!("0x38348c RBX::System::Block::setTextColor(G3D::Color3f const&)")
}

// 0x3834dc — __ZN3RBX6System5Block13appendContentERKSsRKNS0_4TextE
#[doc(alias = "RBX::System::Block::appendContent(std::string const&,RBX::System::Text const&)")]
#[doc(alias = "__ZN3RBX6System5Block13appendContentERKSsRKNS0_4TextE")]
pub fn stub_3834dc() -> ! {
    todo!("0x3834dc RBX::System::Block::appendContent(std::string const&,RBX::System::Text const&)")
}

// 0x3834f4 — __ZN3RBX6System8TextSpanC2Ev
#[doc(alias = "RBX::System::TextSpan::TextSpan(void)")]
#[doc(alias = "__ZN3RBX6System8TextSpanC2Ev")]
pub fn stub_3834f4() -> ! {
    todo!("0x3834f4 RBX::System::TextSpan::TextSpan(void)")
}

// 0x383550 — __ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::System::TextSpan*,std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>>,RBX::System::TextSpan const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_383550() -> ! {
    todo!("0x383550 std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::System::TextSpan*,std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>>,RBX::System::TextSpan const&)")
}

// 0x383634 — __ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EEaSERKS4_
#[doc(alias = "std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::operator=(std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EEaSERKS4_")]
pub fn stub_383634() -> ! {
    todo!("0x383634 std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::operator=(std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>> const&)")
}

// 0x3836fc — __ZN3RBX6System5Block12appendStringERKSs
#[doc(alias = "RBX::System::Block::appendString(std::string const&)")]
#[doc(alias = "__ZN3RBX6System5Block12appendStringERKSs")]
pub fn stub_3836fc() -> ! {
    todo!("0x3836fc RBX::System::Block::appendString(std::string const&)")
}

// 0x383724 — __ZN3RBX6System4Text14appendTextSpanERKSs
#[doc(alias = "RBX::System::Text::appendTextSpan(std::string const&)")]
#[doc(alias = "__ZN3RBX6System4Text14appendTextSpanERKSs")]
pub fn stub_383724() -> ! {
    todo!("0x383724 RBX::System::Text::appendTextSpan(std::string const&)")
}

// 0x3837c0 — __ZN3RBX6System4TextC2Ev
#[doc(alias = "RBX::System::Text::Text(void)")]
#[doc(alias = "__ZN3RBX6System4TextC2Ev")]
pub fn stub_3837c0() -> ! {
    todo!("0x3837c0 RBX::System::Text::Text(void)")
}

// 0x3837fc — __ZN3RBX6System4TextD2Ev
#[doc(alias = "RBX::System::Text::~Text()")]
#[doc(alias = "__ZN3RBX6System4TextD2Ev")]
pub fn stub_3837fc() {
    // IDA 0x3837fc: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x383864 — __ZN3RBX6System4TextC2ERKS1_
#[doc(alias = "RBX::System::Text::Text(RBX::System::Text const&)")]
#[doc(alias = "__ZN3RBX6System4TextC2ERKS1_")]
pub fn stub_383864() -> ! {
    todo!("0x383864 RBX::System::Text::Text(RBX::System::Text const&)")
}

// 0x3838e8 — __ZN3RBX6System4TextaSERKS1_
#[doc(alias = "RBX::System::Text::operator=(RBX::System::Text const&)")]
#[doc(alias = "__ZN3RBX6System4TextaSERKS1_")]
pub fn stub_3838e8() -> ! {
    todo!("0x3838e8 RBX::System::Text::operator=(RBX::System::Text const&)")
}

// 0x3839e4 — __ZN3RBX6System5BlockC2Ev
#[doc(alias = "RBX::System::Block::Block(void)")]
#[doc(alias = "__ZN3RBX6System5BlockC2Ev")]
pub fn stub_3839e4() -> ! {
    todo!("0x3839e4 RBX::System::Block::Block(void)")
}

// 0x3839f4 — __ZN3RBX6System5BlockC2ERKS1_
#[doc(alias = "RBX::System::Block::Block(RBX::System::Block const&)")]
#[doc(alias = "__ZN3RBX6System5BlockC2ERKS1_")]
pub fn stub_3839f4() -> ! {
    todo!("0x3839f4 RBX::System::Block::Block(RBX::System::Block const&)")
}

// 0x383b0c — __ZN3RBX6System5BlockaSERKS1_
#[doc(alias = "RBX::System::Block::operator=(RBX::System::Block const&)")]
#[doc(alias = "__ZN3RBX6System5BlockaSERKS1_")]
pub fn stub_383b0c() -> ! {
    todo!("0x383b0c RBX::System::Block::operator=(RBX::System::Block const&)")
}

// 0x383be4 — __ZN3RBX10SystemUtil11htmlToBlockERKSs
#[doc(alias = "RBX::SystemUtil::htmlToBlock(std::string const&)")]
#[doc(alias = "__ZN3RBX10SystemUtil11htmlToBlockERKSs")]
pub fn stub_383be4() -> ! {
    todo!("0x383be4 RBX::SystemUtil::htmlToBlock(std::string const&)")
}

// 0x383c48 — __ZN3RBX6System8TextSpan6appendERKSs
#[doc(alias = "RBX::System::TextSpan::append(std::string const&)")]
#[doc(alias = "__ZN3RBX6System8TextSpan6appendERKSs")]
pub fn stub_383c48() -> ! {
    todo!("0x383c48 RBX::System::TextSpan::append(std::string const&)")
}

// 0x383cd0 — __ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::push_back(RBX::System::TextSpan const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EE9push_backERKS2_")]
pub fn stub_383cd0() -> ! {
    todo!("0x383cd0 std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::push_back(RBX::System::TextSpan const&)")
}

// 0x383d30 — __ZN3RBX6System5Block5clearEv
#[doc(alias = "RBX::System::Block::clear(void)")]
#[doc(alias = "__ZN3RBX6System5Block5clearEv")]
pub fn stub_383d30() -> ! {
    todo!("0x383d30 RBX::System::Block::clear(void)")
}

// 0x383d9c — __ZN3RBX6System4Text5clearEv
#[doc(alias = "RBX::System::Text::clear(void)")]
#[doc(alias = "__ZN3RBX6System4Text5clearEv")]
pub fn stub_383d9c() -> ! {
    todo!("0x383d9c RBX::System::Text::clear(void)")
}

// 0x383dec — __ZNK3RBX6System5Block7isEmptyEv
#[doc(alias = "RBX::System::Block::isEmpty(void)const")]
#[doc(alias = "__ZNK3RBX6System5Block7isEmptyEv")]
pub fn stub_383dec() -> ! {
    todo!("0x383dec RBX::System::Block::isEmpty(void)const")
}

// 0x383e28 — __ZNK3RBX6System4Text7isEmptyEv
#[doc(alias = "RBX::System::Text::isEmpty(void)const")]
#[doc(alias = "__ZNK3RBX6System4Text7isEmptyEv")]
pub fn stub_383e28() -> ! {
    todo!("0x383e28 RBX::System::Text::isEmpty(void)const")
}

// 0x383e64 — __ZN3RBX18StandardOutMessage7getTextEv
#[doc(alias = "RBX::StandardOutMessage::getText(void)")]
#[doc(alias = "__ZN3RBX18StandardOutMessage7getTextEv")]
pub fn stub_383e64() -> ! {
    todo!("0x383e64 RBX::StandardOutMessage::getText(void)")
}

// 0x383ef0 — __ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EmRKS2_
#[doc(alias = "std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::System::TextSpan*,std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>>,unsigned long,RBX::System::TextSpan const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EmRKS2_")]
pub fn stub_383ef0() -> ! {
    todo!("0x383ef0 std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::System::TextSpan*,std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>>,unsigned long,RBX::System::TextSpan const&)")
}

// 0x384004 — __ZN3RBX6System11TextAndLinkC2Ev
#[doc(alias = "RBX::System::TextAndLink::TextAndLink(void)")]
#[doc(alias = "__ZN3RBX6System11TextAndLinkC2Ev")]
pub fn stub_384004() -> ! {
    todo!("0x384004 RBX::System::TextAndLink::TextAndLink(void)")
}

// 0x384020 — __ZN3RBX6System11TextAndLinkD2Ev
#[doc(alias = "RBX::System::TextAndLink::~TextAndLink()")]
#[doc(alias = "__ZN3RBX6System11TextAndLinkD2Ev")]
pub fn stub_384020() {
    // IDA 0x384020: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x384048 — __ZN3RBX6System8TextLinkC2Ev
#[doc(alias = "RBX::System::TextLink::TextLink(void)")]
#[doc(alias = "__ZN3RBX6System8TextLinkC2Ev")]
pub fn stub_384048() -> ! {
    todo!("0x384048 RBX::System::TextLink::TextLink(void)")
}

// 0x38405c — __ZN3RBX6System8TextLinkD2Ev
#[doc(alias = "RBX::System::TextLink::~TextLink()")]
#[doc(alias = "__ZN3RBX6System8TextLinkD2Ev")]
pub fn stub_38405c() {
    // IDA 0x38405c: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x384090 — __ZN3RBX6System5Block14appendTextSpanERKSs
#[doc(alias = "RBX::System::Block::appendTextSpan(std::string const&)")]
#[doc(alias = "__ZN3RBX6System5Block14appendTextSpanERKSs")]
pub fn stub_384090() -> ! {
    todo!("0x384090 RBX::System::Block::appendTextSpan(std::string const&)")
}

// 0x3840bc — __ZN3RBX6System5Block14appendTextLinkERKSsS3_
#[doc(alias = "RBX::System::Block::appendTextLink(std::string const&,std::string const&)")]
#[doc(alias = "__ZN3RBX6System5Block14appendTextLinkERKSsS3_")]
pub fn stub_3840bc() -> ! {
    todo!("0x3840bc RBX::System::Block::appendTextLink(std::string const&,std::string const&)")
}

// 0x38413c — __ZN3RBX6System5Block11appendBlockERKS1_
#[doc(alias = "RBX::System::Block::appendBlock(RBX::System::Block const&)")]
#[doc(alias = "__ZN3RBX6System5Block11appendBlockERKS1_")]
pub fn stub_38413c() -> ! {
    todo!("0x38413c RBX::System::Block::appendBlock(RBX::System::Block const&)")
}

// 0x3842c0 — __ZNSt6vectorIN3RBX6System11TextAndLinkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::System::TextAndLink,std::allocator<RBX::System::TextAndLink>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::System::TextAndLink*,std::vector<RBX::System::TextAndLink,std::allocator<RBX::System::TextAndLink>>>,RBX::System::TextAndLink const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX6System11TextAndLinkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_3842c0() -> ! {
    todo!("0x3842c0 std::vector<RBX::System::TextAndLink,std::allocator<RBX::System::TextAndLink>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::System::TextAndLink*,std::vector<RBX::System::TextAndLink,std::allocator<RBX::System::TextAndLink>>>,RBX::System::TextAndLink const&)")
}

// 0x3843b0 — __ZNSt6vectorIN3RBX6System8TextLinkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::System::TextLink,std::allocator<RBX::System::TextLink>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::System::TextLink*,std::vector<RBX::System::TextLink,std::allocator<RBX::System::TextLink>>>,RBX::System::TextLink const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX6System8TextLinkESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_3843b0() -> ! {
    todo!("0x3843b0 std::vector<RBX::System::TextLink,std::allocator<RBX::System::TextLink>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::System::TextLink*,std::vector<RBX::System::TextLink,std::allocator<RBX::System::TextLink>>>,RBX::System::TextLink const&)")
}

// 0x3844e8 — __ZN3RBX10SystemUtil14getHtmlAsBlockENS_10SystemHtmlE
#[doc(alias = "RBX::SystemUtil::getHtmlAsBlock(RBX::SystemHtml)")]
#[doc(alias = "__ZN3RBX10SystemUtil14getHtmlAsBlockENS_10SystemHtmlE")]
pub fn stub_3844e8() -> ! {
    todo!("0x3844e8 RBX::SystemUtil::getHtmlAsBlock(RBX::SystemHtml)")
}

// 0x384534 — __ZN3RBX10SystemUtil11getHtmlLinkENS_10SystemHtmlE
#[doc(alias = "RBX::SystemUtil::getHtmlLink(RBX::SystemHtml)")]
#[doc(alias = "__ZN3RBX10SystemUtil11getHtmlLinkENS_10SystemHtmlE")]
pub fn stub_384534() -> ! {
    todo!("0x384534 RBX::SystemUtil::getHtmlLink(RBX::SystemHtml)")
}

// 0x3845d8 — __ZN3RBX10SystemUtil11getHtmlTextENS_10SystemHtmlE
#[doc(alias = "RBX::SystemUtil::getHtmlText(RBX::SystemHtml)")]
#[doc(alias = "__ZN3RBX10SystemUtil11getHtmlTextENS_10SystemHtmlE")]
pub fn stub_3845d8() -> ! {
    todo!("0x3845d8 RBX::SystemUtil::getHtmlText(RBX::SystemHtml)")
}

// 0x384628 — __ZN3RBX10SystemUtil12getHtmlTitleENS_10SystemHtmlE
#[doc(alias = "RBX::SystemUtil::getHtmlTitle(RBX::SystemHtml)")]
#[doc(alias = "__ZN3RBX10SystemUtil12getHtmlTitleENS_10SystemHtmlE")]
pub fn stub_384628() -> ! {
    todo!("0x384628 RBX::SystemUtil::getHtmlTitle(RBX::SystemHtml)")
}

// 0x384690 — __ZN3RBX18StandardOutMessage13getHtmlAsTextEv
#[doc(alias = "RBX::StandardOutMessage::getHtmlAsText(void)")]
#[doc(alias = "__ZN3RBX18StandardOutMessage13getHtmlAsTextEv")]
pub fn stub_384690() -> ! {
    todo!("0x384690 RBX::StandardOutMessage::getHtmlAsText(void)")
}

// 0x3846d0 — __ZN3RBX18StandardOutMessage12getHtmlTitleEv
#[doc(alias = "RBX::StandardOutMessage::getHtmlTitle(void)")]
#[doc(alias = "__ZN3RBX18StandardOutMessage12getHtmlTitleEv")]
pub fn stub_3846d0() -> ! {
    todo!("0x3846d0 RBX::StandardOutMessage::getHtmlTitle(void)")
}

// 0x384710 — __ZN3RBX18StandardOutMessage11getHtmlLinkEv
#[doc(alias = "RBX::StandardOutMessage::getHtmlLink(void)")]
#[doc(alias = "__ZN3RBX18StandardOutMessage11getHtmlLinkEv")]
pub fn stub_384710() -> ! {
    todo!("0x384710 RBX::StandardOutMessage::getHtmlLink(void)")
}

// 0x384758 — __ZN3RBX18StandardOutMessage11getHtmlTextEv
#[doc(alias = "RBX::StandardOutMessage::getHtmlText(void)")]
#[doc(alias = "__ZN3RBX18StandardOutMessage11getHtmlTextEv")]
pub fn stub_384758() -> ! {
    todo!("0x384758 RBX::StandardOutMessage::getHtmlText(void)")
}

// 0x3847a0 — __ZN3RBX6System5Block9setMarginERKN3G3D8Vector2fE
#[doc(alias = "RBX::System::Block::setMargin(G3D::Vector2f const&)")]
#[doc(alias = "__ZN3RBX6System5Block9setMarginERKN3G3D8Vector2fE")]
pub fn stub_3847a0() -> ! {
    todo!("0x3847a0 RBX::System::Block::setMargin(G3D::Vector2f const&)")
}

// 0x3847bc — __ZN3RBX6System5Block11setCellSpaceEf
#[doc(alias = "RBX::System::Block::setCellSpace(float)")]
#[doc(alias = "__ZN3RBX6System5Block11setCellSpaceEf")]
pub fn stub_3847bc() -> ! {
    todo!("0x3847bc RBX::System::Block::setCellSpace(float)")
}

// 0x3847e8 — __ZN3RBX6System5Block12setCellOffsetERKN3G3D8Vector2fE
#[doc(alias = "RBX::System::Block::setCellOffset(G3D::Vector2f const&)")]
#[doc(alias = "__ZN3RBX6System5Block12setCellOffsetERKN3G3D8Vector2fE")]
pub fn stub_3847e8() -> ! {
    todo!("0x3847e8 RBX::System::Block::setCellOffset(G3D::Vector2f const&)")
}

// 0x384804 — __ZN3RBX6System5Block13setBackgroundERKN3G3D7Color3fE
#[doc(alias = "RBX::System::Block::setBackground(G3D::Color3f const&)")]
#[doc(alias = "__ZN3RBX6System5Block13setBackgroundERKN3G3D7Color3fE")]
pub fn stub_384804() -> ! {
    todo!("0x384804 RBX::System::Block::setBackground(G3D::Color3f const&)")
}

// 0x384824 — __ZN3RBX6System5Block10setIsButtonEb
#[doc(alias = "RBX::System::Block::setIsButton(bool)")]
#[doc(alias = "__ZN3RBX6System5Block10setIsButtonEb")]
pub fn stub_384824() -> ! {
    todo!("0x384824 RBX::System::Block::setIsButton(bool)")
}

// 0x384844 — __ZN3RBX6System5Block12setCellBorderEf
#[doc(alias = "RBX::System::Block::setCellBorder(float)")]
#[doc(alias = "__ZN3RBX6System5Block12setCellBorderEf")]
pub fn stub_384844() -> ! {
    todo!("0x384844 RBX::System::Block::setCellBorder(float)")
}

// 0x384868 — __ZN3RBX6System8TextSpanC2ERKS1_
#[doc(alias = "RBX::System::TextSpan::TextSpan(RBX::System::TextSpan const&)")]
#[doc(alias = "__ZN3RBX6System8TextSpanC2ERKS1_")]
pub fn stub_384868() -> ! {
    todo!("0x384868 RBX::System::TextSpan::TextSpan(RBX::System::TextSpan const&)")
}

// 0x384a24 — __ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EmRKS2__HACK
#[doc(alias = "HACK std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::_M_fill_insert HACK")]
#[doc(alias = "__ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EmRKS2__HACK")]
pub fn stub_384a24() -> ! {
    todo!("0x384a24 HACK std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::_M_fill_insert HACK")
}

// 0x384aa8 — __ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EE19_M_range_insert_auxIN9__gnu_cxx17__normal_iteratorIPS2_S4_EEEvT_S8_St20forward_iterator_tagHACK
#[doc(alias = "HACK std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::_M_range_insert_aux HACK")]
#[doc(alias = "__ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EE19_M_range_insert_auxIN9__gnu_cxx17__normal_iteratorIPS2_S4_EEEvT_S8_St20forward_iterator_tagHACK")]
pub fn stub_384aa8() -> ! {
    todo!("0x384aa8 HACK std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::_M_range_insert_aux HACK")
}

// 0x38531c — __ZN3RBX6System5Block10learnFromBEv
#[doc(alias = "RBX::System::Block::learnFromB(void)")]
#[doc(alias = "__ZN3RBX6System5Block10learnFromBEv")]
pub fn stub_38531c() -> ! {
    todo!("0x38531c RBX::System::Block::learnFromB(void)")
}

// 0x385390 — __ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EEaSERKS4__HACK2
#[doc(alias = "HACK2 std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::operator= HACK2")]
#[doc(alias = "__ZNSt6vectorIN3RBX6System8TextSpanESaIS2_EEaSERKS4__HACK2")]
pub fn stub_385390() -> ! {
    todo!("0x385390 HACK2 std::vector<RBX::System::TextSpan,std::allocator<RBX::System::TextSpan>>::operator= HACK2")
}

// 0x3853b4 — __ZN3RBX6System4TextC2EPSs
#[doc(alias = "RBX::System::Text::Text(std::string *)")]
#[doc(alias = "__ZN3RBX6System4TextC2EPSs")]
pub fn stub_3853b4() -> ! {
    todo!("0x3853b4 RBX::System::Text::Text(std::string *)")
}

// 0x3853d4 — __ZN3RBX6System5BlockC2EPSs
#[doc(alias = "RBX::System::Block::Block(std::string *)")]
#[doc(alias = "__ZN3RBX6System5BlockC2EPSs")]
pub fn stub_3853d4() -> ! {
    todo!("0x3853d4 RBX::System::Block::Block(std::string *)")
}

// 0x385578 — __ZN3RBX5Stats13ItemArbiterImp11initializeERN5boost8functionIFvRN3RBX5Stats4ItemEEEE
#[doc(alias = "RBX::Stats::ItemArbiterImp::initialize(boost::function<void ()(RBX::Stats::Item &)> &)")]
#[doc(alias = "__ZN3RBX5Stats13ItemArbiterImp11initializeERN5boost8functionIFvRN3RBX5Stats4ItemEEEE")]
pub fn stub_385578() -> ! {
    todo!("0x385578 RBX::Stats::ItemArbiterImp::initialize(boost::function<void ()(RBX::Stats::Item &)> &)")
}

// 0x385594 — __ZN3RBX5Stats4ItemD1Ev
#[doc(alias = "RBX::Stats::Item::~Item()")]
#[doc(alias = "__ZN3RBX5Stats4ItemD1Ev")]
pub fn stub_385594() {
    // IDA 0x385594: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3855b0 — __ZN3RBX5Stats4ItemD0Ev
#[doc(alias = "RBX::Stats::Item::~Item()")]
#[doc(alias = "__ZN3RBX5Stats4ItemD0Ev")]
pub fn stub_3855b0() {
    // IDA 0x3855b0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x385630 — __ZNSt6vectorIN3RBX5Stats4Item4DataESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
#[doc(alias = "std::vector<RBX::Stats::Item::Data,std::allocator<RBX::Stats::Item::Data>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Stats::Item::Data*,std::vector<RBX::Stats::Item::Data,std::allocator<RBX::Stats::Item::Data>>>,RBX::Stats::Item::Data const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX5Stats4Item4DataESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
pub fn stub_385630() -> ! {
    todo!("0x385630 std::vector<RBX::Stats::Item::Data,std::allocator<RBX::Stats::Item::Data>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Stats::Item::Data*,std::vector<RBX::Stats::Item::Data,std::allocator<RBX::Stats::Item::Data>>>,RBX::Stats::Item::Data const&)")
}

// 0x385770 — __ZN3RBX5Stats4Item9addChildEPS1_
#[doc(alias = "RBX::Stats::Item::addChild(RBX::Stats::Item *)")]
#[doc(alias = "__ZN3RBX5Stats4Item9addChildEPS1_")]
pub fn stub_385770() -> ! {
    todo!("0x385770 RBX::Stats::Item::addChild(RBX::Stats::Item *)")
}

// 0x3857bc — __ZN3RBX5Stats4Item12removeChildEPS1_
#[doc(alias = "RBX::Stats::Item::removeChild(RBX::Stats::Item *)")]
#[doc(alias = "__ZN3RBX5Stats4Item12removeChildEPS1_")]
pub fn stub_3857bc() -> ! {
    todo!("0x3857bc RBX::Stats::Item::removeChild(RBX::Stats::Item *)")
}

// 0x3858a8 — __ZN3RBX5Stats13ItemArbiterImp8addChildEPS1_
#[doc(alias = "RBX::Stats::ItemArbiterImp::addChild(RBX::Stats::Item *)")]
#[doc(alias = "__ZN3RBX5Stats13ItemArbiterImp8addChildEPS1_")]
pub fn stub_3858a8() -> ! {
    todo!("0x3858a8 RBX::Stats::ItemArbiterImp::addChild(RBX::Stats::Item *)")
}

// 0x3858ac — __ZN3RBX5Stats13ItemArbiterImp11removeChildEPS1_
#[doc(alias = "RBX::Stats::ItemArbiterImp::removeChild(RBX::Stats::Item *)")]
#[doc(alias = "__ZN3RBX5Stats13ItemArbiterImp11removeChildEPS1_")]
pub fn stub_3858ac() -> ! {
    todo!("0x3858ac RBX::Stats::ItemArbiterImp::removeChild(RBX::Stats::Item *)")
}

// 0x3858b0 — __ZN3RBX5Stats4Item12getChildrenEv
#[doc(alias = "RBX::Stats::Item::getChildren(void)")]
#[doc(alias = "__ZN3RBX5Stats4Item12getChildrenEv")]
pub fn stub_3858b0() -> ! {
    todo!("0x3858b0 RBX::Stats::Item::getChildren(void)")
}

// 0x3858b4 — __ZNK3RBX5Stats4Item12getChildrenEv
#[doc(alias = "RBX::Stats::Item::getChildren(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item12getChildrenEv")]
pub fn stub_3858b4() -> ! {
    todo!("0x3858b4 RBX::Stats::Item::getChildren(void)const")
}

// 0x385948 — __ZN3RBX5Stats4Item7setNameERKSs
#[doc(alias = "RBX::Stats::Item::setName(std::string const&)")]
#[doc(alias = "__ZN3RBX5Stats4Item7setNameERKSs")]
pub fn stub_385948() -> ! {
    todo!("0x385948 RBX::Stats::Item::setName(std::string const&)")
}

// 0x3859b4 — __ZN3RBX5Stats4Item16setAvgWindowSizeEi
#[doc(alias = "RBX::Stats::Item::setAvgWindowSize(int)")]
#[doc(alias = "__ZN3RBX5Stats4Item16setAvgWindowSizeEi")]
pub fn stub_3859b4() -> ! {
    todo!("0x3859b4 RBX::Stats::Item::setAvgWindowSize(int)")
}

// 0x385a34 — __ZN3RBX5Stats4Item7setDataEf
#[doc(alias = "RBX::Stats::Item::setData(float)")]
#[doc(alias = "__ZN3RBX5Stats4Item7setDataEf")]
pub fn stub_385a34() -> ! {
    todo!("0x385a34 RBX::Stats::Item::setData(float)")
}

// 0x385ab4 — __ZN3RBX5Stats4Item14setComputeFuncERKN5boost8functionIFdRKNS1_4DataEEE
#[doc(alias = "RBX::Stats::Item::setComputeFunc(boost::function<double ()(RBX::Stats::Item::Data const&)> const&)")]
#[doc(alias = "__ZN3RBX5Stats4Item14setComputeFuncERKN5boost8functionIFdRKNS1_4DataEEE")]
pub fn stub_385ab4() -> ! {
    todo!("0x385ab4 RBX::Stats::Item::setComputeFunc(boost::function<double ()(RBX::Stats::Item::Data const&)> const&)")
}

// 0x385b6c — __ZN3RBX5Stats4Item18setBoundComputeFuncERKN5boost8functionIFdRKNS1_4DataEEE
#[doc(alias = "RBX::Stats::Item::setBoundComputeFunc(boost::function<double ()(RBX::Stats::Item::Data const&)> const&)")]
#[doc(alias = "__ZN3RBX5Stats4Item18setBoundComputeFuncERKN5boost8functionIFdRKNS1_4DataEEE")]
pub fn stub_385b6c() -> ! {
    todo!("0x385b6c RBX::Stats::Item::setBoundComputeFunc(boost::function<double ()(RBX::Stats::Item::Data const&)> const&)")
}

// 0x385c10 — __ZN3RBX5Stats4Item19setBoundComputeFuncERKN5boost8functionIFdRKNS1_4DataEEES2_
#[doc(alias = "RBX::Stats::Item::setBoundComputeFunc(boost::function<double ()(RBX::Stats::Item::Data const&)> const&,boost::function<double ()(RBX::Stats::Item::Data const&)> const&)")]
#[doc(alias = "__ZN3RBX5Stats4Item19setBoundComputeFuncERKN5boost8functionIFdRKNS1_4DataEEES2_")]
pub fn stub_385c10() -> ! {
    todo!("0x385c10 RBX::Stats::Item::setBoundComputeFunc(boost::function<double ()(RBX::Stats::Item::Data const&)> const&,boost::function<double ()(RBX::Stats::Item::Data const&)> const&)")
}

// 0x385cac — __ZN3RBX5Stats4Item10setMinBoundEd
#[doc(alias = "RBX::Stats::Item::setMinBound(double)")]
#[doc(alias = "__ZN3RBX5Stats4Item10setMinBoundEd")]
pub fn stub_385cac() -> ! {
    todo!("0x385cac RBX::Stats::Item::setMinBound(double)")
}

// 0x385d04 — __ZN3RBX5Stats4Item10setMaxBoundEd
#[doc(alias = "RBX::Stats::Item::setMaxBound(double)")]
#[doc(alias = "__ZN3RBX5Stats4Item10setMaxBoundEd")]
pub fn stub_385d04() -> ! {
    todo!("0x385d04 RBX::Stats::Item::setMaxBound(double)")
}

// 0x385d5c — __ZN3RBX5Stats4Item13setBoundWeightEf
#[doc(alias = "RBX::Stats::Item::setBoundWeight(float)")]
#[doc(alias = "__ZN3RBX5Stats4Item13setBoundWeightEf")]
pub fn stub_385d5c() -> ! {
    todo!("0x385d5c RBX::Stats::Item::setBoundWeight(float)")
}

// 0x385db4 — __ZN3RBX5Stats4Item12setDataFactorEf
#[doc(alias = "RBX::Stats::Item::setDataFactor(float)")]
#[doc(alias = "__ZN3RBX5Stats4Item12setDataFactorEf")]
pub fn stub_385db4() -> ! {
    todo!("0x385db4 RBX::Stats::Item::setDataFactor(float)")
}

// 0x385e0c — __ZN3RBX5Stats4Item10setLongNameERKSs
#[doc(alias = "RBX::Stats::Item::setLongName(std::string const&)")]
#[doc(alias = "__ZN3RBX5Stats4Item10setLongNameERKSs")]
pub fn stub_385e0c() -> ! {
    todo!("0x385e0c RBX::Stats::Item::setLongName(std::string const&)")
}

// 0x385e3c — __ZNK3RBX5Stats4Item7getNameEv
#[doc(alias = "RBX::Stats::Item::getName(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item7getNameEv")]
pub fn stub_385e3c() -> ! {
    todo!("0x385e3c RBX::Stats::Item::getName(void)const")
}

// 0x385e64 — __ZNK3RBX5Stats4Item11getLongNameEv
#[doc(alias = "RBX::Stats::Item::getLongName(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item11getLongNameEv")]
pub fn stub_385e64() -> ! {
    todo!("0x385e64 RBX::Stats::Item::getLongName(void)const")
}

// 0x385e8c — __ZNK3RBX5Stats4Item7getDataEv
#[doc(alias = "RBX::Stats::Item::getData(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item7getDataEv")]
pub fn stub_385e8c() -> ! {
    todo!("0x385e8c RBX::Stats::Item::getData(void)const")
}

// 0x385eb4 — __ZNK3RBX5Stats4Item10getMinBoundEv
#[doc(alias = "RBX::Stats::Item::getMinBound(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item10getMinBoundEv")]
pub fn stub_385eb4() -> ! {
    todo!("0x385eb4 RBX::Stats::Item::getMinBound(void)const")
}

// 0x385edc — __ZNK3RBX5Stats4Item10getMaxBoundEv
#[doc(alias = "RBX::Stats::Item::getMaxBound(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item10getMaxBoundEv")]
pub fn stub_385edc() -> ! {
    todo!("0x385edc RBX::Stats::Item::getMaxBound(void)const")
}

// 0x385f04 — __ZNK3RBX5Stats4Item14getBoundWeightEv
#[doc(alias = "RBX::Stats::Item::getBoundWeight(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item14getBoundWeightEv")]
pub fn stub_385f04() -> ! {
    todo!("0x385f04 RBX::Stats::Item::getBoundWeight(void)const")
}

// 0x385f2c — __ZNK3RBX5Stats4Item12getDataFactorEv
#[doc(alias = "RBX::Stats::Item::getDataFactor(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item12getDataFactorEv")]
pub fn stub_385f2c() -> ! {
    todo!("0x385f2c RBX::Stats::Item::getDataFactor(void)const")
}

// 0x385f54 — __ZNK3RBX5Stats4Item13getBoundWeightEv
#[doc(alias = "RBX::Stats::Item::getBoundWeight(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item13getBoundWeightEv")]
pub fn stub_385f54() -> ! {
    todo!("0x385f54 RBX::Stats::Item::getBoundWeight(void)const")
}

// 0x385f7c — __ZNK3RBX5Stats4Item16getAvgWindowSizeEv
#[doc(alias = "RBX::Stats::Item::getAvgWindowSize(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item16getAvgWindowSizeEv")]
pub fn stub_385f7c() -> ! {
    todo!("0x385f7c RBX::Stats::Item::getAvgWindowSize(void)const")
}

// 0x385fa4 — __ZNK3RBX5Stats4Item11getChildrenEv
#[doc(alias = "RBX::Stats::Item::getChildren(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item11getChildrenEv")]
pub fn stub_385fa4() -> ! {
    todo!("0x385fa4 RBX::Stats::Item::getChildren(void)const")
}

// 0x385fcc — __ZNK3RBX5Stats4Item13getParentItemsEv
#[doc(alias = "RBX::Stats::Item::getParentItems(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item13getParentItemsEv")]
pub fn stub_385fcc() -> ! {
    todo!("0x385fcc RBX::Stats::Item::getParentItems(void)const")
}

// 0x385fe4 — __ZNK3RBX5Stats4Item7getTypeEv
#[doc(alias = "RBX::Stats::Item::getType(void)const")]
#[doc(alias = "__ZNK3RBX5Stats4Item7getTypeEv")]
pub fn stub_385fe4() -> ! {
    todo!("0x385fe4 RBX::Stats::Item::getType(void)const")
}
