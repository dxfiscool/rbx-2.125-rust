//! core shard IA — 100 core stubs EA-sorted, continuation after HZ 0x3643a0 (EA-sorted ascending, next 100 uncovered).
//! Source: `ida/export.json` filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 100 uncovered after 0x3643a0.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::HeartbeatTask::~HeartbeatTask()")]
// 0x3692bc — __ZN3RBX13HeartbeatTaskD1Ev
pub fn stub_3692bc() -> ! {
    todo!("0x3692bc __ZN3RBX13HeartbeatTaskD1Ev")
}

#[doc(alias = "RBX::HeartbeatTask::~HeartbeatTask()")]
// 0x3693b8 — __ZN3RBX13HeartbeatTaskD0Ev
pub fn stub_3693b8() -> ! {
    todo!("0x3693b8 __ZN3RBX13HeartbeatTaskD0Ev")
}

#[doc(alias = "RBX::HeartbeatTask::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x3694c8 — __ZN3RBX13HeartbeatTask9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_3694c8() -> ! {
    todo!("0x3694c8 __ZN3RBX13HeartbeatTask9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::HeartbeatTask::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x3694e4 — __ZN3RBX13HeartbeatTask5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_3694e4() -> ! {
    todo!("0x3694e4 __ZN3RBX13HeartbeatTask5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::PhysicsJob::~PhysicsJob()")]
// 0x3698b8 — __ZN3RBX10PhysicsJobD1Ev
pub fn stub_3698b8() -> ! {
    todo!("0x3698b8 __ZN3RBX10PhysicsJobD1Ev")
}

#[doc(alias = "RBX::PhysicsJob::~PhysicsJob()")]
// 0x369988 — __ZN3RBX10PhysicsJobD0Ev
pub fn stub_369988() -> ! {
    todo!("0x369988 __ZN3RBX10PhysicsJobD0Ev")
}

#[doc(alias = "RBX::PhysicsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x369a70 — __ZN3RBX10PhysicsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_369a70() -> ! {
    todo!("0x369a70 __ZN3RBX10PhysicsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::PhysicsJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x369ab0 — __ZN3RBX10PhysicsJob5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_369ab0() -> ! {
    todo!("0x369ab0 __ZN3RBX10PhysicsJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::PhysicsJob::getDesiredConcurrencyCount(void)const")]
// 0x369ac8 — __ZNK3RBX10PhysicsJob26getDesiredConcurrencyCountEv
pub fn stub_369ac8() -> ! {
    todo!("0x369ac8 __ZNK3RBX10PhysicsJob26getDesiredConcurrencyCountEv")
}

#[doc(alias = "global constructor keyed to_a_136")]
// 0x36a0bc — __GLOBAL__I_a_136
pub fn stub_36a0bc() -> ! {
    todo!("0x36a0bc __GLOBAL__I_a_136")
}

