//! core shard AR — 100 core stubs EA-sorted, next uncovered after AQ 0x3616a8..0x3616b0 (strict RBX|boost|std earliest gap).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x3616a8.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "non-virtual thunk toRBX::RunService::~RunService()")]
// 0x3616b0 — __ZThn36_N3RBX10RunServiceD1Ev
// was: `non-virtual thunk to'RBX::RunService::~RunService()
pub fn stub_0x3616b0() -> ! {
    todo!("0x3616b0 __ZThn36_N3RBX10RunServiceD1Ev")
}

#[doc(alias = "RBX::RunService::getPhysicsJob(void)")]
// 0x3616b8 — __ZN3RBX10RunService13getPhysicsJobEv
pub fn stub_0x3616b8() -> ! {
    todo!("0x3616b8 __ZN3RBX10RunService13getPhysicsJobEv")
}

#[doc(alias = "RBX::RunService::raiseHeartbeat(double,RBX::Time::Interval const&)")]
// 0x3616bc — __ZN3RBX10RunService14raiseHeartbeatEdRKNS_4Time8IntervalE
pub fn stub_0x3616bc() -> ! {
    todo!("0x3616bc __ZN3RBX10RunService14raiseHeartbeatEdRKNS_4Time8IntervalE")
}

#[doc(alias = "RBX::RunService::gameStepped(double)")]
// 0x361750 — __ZN3RBX10RunService11gameSteppedEd
pub fn stub_0x361750() -> ! {
    todo!("0x361750 __ZN3RBX10RunService11gameSteppedEd")
}

#[doc(alias = "RBX::RunService::setRunState(RBX::RunState)")]
// 0x3617b8 — __ZN3RBX10RunService11setRunStateENS_8RunStateE
pub fn stub_0x3617b8() -> ! {
    todo!("0x3617b8 __ZN3RBX10RunService11setRunStateENS_8RunStateE")
}

#[doc(alias = "RBX::RunService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x361818 — __ZN3RBX10RunService17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_0x361818() -> ! {
    todo!("0x361818 __ZN3RBX10RunService17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::RunService::smoothFps(void)const")]
// 0x361824 — __ZNK3RBX10RunService9smoothFpsEv
pub fn stub_0x361824() -> ! {
    todo!("0x361824 __ZNK3RBX10RunService9smoothFpsEv")
}

#[doc(alias = "RBX::RunService::heartbeatFps(void)const")]
// 0x36182c — __ZNK3RBX10RunService12heartbeatFpsEv
pub fn stub_0x36182c() -> ! {
    todo!("0x36182c __ZNK3RBX10RunService12heartbeatFpsEv")
}

#[doc(alias = "RBX::RunService::physicsAverageStep(void)const")]
// 0x361834 — __ZNK3RBX10RunService18physicsAverageStepEv
pub fn stub_0x361834() -> ! {
    todo!("0x361834 __ZNK3RBX10RunService18physicsAverageStepEv")
}

#[doc(alias = "RBX::RunService::heartbeatAverageStep(void)const")]
// 0x36183c — __ZNK3RBX10RunService20heartbeatAverageStepEv
pub fn stub_0x36183c() -> ! {
    todo!("0x36183c __ZNK3RBX10RunService20heartbeatAverageStepEv")
}

#[doc(alias = "RBX::RunService::physicsCpuFraction(void)const")]
// 0x361844 — __ZNK3RBX10RunService18physicsCpuFractionEv
pub fn stub_0x361844() -> ! {
    todo!("0x361844 __ZNK3RBX10RunService18physicsCpuFractionEv")
}

#[doc(alias = "RBX::RunService::heartbeatCpuFraction(void)const")]
// 0x36184c — __ZNK3RBX10RunService20heartbeatCpuFractionEv
pub fn stub_0x36184c() -> ! {
    todo!("0x36184c __ZNK3RBX10RunService20heartbeatCpuFractionEv")
}