#[doc(alias = "RBX::findLocalFile(std::string const&,std::string *)")]
// 0x36a710 — __ZN3RBXL13findLocalFileERKSsPSs
pub fn stub_36a710() -> ! {
    todo!("0x36a710 __ZN3RBXL13findLocalFileERKSsPSs")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
// 0x36b644 — __ZN3RBX20SizeEnforcedLRUCacheISsSsE6insertERKSsS3_m
pub fn stub_36b644() -> ! {
    todo!("0x36b644 __ZN3RBX20SizeEnforcedLRUCacheISsSsE6insertERKSsS3_m")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
// 0x36de5c — __ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m
pub fn stub_36de5c() -> ! {
    todo!("0x36de5c __ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::removeLeastRecentlyUsed(void)")]
// 0x36e3e4 — __ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv
pub fn stub_36e3e4() -> ! {
    todo!("0x36e3e4 __ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::remove(std::string const&)")]
// 0x36e43c — __ZN3RBX8LRUCacheISsSsE6removeERKSs
pub fn stub_36e43c() -> ! {
    todo!("0x36e43c __ZN3RBX8LRUCacheISsSsE6removeERKSs")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>::destroy(std::pair<std::string,std::pair<unsigned long,std::string>>*)")]
// 0x36e558 — __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_
pub fn stub_36e558() -> ! {
    todo!("0x36e558 __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_")
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,std::string>>::pair(std::string const&,std::pair<unsigned long,std::string> const&)")]
// 0x36eb80 — __ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_
pub fn stub_36eb80() -> ! {
    todo!("0x36eb80 __ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_")
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,std::string>> const&)")]
// 0x36ec4c — __ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_36ec4c() -> ! {
    todo!("0x36ec4c __ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::~LRUCache()")]
// 0x3705a0 — __ZN3RBX8LRUCacheISsSsED2Ev
pub fn stub_3705a0() -> ! {
    todo!("0x3705a0 __ZN3RBX8LRUCacheISsSsED2Ev")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::resize(unsigned long)")]
// 0x3706b4 — __ZN3RBX8LRUCacheISsSsE6resizeEm
pub fn stub_3706b4() -> ! {
    todo!("0x3706b4 __ZN3RBX8LRUCacheISsSsE6resizeEm")
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_clear(void)")]
// 0x3706ec — __ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv
pub fn stub_3706ec() -> ! {
    todo!("0x3706ec __ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::LRUCache(void)")]
// 0x370780 — __ZN3RBX8LRUCacheISsSsEC2Ev
pub fn stub_370780() -> ! {
    todo!("0x370780 __ZN3RBX8LRUCacheISsSsEC2Ev")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::resize(unsigned long)")]
// 0x370860 — __ZN3RBX20SizeEnforcedLRUCacheISsSsE6resizeEm
pub fn stub_370860() -> ! {
    todo!("0x370860 __ZN3RBX20SizeEnforcedLRUCacheISsSsE6resizeEm")
}

#[doc(alias = "global constructor keyed to_a_137")]
// 0x371254 — __GLOBAL__I_a_137
pub fn stub_371254() -> ! {
    todo!("0x371254 __GLOBAL__I_a_137")
}

#[doc(alias = "initReverbs(void)")]
// 0x3729c4 — __ZL11initReverbsv
pub fn stub_3729c4() -> ! {
    todo!("0x3729c4 __ZL11initReverbsv")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int>(char const*,int const&)")]
// 0x37da40 — __ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_
pub fn stub_37da40() -> ! {
    todo!("0x37da40 __ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_")
}

#[doc(alias = "global constructor keyed to_a_138")]
// 0x37ead8 — __GLOBAL__I_a_138
pub fn stub_37ead8() -> ! {
    todo!("0x37ead8 __GLOBAL__I_a_138")
}

#[doc(alias = "global constructor keyed to_a_139")]
// 0x38039c — __GLOBAL__I_a_139
pub fn stub_38039c() -> ! {
    todo!("0x38039c __GLOBAL__I_a_139")
}

#[doc(alias = "RBX::SpanningEdge::getConstChildSpanningNode(void)const")]
// 0x380464 — __ZNK3RBX12SpanningEdge25getConstChildSpanningNodeEv
pub fn stub_380464() -> ! {
    todo!("0x380464 __ZNK3RBX12SpanningEdge25getConstChildSpanningNodeEv")
}

#[doc(alias = "RBX::SpanningEdge::getChildSpanningNode(void)")]
// 0x3804e0 — __ZN3RBX12SpanningEdge20getChildSpanningNodeEv
pub fn stub_3804e0() -> ! {
    todo!("0x3804e0 __ZN3RBX12SpanningEdge20getChildSpanningNodeEv")
}

#[doc(alias = "RBX::SpanningEdge::getParentSpanningNode(void)")]
// 0x3804e4 — __ZN3RBX12SpanningEdge21getParentSpanningNodeEv
pub fn stub_3804e4() -> ! {
    todo!("0x3804e4 __ZN3RBX12SpanningEdge21getParentSpanningNodeEv")
}

#[doc(alias = "RBX::SpanningEdge::removeFromSpanningTree(void)")]
// 0x3804fc — __ZN3RBX12SpanningEdge22removeFromSpanningTreeEv
pub fn stub_3804fc() -> ! {
    todo!("0x3804fc __ZN3RBX12SpanningEdge22removeFromSpanningTreeEv")
}

#[doc(alias = "RBX::SpanningEdge::addToSpanningTree(RBX::SpanningNode *)")]
// 0x380568 — __ZN3RBX12SpanningEdge17addToSpanningTreeEPNS_12SpanningNodeE
pub fn stub_380568() -> ! {
    todo!("0x380568 __ZN3RBX12SpanningEdge17addToSpanningTreeEPNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningEdge::inSpanningTree(void)const")]
// 0x3806bc — __ZNK3RBX12SpanningEdge14inSpanningTreeEv
pub fn stub_3806bc() -> ! {
    todo!("0x3806bc __ZNK3RBX12SpanningEdge14inSpanningTreeEv")
}

#[doc(alias = "global constructor keyed to_a_140")]
// 0x3806e4 — __GLOBAL__I_a_140
pub fn stub_3806e4() -> ! {
    todo!("0x3806e4 __GLOBAL__I_a_140")
}

#[doc(alias = "RBX::SpanningNode::setEdgeToParent(RBX::SpanningEdge *)")]
// 0x3807ac — __ZN3RBX12SpanningNode15setEdgeToParentEPNS_12SpanningEdgeE
pub fn stub_3807ac() -> ! {
    todo!("0x3807ac __ZN3RBX12SpanningNode15setEdgeToParentEPNS_12SpanningEdgeE")
}

#[doc(alias = "global constructor keyed to_a_141")]
// 0x3807b0 — __GLOBAL__I_a_141
pub fn stub_3807b0() -> ! {
    todo!("0x3807b0 __GLOBAL__I_a_141")
}

#[doc(alias = "RBX::SpanningTree::SpanningTree(void)")]
// 0x380878 — __ZN3RBX12SpanningTreeC2Ev
pub fn stub_380878() -> ! {
    todo!("0x380878 __ZN3RBX12SpanningTreeC2Ev")
}

#[doc(alias = "RBX::SpanningTree::~SpanningTree()")]
// 0x38089c — __ZN3RBX12SpanningTreeD2Ev
pub fn stub_38089c() -> ! {
    todo!("0x38089c __ZN3RBX12SpanningTreeD2Ev")
}

#[doc(alias = "RBX::SpanningTree::insertSpanningTreeEdge(RBX::SpanningEdge *)")]
// 0x3809c4 — __ZN3RBX12SpanningTree22insertSpanningTreeEdgeEPNS_12SpanningEdgeE
pub fn stub_3809c4() -> ! {
    todo!("0x3809c4 __ZN3RBX12SpanningTree22insertSpanningTreeEdgeEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningEdge *,RBX::SpanningEdge *&,int &)")]
// 0x380a6c — __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningEdgeERS2_Ri
pub fn stub_380a6c() -> ! {
    todo!("0x380a6c __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningEdgeERS2_Ri")
}

#[doc(alias = "RBX::SpanningTree::swapTree(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x380abc — __ZN3RBX12SpanningTree8swapTreeEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
pub fn stub_380abc() -> ! {
    todo!("0x380abc __ZN3RBX12SpanningTree8swapTreeEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningTree::removeSpanningTreeEdge(RBX::SpanningEdge *)")]
// 0x380b30 — __ZN3RBX12SpanningTree22removeSpanningTreeEdgeEPNS_12SpanningEdgeE
pub fn stub_380b30() -> ! {
    todo!("0x380b30 __ZN3RBX12SpanningTree22removeSpanningTreeEdgeEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningTree::findHeaviestDownstream(RBX::SpanningNode *,RBX::SpanningNode *&)")]
// 0x380bac — __ZN3RBX12SpanningTree22findHeaviestDownstreamEPNS_12SpanningNodeERS2_
pub fn stub_380bac() -> ! {
    todo!("0x380bac __ZN3RBX12SpanningTree22findHeaviestDownstreamEPNS_12SpanningNodeERS2_")
}

#[doc(alias = "RBX::SpanningTree::swap(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x380cdc — __ZN3RBX12SpanningTree4swapEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
pub fn stub_380cdc() -> ! {
    todo!("0x380cdc __ZN3RBX12SpanningTree4swapEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningTree::removeEdge(RBX::SpanningEdge *)")]
// 0x380d50 — __ZN3RBX12SpanningTree10removeEdgeEPNS_12SpanningEdgeE
pub fn stub_380d50() -> ! {
    todo!("0x380d50 __ZN3RBX12SpanningTree10removeEdgeEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningTree::addEdge(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x380e34 — __ZN3RBX12SpanningTree7addEdgeEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_380e34() -> ! {
    todo!("0x380e34 __ZN3RBX12SpanningTree7addEdgeEPNS_12SpanningEdgeEPNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningTree::findAndDeactivateEdges(RBX::SpanningNode *,RBX::SpanningEdge *,G3D::Array<RBX::SpanningEdge *,10,32ul> &)")]
// 0x380f1c — __ZN3RBX12SpanningTree22findAndDeactivateEdgesEPNS_12SpanningNodeEPNS_12SpanningEdgeERN3G3D5ArrayIS4_Li10ELm32EEE
pub fn stub_380f1c() -> ! {
    todo!("0x380f1c __ZN3RBX12SpanningTree22findAndDeactivateEdgesEPNS_12SpanningNodeEPNS_12SpanningEdgeERN3G3D5ArrayIS4_Li10ELm32EEE")
}

#[doc(alias = "RBX::SpanningTree::activateEdges(RBX::SpanningNode *,G3D::Array<RBX::SpanningEdge *,10,32ul> const&)")]
// 0x38103c — __ZN3RBX12SpanningTree13activateEdgesEPNS_12SpanningNodeERKN3G3D5ArrayIPNS_12SpanningEdgeELi10ELm32EEE
pub fn stub_38103c() -> ! {
    todo!("0x38103c __ZN3RBX12SpanningTree13activateEdgesEPNS_12SpanningNodeERKN3G3D5ArrayIPNS_12SpanningEdgeELi10ELm32EEE")
}

#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningNode *,RBX::SpanningNode *,int,int,RBX::SpanningEdge *&,int &)")]
// 0x381120 — __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningNodeES2_iiRPNS_12SpanningEdgeERi
pub fn stub_381120() -> ! {
    todo!("0x381120 __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningNodeES2_iiRPNS_12SpanningEdgeERi")
}

#[doc(alias = "RBX::SpanningTree::buildDownstreamTree(RBX::SpanningNode *,std::set<RBX::SpanningNode *,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>> &)")]
// 0x38120c — __ZN3RBX12SpanningTree19buildDownstreamTreeEPNS_12SpanningNodeERSt3setIS2_St4lessIS2_ESaIS2_EE
pub fn stub_38120c() -> ! {
    todo!("0x38120c __ZN3RBX12SpanningTree19buildDownstreamTreeEPNS_12SpanningNodeERSt3setIS2_St4lessIS2_ESaIS2_EE")
}

#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::append(RBX::SpanningEdge * const&)")]
// 0x3812ac — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6appendERKS3_
pub fn stub_3812ac() -> ! {
    todo!("0x3812ac __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6appendERKS3_")
}

#[doc(alias = "RBX::SpanningNode::getDepth(RBX::SpanningNode*)")]
// 0x381308 — __ZN3RBX12SpanningNode8getDepthEPS0_
pub fn stub_381308() -> ! {
    todo!("0x381308 __ZN3RBX12SpanningNode8getDepthEPS0_")
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x381328 — __ZN3RBX12SpanningTree20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_381328() -> ! {
    todo!("0x381328 __ZN3RBX12SpanningTree20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
// 0x38132c — __ZN3RBX12SpanningTree19onSpanningEdgeAddedEPNS_12SpanningEdgeE
pub fn stub_38132c() -> ! {
    todo!("0x38132c __ZN3RBX12SpanningTree19onSpanningEdgeAddedEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
// 0x381330 — __ZN3RBX12SpanningTree22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
pub fn stub_381330() -> ! {
    todo!("0x381330 __ZN3RBX12SpanningTree22onSpanningEdgeRemovingEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x381334 — __ZN3RBX12SpanningTree21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_381334() -> ! {
    todo!("0x381334 __ZN3RBX12SpanningTree21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningTree::validateTree(RBX::SpanningNode *)")]
// 0x381338 — __ZN3RBX12SpanningTree12validateTreeEPNS_12SpanningNodeE
pub fn stub_381338() -> ! {
    todo!("0x381338 __ZN3RBX12SpanningTree12validateTreeEPNS_12SpanningNodeE")
}

#[doc(alias = "RBX::FindHeaviest::operator()(RBX::SpanningNode *,RBX::SpanningEdge *)")]
// 0x38133c — __ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE
pub fn stub_38133c() -> ! {
    todo!("0x38133c __ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE")
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert_unique(RBX::SpanningNode * const&)")]
// 0x3813bc — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_3813bc() -> ! {
    todo!("0x3813bc __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::SpanningNode * const&)")]
// 0x381424 — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_381424() -> ! {
    todo!("0x381424 __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::resize(int,bool)")]
// 0x38147c — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6resizeEib
pub fn stub_38147c() -> ! {
    todo!("0x38147c __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6resizeEib")
}

#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::realloc(int)")]
// 0x381534 — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE7reallocEi
pub fn stub_381534() -> ! {
    todo!("0x381534 __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE7reallocEi")
}

#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::~Array()")]
// 0x38171c — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EED2Ev
pub fn stub_38171c() -> ! {
    todo!("0x38171c __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EED2Ev")
}

#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::Array(void)")]
// 0x3817f0 — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EEC2Ev
pub fn stub_3817f0() -> ! {
    todo!("0x3817f0 __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EEC2Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_erase(std::_Rb_tree_node<RBX::SpanningNode *> *)")]
// 0x3818e0 — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_3818e0() -> ! {
    todo!("0x3818e0 __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "global constructor keyed to_a_142")]
// 0x381908 — __GLOBAL__I_a_142
pub fn stub_381908() -> ! {
    todo!("0x381908 __GLOBAL__I_a_142")
}

#[doc(alias = "RBX::StandardOut::singleton(void)")]
// 0x3819d0 — __ZN3RBX11StandardOut9singletonEv
pub fn stub_3819d0() -> ! {
    todo!("0x3819d0 __ZN3RBX11StandardOut9singletonEv")
}

#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,std::exception const&)")]
// 0x381c38 — __ZN3RBX11StandardOut5printENS_11MessageTypeERKSt9exception
pub fn stub_381c38() -> ! {
    todo!("0x381c38 __ZN3RBX11StandardOut5printENS_11MessageTypeERKSt9exception")
}

#[doc(alias = "RBX::StandardOut::printf(RBX::MessageType,char const*,...)")]
// 0x381c58 — __ZN3RBX11StandardOut6printfENS_11MessageTypeEPKcz
pub fn stub_381c58() -> ! {
    todo!("0x381c58 __ZN3RBX11StandardOut6printfENS_11MessageTypeEPKcz")
}

#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,std::string const&)")]
// 0x381d88 — __ZN3RBX11StandardOut5printENS_11MessageTypeERKSs
pub fn stub_381d88() -> ! {
    todo!("0x381d88 __ZN3RBX11StandardOut5printENS_11MessageTypeERKSs")
}

#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,char const*)")]
// 0x3820c4 — __ZN3RBX11StandardOut5printENS_11MessageTypeEPKc
pub fn stub_3820c4() -> ! {
    todo!("0x3820c4 __ZN3RBX11StandardOut5printENS_11MessageTypeEPKc")
}

#[doc(alias = "RBX::StandardOut::~StandardOut()")]
// 0x3827e8 — __ZN3RBX11StandardOutD2Ev
pub fn stub_3827e8() -> ! {
    todo!("0x3827e8 __ZN3RBX11StandardOutD2Ev")
}

#[doc(alias = "RBX::StandardOutMessage::StandardOutMessage(RBX::MessageType,char const*)")]
// 0x382b38 — __ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc
pub fn stub_382b38() -> ! {
    todo!("0x382b38 __ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc")
}

#[doc(alias = "RBX::StandardOut::StandardOut(void)")]
// 0x382bfc — __ZN3RBX11StandardOutC2Ev
pub fn stub_382bfc() -> ! {
    todo!("0x382bfc __ZN3RBX11StandardOutC2Ev")
}

#[doc(alias = "global constructor keyed to_a_143")]
// 0x382d18 — __GLOBAL__I_a_143
pub fn stub_382d18() -> ! {
    todo!("0x382d18 __GLOBAL__I_a_143")
}

#[doc(alias = "SetBaseURL(std::string const&)")]
// 0x382de0 — __Z10SetBaseURLRKSs
pub fn stub_382de0() -> ! {
    todo!("0x382de0 __Z10SetBaseURLRKSs")
}

#[doc(alias = "GetBaseURL(void)")]
// 0x382df4 — __Z10GetBaseURLv
pub fn stub_382df4() -> ! {
    todo!("0x382df4 __Z10GetBaseURLv")
}

#[doc(alias = "RBX::Http::urlEncode(std::string)")]
// 0x382e04 — __ZN3RBX4Http9urlEncodeESs
pub fn stub_382e04() -> ! {
    todo!("0x382e04 __ZN3RBX4Http9urlEncodeESs")
}

#[doc(alias = "FetchLocalClientSettingsData(char const*,SimpleJSON *)")]
// 0x382f9c — __Z28FetchLocalClientSettingsDataPKcP10SimpleJSON
pub fn stub_382f9c() -> ! {
    todo!("0x382f9c __Z28FetchLocalClientSettingsDataPKcP10SimpleJSON")
}

#[doc(alias = "LoadClientSettingsFromString(char const*,std::string const&,SimpleJSON *)")]
// 0x3834bc — __Z28LoadClientSettingsFromStringPKcRKSsP10SimpleJSON
pub fn stub_3834bc() -> ! {
    todo!("0x3834bc __Z28LoadClientSettingsFromStringPKcRKSsP10SimpleJSON")
}

#[doc(alias = "FetchClientSettingsData(char const*,char const*,SimpleJSON *)")]
// 0x383538 — __Z23FetchClientSettingsDataPKcS0_P10SimpleJSON
pub fn stub_383538() -> ! {
    todo!("0x383538 __Z23FetchClientSettingsDataPKcS0_P10SimpleJSON")
}

#[doc(alias = "FetchClientSettingsData(char const*,char const*,std::string *)")]
// 0x38367c — __Z23FetchClientSettingsDataPKcS0_PSs
pub fn stub_38367c() -> ! {
    todo!("0x38367c __Z23FetchClientSettingsDataPKcS0_PSs")
}

#[doc(alias = "ReportStatisticPost(std::string const&,std::string const&,std::string const&,char const*,char const*,char const*,char const*)")]
// 0x383c54 — __Z19ReportStatisticPostRKSsS0_S0_PKcS2_S2_S2_
pub fn stub_383c54() -> ! {
    todo!("0x383c54 __Z19ReportStatisticPostRKSsS0_S0_PKcS2_S2_S2_")
}

#[doc(alias = "ReportStatistic(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,std::string const&)")]
// 0x384ae0 — __Z15ReportStatisticRKSsS0_S0_S0_S0_S0_
pub fn stub_384ae0() -> ! {
    todo!("0x384ae0 __Z15ReportStatisticRKSsS0_S0_S0_S0_S0_")
}

#[doc(alias = "DontCareResponse(std::string *,std::exception *)")]
// 0x384c38 — __Z16DontCareResponsePSsPSt9exception
pub fn stub_384c38() -> ! {
    todo!("0x384c38 __Z16DontCareResponsePSsPSt9exception")
}

#[doc(alias = "global constructor keyed to_a_144")]
// 0x384c44 — __GLOBAL__I_a_144
pub fn stub_384c44() -> ! {
    todo!("0x384c44 __GLOBAL__I_a_144")
}

#[doc(alias = "RBX::IStepped::onServiceProviderIStepped(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x384d34 — __ZN3RBX8IStepped25onServiceProviderISteppedEPNS_15ServiceProviderES2_
pub fn stub_384d34() -> ! {
    todo!("0x384d34 __ZN3RBX8IStepped25onServiceProviderISteppedEPNS_15ServiceProviderES2_")
}

#[doc(alias = "global constructor keyed to_a_145")]
// 0x38587c — __GLOBAL__I_a_145
pub fn stub_38587c() -> ! {
    todo!("0x38587c __GLOBAL__I_a_145")
}

#[doc(alias = "RBX::SystemAddress::operator==(RBX::SystemAddress const&)const")]
// 0x385a3c — __ZNK3RBX13SystemAddresseqERKS0_
pub fn stub_385a3c() -> ! {
    todo!("0x385a3c __ZNK3RBX13SystemAddresseqERKS0_")
}

#[doc(alias = "RBX::SystemAddress::operator!=(RBX::SystemAddress const&)const")]
// 0x385a58 — __ZNK3RBX13SystemAddressneERKS0_
pub fn stub_385a58() -> ! {
    todo!("0x385a58 __ZNK3RBX13SystemAddressneERKS0_")
}

#[doc(alias = "RBX::SystemAddress::operator<(RBX::SystemAddress const&)const")]
// 0x385a78 — __ZNK3RBX13SystemAddressltERKS0_
pub fn stub_385a78() -> ! {
    todo!("0x385a78 __ZNK3RBX13SystemAddressltERKS0_")
}

#[doc(alias = "RBX::BaseThreadPool::BaseThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy,RBX::BaseThreadPool::PoolData *)")]
// 0x385a9c — __ZN3RBX14BaseThreadPoolC2EiNS0_14ShutdownPolicyEPNS0_8PoolDataE
pub fn stub_385a9c() -> ! {
    todo!("0x385a9c __ZN3RBX14BaseThreadPoolC2EiNS0_14ShutdownPolicyEPNS0_8PoolDataE")
}

#[doc(alias = "RBX::BaseThreadPool::getThreadCount(void)const")]
// 0x385fe4 — __ZNK3RBX14BaseThreadPool14getThreadCountEv
pub fn stub_385fe4() -> ! {
    todo!("0x385fe4 __ZNK3RBX14BaseThreadPool14getThreadCountEv")
}

#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// 0x385fe8 — __ZN3RBX14BaseThreadPoolD0Ev
pub fn stub_385fe8() -> ! {
    todo!("0x385fe8 __ZN3RBX14BaseThreadPoolD0Ev")
}

#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// 0x386088 — __ZN3RBX14BaseThreadPoolD1Ev
pub fn stub_386088() -> ! {
    todo!("0x386088 __ZN3RBX14BaseThreadPoolD1Ev")
}

#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// 0x38608c — __ZN3RBX14BaseThreadPoolD2Ev
pub fn stub_38608c() -> ! {
    todo!("0x38608c __ZN3RBX14BaseThreadPoolD2Ev")
}

#[doc(alias = "RBX::BaseThreadPool::taskAdded(void)")]
// 0x3864e4 — __ZN3RBX14BaseThreadPool9taskAddedEv
pub fn stub_3864e4() -> ! {
    todo!("0x3864e4 __ZN3RBX14BaseThreadPool9taskAddedEv")
}

#[doc(alias = "RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// 0x3865f4 — __ZN3RBX10ThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
pub fn stub_3865f4() -> ! {
    todo!("0x3865f4 __ZN3RBX10ThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE")
}

#[doc(alias = "RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// 0x3865f8 — __ZN3RBX10ThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
pub fn stub_3865f8() -> ! {
    todo!("0x3865f8 __ZN3RBX10ThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE")
}