#[doc(alias = "RBX::RunService::run(void)")]
// 0x3618a0 — __ZN3RBX10RunService3runEv
pub fn stub_0x3618a0() -> ! {
    todo!("0x3618a0 __ZN3RBX10RunService3runEv")
}

#[doc(alias = "RBX::RunService::pause(void)")]
// 0x3618cc — __ZN3RBX10RunService5pauseEv
pub fn stub_0x3618cc() -> ! {
    todo!("0x3618cc __ZN3RBX10RunService5pauseEv")
}

#[doc(alias = "RBX::RunService::stop(void)")]
// 0x3618d4 — __ZN3RBX10RunService4stopEv
pub fn stub_0x3618d4() -> ! {
    todo!("0x3618d4 __ZN3RBX10RunService4stopEv")
}

#[doc(alias = "RBX::HeartbeatTask::~HeartbeatTask()")]
// 0x3692bc — __ZN3RBX13HeartbeatTaskD1Ev
pub fn stub_0x3692bc() -> ! {
    todo!("0x3692bc __ZN3RBX13HeartbeatTaskD1Ev")
}

#[doc(alias = "RBX::HeartbeatTask::~HeartbeatTask()")]
// 0x3693b8 — __ZN3RBX13HeartbeatTaskD0Ev
pub fn stub_0x3693b8() -> ! {
    todo!("0x3693b8 __ZN3RBX13HeartbeatTaskD0Ev")
}

#[doc(alias = "RBX::HeartbeatTask::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x3694c8 — __ZN3RBX13HeartbeatTask9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_0x3694c8() -> ! {
    todo!("0x3694c8 __ZN3RBX13HeartbeatTask9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::HeartbeatTask::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x3694e4 — __ZN3RBX13HeartbeatTask5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_0x3694e4() -> ! {
    todo!("0x3694e4 __ZN3RBX13HeartbeatTask5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::PhysicsJob::~PhysicsJob()")]
// 0x3698b8 — __ZN3RBX10PhysicsJobD1Ev
pub fn stub_0x3698b8() -> ! {
    todo!("0x3698b8 __ZN3RBX10PhysicsJobD1Ev")
}

#[doc(alias = "RBX::PhysicsJob::~PhysicsJob()")]
// 0x369988 — __ZN3RBX10PhysicsJobD0Ev
pub fn stub_0x369988() -> ! {
    todo!("0x369988 __ZN3RBX10PhysicsJobD0Ev")
}

#[doc(alias = "RBX::PhysicsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x369a70 — __ZN3RBX10PhysicsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_0x369a70() -> ! {
    todo!("0x369a70 __ZN3RBX10PhysicsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::PhysicsJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x369ab0 — __ZN3RBX10PhysicsJob5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_0x369ab0() -> ! {
    todo!("0x369ab0 __ZN3RBX10PhysicsJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::PhysicsJob::getDesiredConcurrencyCount(void)const")]
// 0x369ac8 — __ZNK3RBX10PhysicsJob26getDesiredConcurrencyCountEv
pub fn stub_0x369ac8() -> ! {
    todo!("0x369ac8 __ZNK3RBX10PhysicsJob26getDesiredConcurrencyCountEv")
}

#[doc(alias = "RBX::findLocalFile(std::string const&,std::string *)")]
// 0x36a710 — __ZN3RBXL13findLocalFileERKSsPSs
pub fn stub_0x36a710() -> ! {
    todo!("0x36a710 __ZN3RBXL13findLocalFileERKSsPSs")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
// 0x36b644 — __ZN3RBX20SizeEnforcedLRUCacheISsSsE6insertERKSsS3_m
pub fn stub_0x36b644() -> ! {
    todo!("0x36b644 __ZN3RBX20SizeEnforcedLRUCacheISsSsE6insertERKSsS3_m")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
// 0x36de5c — __ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m
pub fn stub_0x36de5c() -> ! {
    todo!("0x36de5c __ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::removeLeastRecentlyUsed(void)")]
// 0x36e3e4 — __ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv
pub fn stub_0x36e3e4() -> ! {
    todo!("0x36e3e4 __ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::remove(std::string const&)")]
// 0x36e43c — __ZN3RBX8LRUCacheISsSsE6removeERKSs
pub fn stub_0x36e43c() -> ! {
    todo!("0x36e43c __ZN3RBX8LRUCacheISsSsE6removeERKSs")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>::destroy(std::pair<std::string,std::pair<unsigned long,std::string>>*)")]
// 0x36e558 — __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_
pub fn stub_0x36e558() -> ! {
    todo!("0x36e558 __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_")
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,std::string>>::pair(std::string const&,std::pair<unsigned long,std::string> const&)")]
// 0x36eb80 — __ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_
pub fn stub_0x36eb80() -> ! {
    todo!("0x36eb80 __ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_")
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,std::string>> const&)")]
// 0x36ec4c — __ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_0x36ec4c() -> ! {
    todo!("0x36ec4c __ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::~LRUCache()")]
// 0x3705a0 — __ZN3RBX8LRUCacheISsSsED2Ev
pub fn stub_0x3705a0() -> ! {
    todo!("0x3705a0 __ZN3RBX8LRUCacheISsSsED2Ev")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::resize(unsigned long)")]
// 0x3706b4 — __ZN3RBX8LRUCacheISsSsE6resizeEm
pub fn stub_0x3706b4() -> ! {
    todo!("0x3706b4 __ZN3RBX8LRUCacheISsSsE6resizeEm")
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_clear(void)")]
// 0x3706ec — __ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv
pub fn stub_0x3706ec() -> ! {
    todo!("0x3706ec __ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::LRUCache(void)")]
// 0x370780 — __ZN3RBX8LRUCacheISsSsEC2Ev
pub fn stub_0x370780() -> ! {
    todo!("0x370780 __ZN3RBX8LRUCacheISsSsEC2Ev")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::resize(unsigned long)")]
// 0x370860 — __ZN3RBX20SizeEnforcedLRUCacheISsSsE6resizeEm
pub fn stub_0x370860() -> ! {
    todo!("0x370860 __ZN3RBX20SizeEnforcedLRUCacheISsSsE6resizeEm")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int>(char const*,int const&)")]
// 0x37da40 — __ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_
pub fn stub_0x37da40() -> ! {
    todo!("0x37da40 __ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_")
}

#[doc(alias = "RBX::SpanningEdge::getConstChildSpanningNode(void)const")]
// 0x380464 — __ZNK3RBX12SpanningEdge25getConstChildSpanningNodeEv
pub fn stub_0x380464() -> ! {
    todo!("0x380464 __ZNK3RBX12SpanningEdge25getConstChildSpanningNodeEv")
}

#[doc(alias = "RBX::SpanningEdge::getChildSpanningNode(void)")]
// 0x3804e0 — __ZN3RBX12SpanningEdge20getChildSpanningNodeEv
pub fn stub_0x3804e0() -> ! {
    todo!("0x3804e0 __ZN3RBX12SpanningEdge20getChildSpanningNodeEv")
}

#[doc(alias = "RBX::SpanningEdge::getParentSpanningNode(void)")]
// 0x3804e4 — __ZN3RBX12SpanningEdge21getParentSpanningNodeEv
pub fn stub_0x3804e4() -> ! {
    todo!("0x3804e4 __ZN3RBX12SpanningEdge21getParentSpanningNodeEv")
}

#[doc(alias = "RBX::SpanningEdge::removeFromSpanningTree(void)")]
// 0x3804fc — __ZN3RBX12SpanningEdge22removeFromSpanningTreeEv
pub fn stub_0x3804fc() -> ! {
    todo!("0x3804fc __ZN3RBX12SpanningEdge22removeFromSpanningTreeEv")
}

#[doc(alias = "RBX::SpanningEdge::addToSpanningTree(RBX::SpanningNode *)")]
// 0x380568 — __ZN3RBX12SpanningEdge17addToSpanningTreeEPNS_12SpanningNodeE
pub fn stub_0x380568() -> ! {
    todo!("0x380568 __ZN3RBX12SpanningEdge17addToSpanningTreeEPNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningEdge::inSpanningTree(void)const")]
// 0x3806bc — __ZNK3RBX12SpanningEdge14inSpanningTreeEv
pub fn stub_0x3806bc() -> ! {
    todo!("0x3806bc __ZNK3RBX12SpanningEdge14inSpanningTreeEv")
}

#[doc(alias = "RBX::SpanningNode::setEdgeToParent(RBX::SpanningEdge *)")]
// 0x3807ac — __ZN3RBX12SpanningNode15setEdgeToParentEPNS_12SpanningEdgeE
pub fn stub_0x3807ac() -> ! {
    todo!("0x3807ac __ZN3RBX12SpanningNode15setEdgeToParentEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningTree::SpanningTree(void)")]
// 0x380878 — __ZN3RBX12SpanningTreeC2Ev
pub fn stub_0x380878() -> ! {
    todo!("0x380878 __ZN3RBX12SpanningTreeC2Ev")
}

#[doc(alias = "RBX::SpanningTree::~SpanningTree()")]
// 0x38089c — __ZN3RBX12SpanningTreeD2Ev
pub fn stub_0x38089c() -> ! {
    todo!("0x38089c __ZN3RBX12SpanningTreeD2Ev")
}

#[doc(alias = "RBX::SpanningTree::insertSpanningTreeEdge(RBX::SpanningEdge *)")]
// 0x3809c4 — __ZN3RBX12SpanningTree22insertSpanningTreeEdgeEPNS_12SpanningEdgeE
pub fn stub_0x3809c4() -> ! {
    todo!("0x3809c4 __ZN3RBX12SpanningTree22insertSpanningTreeEdgeEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningEdge *,RBX::SpanningEdge *&,int &)")]
// 0x380a6c — __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningEdgeERS2_Ri
pub fn stub_0x380a6c() -> ! {
    todo!("0x380a6c __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningEdgeERS2_Ri")
}

#[doc(alias = "RBX::SpanningTree::swapTree(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x380abc — __ZN3RBX12SpanningTree8swapTreeEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
pub fn stub_0x380abc() -> ! {
    todo!("0x380abc __ZN3RBX12SpanningTree8swapTreeEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningTree::removeSpanningTreeEdge(RBX::SpanningEdge *)")]
// 0x380b30 — __ZN3RBX12SpanningTree22removeSpanningTreeEdgeEPNS_12SpanningEdgeE
pub fn stub_0x380b30() -> ! {
    todo!("0x380b30 __ZN3RBX12SpanningTree22removeSpanningTreeEdgeEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningTree::findHeaviestDownstream(RBX::SpanningNode *,RBX::SpanningNode *&)")]
// 0x380bac — __ZN3RBX12SpanningTree22findHeaviestDownstreamEPNS_12SpanningNodeERS2_
pub fn stub_0x380bac() -> ! {
    todo!("0x380bac __ZN3RBX12SpanningTree22findHeaviestDownstreamEPNS_12SpanningNodeERS2_")
}

#[doc(alias = "RBX::SpanningTree::swap(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x380cdc — __ZN3RBX12SpanningTree4swapEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE
pub fn stub_0x380cdc() -> ! {
    todo!("0x380cdc __ZN3RBX12SpanningTree4swapEPNS_12SpanningEdgeES2_PNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningTree::removeEdge(RBX::SpanningEdge *)")]
// 0x380d50 — __ZN3RBX12SpanningTree10removeEdgeEPNS_12SpanningEdgeE
pub fn stub_0x380d50() -> ! {
    todo!("0x380d50 __ZN3RBX12SpanningTree10removeEdgeEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningTree::addEdge(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x380e34 — __ZN3RBX12SpanningTree7addEdgeEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_0x380e34() -> ! {
    todo!("0x380e34 __ZN3RBX12SpanningTree7addEdgeEPNS_12SpanningEdgeEPNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningNode *,RBX::SpanningNode *,int,int,RBX::SpanningEdge *&,int &)")]
// 0x381120 — __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningNodeES2_iiRPNS_12SpanningEdgeERi
pub fn stub_0x381120() -> ! {
    todo!("0x381120 __ZN3RBX12SpanningTree20findLightestUpstreamEPNS_12SpanningNodeES2_iiRPNS_12SpanningEdgeERi")
}

#[doc(alias = "RBX::SpanningTree::buildDownstreamTree(RBX::SpanningNode *,std::set<RBX::SpanningNode *,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>> &)")]
// 0x38120c — __ZN3RBX12SpanningTree19buildDownstreamTreeEPNS_12SpanningNodeERSt3setIS2_St4lessIS2_ESaIS2_EE
pub fn stub_0x38120c() -> ! {
    todo!("0x38120c __ZN3RBX12SpanningTree19buildDownstreamTreeEPNS_12SpanningNodeERSt3setIS2_St4lessIS2_ESaIS2_EE")
}

#[doc(alias = "RBX::SpanningNode::getDepth(RBX::SpanningNode*)")]
// 0x381308 — __ZN3RBX12SpanningNode8getDepthEPS0_
pub fn stub_0x381308() -> ! {
    todo!("0x381308 __ZN3RBX12SpanningNode8getDepthEPS0_")
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x381328 — __ZN3RBX12SpanningTree20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_0x381328() -> ! {
    todo!("0x381328 __ZN3RBX12SpanningTree20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE")
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
// 0x38132c — __ZN3RBX12SpanningTree19onSpanningEdgeAddedEPNS_12SpanningEdgeE
pub fn stub_0x38132c() -> ! {
    todo!("0x38132c __ZN3RBX12SpanningTree19onSpanningEdgeAddedEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
// 0x381330 — __ZN3RBX12SpanningTree22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
pub fn stub_0x381330() -> ! {
    todo!("0x381330 __ZN3RBX12SpanningTree22onSpanningEdgeRemovingEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::UserInputBase::getGameCursor(RBX::Adorn *)")]
// 0x38c6b4 — __ZN3RBX13UserInputBase13getGameCursorEPNS_5AdornE
pub fn stub_0x38c6b4() -> ! {
    todo!("0x38c6b4 __ZN3RBX13UserInputBase13getGameCursorEPNS_5AdornE")
}

#[doc(alias = "RBX::UserInputBase::setCursorId(RBX::Adorn *,RBX::TextureId const&)")]
// 0x38c928 — __ZN3RBX13UserInputBase11setCursorIdEPNS_5AdornERKNS_9TextureIdE
pub fn stub_0x38c928() -> ! {
    todo!("0x38c928 __ZN3RBX13UserInputBase11setCursorIdEPNS_5AdornERKNS_9TextureIdE")
}

#[doc(alias = "RBX::UserInputBase::renderGameCursor(RBX::Adorn *)")]
// 0x38c974 — __ZN3RBX13UserInputBase16renderGameCursorEPNS_5AdornE
pub fn stub_0x38c974() -> ! {
    todo!("0x38c974 __ZN3RBX13UserInputBase16renderGameCursorEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// 0x38e9d0 — __ZThn36_N3RBX12AccoutrementD0Ev
// was: `non-virtual thunk to'RBX::Accoutrement::~Accoutrement()
pub fn stub_0x38e9d0() -> ! {
    todo!("0x38e9d0 __ZThn36_N3RBX12AccoutrementD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// 0x38e9d8 — __ZThn92_N3RBX12AccoutrementD0Ev
// was: `non-virtual thunk to'RBX::Accoutrement::~Accoutrement()
pub fn stub_0x38e9d8() -> ! {
    todo!("0x38e9d8 __ZThn92_N3RBX12AccoutrementD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// 0x38e9e0 — __ZThn128_N3RBX12AccoutrementD0Ev
// was: `non-virtual thunk to'RBX::Accoutrement::~Accoutrement()
pub fn stub_0x38e9e0() -> ! {
    todo!("0x38e9e0 __ZThn128_N3RBX12AccoutrementD0Ev")
}

#[doc(alias = "RBX::Accoutrement::~Accoutrement()")]
// 0x38e9e8 — __ZN3RBX12AccoutrementD2Ev
pub fn stub_0x38e9e8() -> ! {
    todo!("0x38e9e8 __ZN3RBX12AccoutrementD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// 0x38ef1c — __ZThn32_N3RBX12AccoutrementD1Ev
// was: `non-virtual thunk to'RBX::Accoutrement::~Accoutrement()
pub fn stub_0x38ef1c() -> ! {
    todo!("0x38ef1c __ZThn32_N3RBX12AccoutrementD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// 0x38ef2c — __ZThn36_N3RBX12AccoutrementD1Ev
// was: `non-virtual thunk to'RBX::Accoutrement::~Accoutrement()
pub fn stub_0x38ef2c() -> ! {
    todo!("0x38ef2c __ZThn36_N3RBX12AccoutrementD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// 0x38ef3c — __ZThn92_N3RBX12AccoutrementD1Ev
// was: `non-virtual thunk to'RBX::Accoutrement::~Accoutrement()
pub fn stub_0x38ef3c() -> ! {
    todo!("0x38ef3c __ZThn92_N3RBX12AccoutrementD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// 0x38ef4c — __ZThn128_N3RBX12AccoutrementD1Ev
// was: `non-virtual thunk to'RBX::Accoutrement::~Accoutrement()
pub fn stub_0x38ef4c() -> ! {
    todo!("0x38ef4c __ZThn128_N3RBX12AccoutrementD1Ev")
}

#[doc(alias = "RBX::Accoutrement::onCameraNear(float)")]
// 0x38ef5c — __ZN3RBX12Accoutrement12onCameraNearEf
pub fn stub_0x38ef5c() -> ! {
    todo!("0x38ef5c __ZN3RBX12Accoutrement12onCameraNearEf")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::onCameraNear(float)")]
// 0x38ef98 — __ZThn128_N3RBX12Accoutrement12onCameraNearEf
// was: `non-virtual thunk to'RBX::Accoutrement::onCameraNear(float)
pub fn stub_0x38ef98() -> ! {
    todo!("0x38ef98 __ZThn128_N3RBX12Accoutrement12onCameraNearEf")
}

#[doc(alias = "RBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
// 0x38efa0 — __ZN3RBX12Accoutrement14render3dSelectEPNS_5AdornENS_11SelectStateE
pub fn stub_0x38efa0() -> ! {
    todo!("0x38efa0 __ZN3RBX12Accoutrement14render3dSelectEPNS_5AdornENS_11SelectStateE")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
// 0x38f014 — __ZThn104_N3RBX12Accoutrement14render3dSelectEPNS_5AdornENS_11SelectStateE
// was: `non-virtual thunk to'RBX::Accoutrement::render3dSelect(RBX::Adorn *,RBX::SelectState)
pub fn stub_0x38f014() -> ! {
    todo!("0x38f014 __ZThn104_N3RBX12Accoutrement14render3dSelectEPNS_5AdornENS_11SelectStateE")
}

#[doc(alias = "RBX::Accoutrement::getHandleConst(void)const")]
// 0x38f054 — __ZNK3RBX12Accoutrement14getHandleConstEv
pub fn stub_0x38f054() -> ! {
    todo!("0x38f054 __ZNK3RBX12Accoutrement14getHandleConstEv")
}

#[doc(alias = "RBX::Accoutrement::getLocation(void)")]
// 0x38f1c4 — __ZN3RBX12Accoutrement11getLocationEv
pub fn stub_0x38f1c4() -> ! {
    todo!("0x38f1c4 __ZN3RBX12Accoutrement11getLocationEv")
}

#[doc(alias = "virtual thunk toRBX::Accoutrement::getLocation(void)")]
// 0x38f1f8 — __ZTv0_n12_N3RBX12Accoutrement11getLocationEv
// was: `virtual thunk to'RBX::Accoutrement::getLocation(void)
pub fn stub_0x38f1f8() -> ! {
    todo!("0x38f1f8 __ZTv0_n12_N3RBX12Accoutrement11getLocationEv")
}

#[doc(alias = "RBX::Accoutrement::connectTouchEvent(void)")]
// 0x38f20c — __ZN3RBX12Accoutrement17connectTouchEventEv
pub fn stub_0x38f20c() -> ! {
    todo!("0x38f20c __ZN3RBX12Accoutrement17connectTouchEventEv")
}

#[doc(alias = "RBX::Accoutrement::rebuildBackendState(void)")]
// 0x38f47c — __ZN3RBX12Accoutrement19rebuildBackendStateEv
pub fn stub_0x38f47c() -> ! {
    todo!("0x38f47c __ZN3RBX12Accoutrement19rebuildBackendStateEv")
}

#[doc(alias = "RBX::Accoutrement::computeDesiredState(void)")]
// 0x38f4f4 — __ZN3RBX12Accoutrement19computeDesiredStateEv
pub fn stub_0x38f4f4() -> ! {
    todo!("0x38f4f4 __ZN3RBX12Accoutrement19computeDesiredStateEv")
}

#[doc(alias = "RBX::Accoutrement::setDesiredState(RBX::Accoutrement::AccoutrementState,RBX::ServiceProvider const*)")]
// 0x38f578 — __ZN3RBX12Accoutrement15setDesiredStateENS0_17AccoutrementStateEPKNS_15ServiceProviderE
pub fn stub_0x38f578() -> ! {
    todo!("0x38f578 __ZN3RBX12Accoutrement15setDesiredStateENS0_17AccoutrementStateEPKNS_15ServiceProviderE")
}

#[doc(alias = "RBX::Accoutrement::upTo_Equipped(void)")]
// 0x38f714 — __ZN3RBX12Accoutrement13upTo_EquippedEv
pub fn stub_0x38f714() -> ! {
    todo!("0x38f714 __ZN3RBX12Accoutrement13upTo_EquippedEv")
}

#[doc(alias = "RBX::Accoutrement::upTo_InCharacter(void)")]
// 0x38f92c — __ZN3RBX12Accoutrement16upTo_InCharacterEv
pub fn stub_0x38f92c() -> ! {
    todo!("0x38f92c __ZN3RBX12Accoutrement16upTo_InCharacterEv")
}

#[doc(alias = "RBX::Accoutrement::downFrom_Equipped(void)")]
// 0x38fbcc — __ZN3RBX12Accoutrement17downFrom_EquippedEv
pub fn stub_0x38fbcc() -> ! {
    todo!("0x38fbcc __ZN3RBX12Accoutrement17downFrom_EquippedEv")
}

#[doc(alias = "RBX::Accoutrement::downFrom_HasHandle(void)")]
// 0x38fd24 — __ZN3RBX12Accoutrement18downFrom_HasHandleEv
pub fn stub_0x38fd24() -> ! {
    todo!("0x38fd24 __ZN3RBX12Accoutrement18downFrom_HasHandleEv")
}

#[doc(alias = "RBX::Accoutrement::onAncestorChanged(RBX::AncestorChanged const&)")]
// 0x38ff84 — __ZN3RBX12Accoutrement17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_0x38ff84() -> ! {
    todo!("0x38ff84 __ZN3RBX12Accoutrement17onAncestorChangedERKNS_15AncestorChangedE")
}

#[doc(alias = "RBX::Hat::Hat(void)")]
// 0x38fff0 — __ZN3RBX3HatC1Ev
pub fn stub_0x38fff0() -> ! {
    todo!("0x38fff0 __ZN3RBX3HatC1Ev")
}

#[doc(alias = "RBX::Accoutrement::getAttachmentPoint(void)const")]
// 0x3901bc — __ZNK3RBX12Accoutrement18getAttachmentPointEv
pub fn stub_0x3901bc() -> ! {
    todo!("0x3901bc __ZNK3RBX12Accoutrement18getAttachmentPointEv")
}

#[doc(alias = "RBX::Accoutrement::getBackendAccoutrementState(void)const")]
// 0x390208 — __ZNK3RBX12Accoutrement27getBackendAccoutrementStateEv
pub fn stub_0x390208() -> ! {
    todo!("0x390208 __ZNK3RBX12Accoutrement27getBackendAccoutrementStateEv")
}

#[doc(alias = "RBX::Accoutrement::getRenderLocation(void)")]
// 0x39066c — __ZN3RBX12Accoutrement17getRenderLocationEv
pub fn stub_0x39066c() -> ! {
    todo!("0x39066c __ZN3RBX12Accoutrement17getRenderLocationEv")
}

#[doc(alias = "RBX::Accoutrement::getRenderSize(void)")]
// 0x39067c — __ZN3RBX12Accoutrement13getRenderSizeEv
pub fn stub_0x39067c() -> ! {
    todo!("0x39067c __ZN3RBX12Accoutrement13getRenderSizeEv")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::getRenderLocation(void)")]
// 0x3906b4 — __ZThn128_N3RBX12Accoutrement17getRenderLocationEv
// was: `non-virtual thunk to'RBX::Accoutrement::getRenderLocation(void)
pub fn stub_0x3906b4() -> ! {
    todo!("0x3906b4 __ZThn128_N3RBX12Accoutrement17getRenderLocationEv")
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::getRenderSize(void)")]
// 0x3906c4 — __ZThn128_N3RBX12Accoutrement13getRenderSizeEv
// was: `non-virtual thunk to'RBX::Accoutrement::getRenderSize(void)
pub fn stub_0x3906c4() -> ! {
    todo!("0x3906c4 __ZThn128_N3RBX12Accoutrement13getRenderSizeEv")
}

#[doc(alias = "RBX::Hat::~Hat()")]
// 0x3906d8 — __ZN3RBX3HatD1Ev
pub fn stub_0x3906d8() -> ! {
    todo!("0x3906d8 __ZN3RBX3HatD1Ev")
}

#[doc(alias = "RBX::Hat::~Hat()")]
// 0x3906ec — __ZN3RBX3HatD0Ev
pub fn stub_0x3906ec() -> ! {
    todo!("0x3906ec __ZN3RBX3HatD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
// 0x3907ac — __ZThn32_N3RBX3HatD1Ev
// was: `non-virtual thunk to'RBX::Hat::~Hat()
pub fn stub_0x3907ac() -> ! {
    todo!("0x3907ac __ZThn32_N3RBX3HatD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
// 0x3907c0 — __ZThn32_N3RBX3HatD0Ev
// was: `non-virtual thunk to'RBX::Hat::~Hat()
pub fn stub_0x3907c0() -> ! {
    todo!("0x3907c0 __ZThn32_N3RBX3HatD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Hat::~Hat()")]
// 0x390884 — __ZThn36_N3RBX3HatD1Ev
// was: `non-virtual thunk to'RBX::Hat::~Hat()
pub fn stub_0x390884() -> ! {
    todo!("0x390884 __ZThn36_N3RBX3HatD1Ev")
}