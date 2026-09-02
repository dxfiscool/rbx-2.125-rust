//! rendering shard watchdog rendB — 120 stubs 0x7c6d48..0x899020 EA-sorted asc, globally deduped via /tmp/global_eas.txt (Ogre|Gfx|Render|G3D|Adorn exhausted -> gap filler)
//! Source: ida/export.json (85545 funcs) gap filler next 120 uncovered EA asc after global+crate dedup
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7c6d48 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotD1Ev
// type: 
#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotD1Ev")]
pub fn stub_7c6d48() -> ! {
    todo!("0x7c6d48 rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot::~slot()")
}

// 0x7c6d74 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotD0Ev
// type: 
#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotD0Ev")]
pub fn stub_7c6d74() -> ! {
    todo!("0x7c6d74 rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot::~slot()")
}

// 0x7cb6a0 — __ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE9push_backERKS2_
// type: 
#[doc(alias = "std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>::push_back(RBX::Primitive * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE9push_backERKS2_")]
pub fn stub_7cb6a0() -> ! {
    todo!("0x7cb6a0 std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>::push_back(RBX::Primitive * const&)")
}

// 0x7cb6cc — __ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>,RBX::Primitive * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_7cb6cc() -> ! {
    todo!("0x7cb6cc std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>,RBX::Primitive * const&)")
}

// 0x7cd220 — __ZN3RBX5HUMAN8getStateENS0_9StateTypeENS0_9EventTypeE
// type: 
#[doc(alias = "RBX::HUMAN::getState(RBX::HUMAN::StateType,RBX::HUMAN::EventType)")]
#[doc(alias = "__ZN3RBX5HUMAN8getStateENS0_9StateTypeENS0_9EventTypeE")]
pub fn stub_7cd220() -> ! {
    todo!("0x7cd220 RBX::HUMAN::getState(RBX::HUMAN::StateType,RBX::HUMAN::EventType)")
}

// 0x7cd2f4 — __ZN3RBX5HUMAN13HumanoidStateC2EPNS_8HumanoidENS0_9StateTypeE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::HUMAN::HumanoidState::HumanoidState(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidStateC2EPNS_8HumanoidENS0_9StateTypeE")]
pub fn stub_7cd2f4() -> ! {
    todo!("0x7cd2f4 RBX::HUMAN::HumanoidState::HumanoidState(RBX::Humanoid *,RBX::HUMAN::StateType)")
}

// 0x7cd5fc — __ZN3RBX5HUMAN13HumanoidState19setCanThrottleStateEb
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, bool)
#[doc(alias = "RBX::HUMAN::HumanoidState::setCanThrottleState(bool)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState19setCanThrottleStateEb")]
pub fn stub_7cd5fc() -> ! {
    todo!("0x7cd5fc RBX::HUMAN::HumanoidState::setCanThrottleState(bool)")
}

// 0x7cd6a4 — __ZN3RBX5HUMAN13HumanoidStateD0Ev
// type: void __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::~HumanoidState()")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidStateD0Ev")]
pub fn stub_7cd6a4() -> ! {
    todo!("0x7cd6a4 RBX::HUMAN::HumanoidState::~HumanoidState()")
}

// 0x88e2c8 — __ZN3RBX11PluginMouseC1Ev
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::PluginMouse(void)")]
#[doc(alias = "__ZN3RBX11PluginMouseC1Ev")]
pub fn stub_88e2c8() -> ! {
    todo!("0x88e2c8 RBX::PluginMouse::PluginMouse(void)")
}

// 0x88e2cc — __ZN3RBX11PluginMouseC2Ev
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::PluginMouse(void)")]
#[doc(alias = "__ZN3RBX11PluginMouseC2Ev")]
pub fn stub_88e2cc() -> ! {
    todo!("0x88e2cc RBX::PluginMouse::PluginMouse(void)")
}

// 0x88e468 — __ZN3RBX11PluginMouseD0Ev
// type: void __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZN3RBX11PluginMouseD0Ev")]
pub fn stub_88e468() -> ! {
    todo!("0x88e468 RBX::PluginMouse::~PluginMouse()")
}

// 0x88e508 — __ZN3RBX11PluginMouseD1Ev
// type: void __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZN3RBX11PluginMouseD1Ev")]
pub fn stub_88e508() -> ! {
    todo!("0x88e508 RBX::PluginMouse::~PluginMouse()")
}

// 0x88e50c — __ZThn32_N3RBX11PluginMouseD0Ev
// type: void __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZThn32_N3RBX11PluginMouseD0Ev")]
pub fn stub_88e50c() -> ! {
    todo!("0x88e50c `non-virtual thunk toRBX::PluginMouse::~PluginMouse()")
}

// 0x88e514 — __ZThn36_N3RBX11PluginMouseD0Ev
// type: void __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZThn36_N3RBX11PluginMouseD0Ev")]
pub fn stub_88e514() -> ! {
    todo!("0x88e514 `non-virtual thunk toRBX::PluginMouse::~PluginMouse()")
}

// 0x88e51c — __ZN3RBX11PluginMouseD2Ev
// type: void __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZN3RBX11PluginMouseD2Ev")]
pub fn stub_88e51c() -> ! {
    todo!("0x88e51c RBX::PluginMouse::~PluginMouse()")
}

// 0x88e614 — __ZThn32_N3RBX11PluginMouseD1Ev
// type: void __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZThn32_N3RBX11PluginMouseD1Ev")]
pub fn stub_88e614() -> ! {
    todo!("0x88e614 `non-virtual thunk toRBX::PluginMouse::~PluginMouse()")
}

// 0x88e61c — __ZThn36_N3RBX11PluginMouseD1Ev
// type: void __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::PluginMouse::~PluginMouse()")]
#[doc(alias = "__ZThn36_N3RBX11PluginMouseD1Ev")]
pub fn stub_88e61c() -> ! {
    todo!("0x88e61c `non-virtual thunk toRBX::PluginMouse::~PluginMouse()")
}

// 0x88e624 — __ZNK3RBX11PluginMouse6getHitEv
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::getHit(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse6getHitEv")]
pub fn stub_88e624() -> ! {
    todo!("0x88e624 RBX::PluginMouse::getHit(void)const")
}

// 0x88e8ec — __ZNK3RBX11PluginMouse9getOriginEv
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::getOrigin(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse9getOriginEv")]
pub fn stub_88e8ec() -> ! {
    todo!("0x88e8ec RBX::PluginMouse::getOrigin(void)const")
}

// 0x88e9f4 — __ZNK3RBX11PluginMouse10getUnitRayEv
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::getUnitRay(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse10getUnitRayEv")]
pub fn stub_88e9f4() -> ! {
    todo!("0x88e9f4 RBX::PluginMouse::getUnitRay(void)const")
}

// 0x88eae0 — __ZNK3RBX11PluginMouse9getTargetEv
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::getTarget(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse9getTargetEv")]
pub fn stub_88eae0() -> ! {
    todo!("0x88eae0 RBX::PluginMouse::getTarget(void)const")
}

// 0x88ee2c — __ZNK3RBX11PluginMouse16getTargetSurfaceEv
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::getTargetSurface(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse16getTargetSurfaceEv")]
pub fn stub_88ee2c() -> ! {
    todo!("0x88ee2c RBX::PluginMouse::getTargetSurface(void)const")
}

// 0x88ef90 — __ZN3RBX11PluginMouse6updateERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::PluginMouse::update(RBX::UIEvent const&)")]
#[doc(alias = "__ZN3RBX11PluginMouse6updateERKNS_7UIEventE")]
pub fn stub_88ef90() -> ! {
    todo!("0x88ef90 RBX::PluginMouse::update(RBX::UIEvent const&)")
}

// 0x88f2a8 — __ZNK3RBX11PluginMouse4getXEv
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::getX(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse4getXEv")]
pub fn stub_88f2a8() -> ! {
    todo!("0x88f2a8 RBX::PluginMouse::getX(void)const")
}

// 0x88f2b0 — __ZNK3RBX11PluginMouse4getYEv
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::getY(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse4getYEv")]
pub fn stub_88f2b0() -> ! {
    todo!("0x88f2b0 RBX::PluginMouse::getY(void)const")
}

// 0x88f4e0 — __ZNK3RBX11PluginMouse11checkActiveEv
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::checkActive(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse11checkActiveEv")]
pub fn stub_88f4e0() -> ! {
    todo!("0x88f4e0 RBX::PluginMouse::checkActive(void)const")
}

// 0x88fdc0 — __ZN3RBX15BallCellContactD0Ev
// type: void __fastcall(RBX::BallCellContact *__hidden this)
#[doc(alias = "RBX::BallCellContact::~BallCellContact()")]
#[doc(alias = "__ZN3RBX15BallCellContactD0Ev")]
pub fn stub_88fdc0() -> ! {
    todo!("0x88fdc0 RBX::BallCellContact::~BallCellContact()")
}

// 0x88fe74 — __ZN3RBX15BallCellContactD1Ev
// type: void __fastcall(RBX::BallCellContact *__hidden this)
#[doc(alias = "RBX::BallCellContact::~BallCellContact()")]
#[doc(alias = "__ZN3RBX15BallCellContactD1Ev")]
pub fn stub_88fe74() -> ! {
    todo!("0x88fe74 RBX::BallCellContact::~BallCellContact()")
}

// 0x88fe78 — __ZN3RBX15BallCellContactD2Ev
// type: void __fastcall(RBX::BallCellContact *__hidden this)
#[doc(alias = "RBX::BallCellContact::~BallCellContact()")]
#[doc(alias = "__ZN3RBX15BallCellContactD2Ev")]
pub fn stub_88fe78() -> ! {
    todo!("0x88fe78 RBX::BallCellContact::~BallCellContact()")
}

// 0x88ff94 — __ZN3RBX15BallCellContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
// type: 
#[doc(alias = "RBX::BallCellContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX15BallCellContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
pub fn stub_88ff94() -> ! {
    todo!("0x88ff94 RBX::BallCellContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")
}

// 0x890268 — __ZN3RBX15BallCellContact21newBallPlaneConnectorEPKNS_4POLY4FaceE
// type: _DWORD __fastcall(RBX::BallCellContact *__hidden this, const RBX::POLY::Face *)
#[doc(alias = "RBX::BallCellContact::newBallPlaneConnector(RBX::POLY::Face const*)")]
#[doc(alias = "__ZN3RBX15BallCellContact21newBallPlaneConnectorEPKNS_4POLY4FaceE")]
pub fn stub_890268() -> ! {
    todo!("0x890268 RBX::BallCellContact::newBallPlaneConnector(RBX::POLY::Face const*)")
}

// 0x890514 — __ZN3RBX15BallCellContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE
// type: _DWORD __fastcall(RBX::BallCellContact *__hidden this, const RBX::POLY::Edge *)
#[doc(alias = "RBX::BallCellContact::newBallEdgeConnector(RBX::POLY::Edge const*)")]
#[doc(alias = "__ZN3RBX15BallCellContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE")]
pub fn stub_890514() -> ! {
    todo!("0x890514 RBX::BallCellContact::newBallEdgeConnector(RBX::POLY::Edge const*)")
}

// 0x8908e8 — __ZN3RBX15BallCellContact22newBallVertexConnectorEPKNS_4POLY6VertexE
// type: int __fastcall(int, int, int, int, boost::mutex *, int, int, int, int, int)
#[doc(alias = "RBX::BallCellContact::newBallVertexConnector(RBX::POLY::Vertex const*)")]
#[doc(alias = "__ZN3RBX15BallCellContact22newBallVertexConnectorEPKNS_4POLY6VertexE")]
pub fn stub_8908e8() -> ! {
    todo!("0x8908e8 RBX::BallCellContact::newBallVertexConnector(RBX::POLY::Vertex const*)")
}

// 0x890ad4 — __ZN3RBX15BallCellContact34generateDataForMovingAssemblyStageEv
// type: _DWORD __fastcall(RBX::BallCellContact *__hidden this)
#[doc(alias = "RBX::BallCellContact::generateDataForMovingAssemblyStage(void)")]
#[doc(alias = "__ZN3RBX15BallCellContact34generateDataForMovingAssemblyStageEv")]
pub fn stub_890ad4() -> ! {
    todo!("0x890ad4 RBX::BallCellContact::generateDataForMovingAssemblyStage(void)")
}

// 0x890ad8 — __ZN3RBX9AllocatorINS_15BallCellContactEEC2Ev
// type: 
#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallCellContactEEC2Ev")]
pub fn stub_890ad8() -> ! {
    todo!("0x890ad8 RBX::Allocator<RBX::BallCellContact>::Allocator(void)")
}

// 0x890b3c — __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE9push_backERKS2_
// type: 
#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::push_back(RBX::PolyConnector * const&)")]
#[doc(alias = "__ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE9push_backERKS2_")]
pub fn stub_890b3c() -> ! {
    todo!("0x890b3c RBX::FixedArray<RBX::PolyConnector *,40ul>::push_back(RBX::PolyConnector * const&)")
}

// 0x890c24 — __ZNK3RBX4POLY4Face5planeEv
// type: _DWORD __fastcall(RBX::POLY::Face *__hidden this)
#[doc(alias = "RBX::POLY::Face::plane(void)const")]
#[doc(alias = "__ZNK3RBX4POLY4Face5planeEv")]
pub fn stub_890c24() -> ! {
    todo!("0x890c24 RBX::POLY::Face::plane(void)const")
}

// 0x890ce4 — __ZN3RBX9AllocatorINS_18BallPlaneConnectorEEnwEm
// type: 
#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_18BallPlaneConnectorEEnwEm")]
pub fn stub_890ce4() -> ! {
    todo!("0x890ce4 RBX::Allocator<RBX::BallPlaneConnector>::operator new(unsigned long)")
}

// 0x890d54 — __ZN3RBX9AllocatorINS_17BallEdgeConnectorEEnwEm
// type: 
#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17BallEdgeConnectorEEnwEm")]
pub fn stub_890d54() -> ! {
    todo!("0x890d54 RBX::Allocator<RBX::BallEdgeConnector>::operator new(unsigned long)")
}

// 0x890dc4 — __ZNK3RBX4POLY4Edge13computeNormalEPKNS0_4FaceE
// type: _DWORD __fastcall(RBX::POLY::Edge *__hidden this, const RBX::POLY::Face *)
#[doc(alias = "RBX::POLY::Edge::computeNormal(RBX::POLY::Face const*)const")]
#[doc(alias = "__ZNK3RBX4POLY4Edge13computeNormalEPKNS0_4FaceE")]
pub fn stub_890dc4() -> ! {
    todo!("0x890dc4 RBX::POLY::Edge::computeNormal(RBX::POLY::Face const*)const")
}

// 0x890e50 — __ZN3RBX9AllocatorINS_19BallVertexConnectorEEnwEm
// type: 
#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19BallVertexConnectorEEnwEm")]
pub fn stub_890e50() -> ! {
    todo!("0x890e50 RBX::Allocator<RBX::BallVertexConnector>::operator new(unsigned long)")
}

// 0x890ec0 — __ZNK3RBX11CellContact13numConnectorsEv
// type: _DWORD __fastcall(RBX::CellContact *__hidden this)
#[doc(alias = "RBX::CellContact::numConnectors(void)const")]
#[doc(alias = "__ZNK3RBX11CellContact13numConnectorsEv")]
pub fn stub_890ec0() -> ! {
    todo!("0x890ec0 RBX::CellContact::numConnectors(void)const")
}

// 0x890ec8 — __ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_890ec8() -> ! {
    todo!("0x890ec8 boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x890f18 — __ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_890f18() -> ! {
    todo!("0x890f18 boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x890f50 — __ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_890f50() -> ! {
    todo!("0x890f50 boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x890fa0 — __ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_890fa0() -> ! {
    todo!("0x890fa0 boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x890fd8 — __ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_890fd8() -> ! {
    todo!("0x890fd8 boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x891028 — __ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_891028() -> ! {
    todo!("0x891028 boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x891060 — __ZN3RBX9AllocatorINS_15BallCellContactEE13releaseMemoryEv
// type: 
#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallCellContactEE13releaseMemoryEv")]
pub fn stub_891060() -> ! {
    todo!("0x891060 RBX::Allocator<RBX::BallCellContact>::releaseMemory(void)")
}

// 0x89107c — __ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_89107c() -> ! {
    todo!("0x89107c boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x8910ac — __ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_8910ac() -> ! {
    todo!("0x8910ac boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x8910fc — __ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_8910fc() -> ! {
    todo!("0x8910fc boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x89114c — __ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_89114c() -> ! {
    todo!("0x89114c boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x89119c — __ZN3RBX9AllocatorINS_19BallVertexConnectorEEC2Ev
// type: 
#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19BallVertexConnectorEEC2Ev")]
pub fn stub_89119c() -> ! {
    todo!("0x89119c RBX::Allocator<RBX::BallVertexConnector>::Allocator(void)")
}

// 0x891200 — __ZNK3RBX19BallVertexConnector16getConnectorTypeEv
// type: _DWORD __fastcall(RBX::BallVertexConnector *__hidden this)
#[doc(alias = "RBX::BallVertexConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX19BallVertexConnector16getConnectorTypeEv")]
pub fn stub_891200() -> ! {
    todo!("0x891200 RBX::BallVertexConnector::getConnectorType(void)const")
}

// 0x891204 — __ZN3RBX9AllocatorINS_19BallVertexConnectorEE13releaseMemoryEv
// type: 
#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19BallVertexConnectorEE13releaseMemoryEv")]
pub fn stub_891204() -> ! {
    todo!("0x891204 RBX::Allocator<RBX::BallVertexConnector>::releaseMemory(void)")
}

// 0x891220 — __ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: 
#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_891220() -> ! {
    todo!("0x891220 boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x891250 — __ZN3RBX13PolyConnectorD1Ev
// type: void __fastcall(RBX::PolyConnector *__hidden this)
#[doc(alias = "RBX::PolyConnector::~PolyConnector()")]
#[doc(alias = "__ZN3RBX13PolyConnectorD1Ev")]
pub fn stub_891250() -> ! {
    todo!("0x891250 RBX::PolyConnector::~PolyConnector()")
}

// 0x891254 — __ZN3RBX13PolyConnectorD0Ev
// type: void __fastcall(RBX::PolyConnector *__hidden this)
#[doc(alias = "RBX::PolyConnector::~PolyConnector()")]
#[doc(alias = "__ZN3RBX13PolyConnectorD0Ev")]
pub fn stub_891254() -> ! {
    todo!("0x891254 RBX::PolyConnector::~PolyConnector()")
}

// 0x891258 — __ZN3RBX9AllocatorINS_17BallEdgeConnectorEEC2Ev
// type: 
#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17BallEdgeConnectorEEC2Ev")]
pub fn stub_891258() -> ! {
    todo!("0x891258 RBX::Allocator<RBX::BallEdgeConnector>::Allocator(void)")
}

// 0x8912bc — __ZNK3RBX17BallEdgeConnector16getConnectorTypeEv
// type: _DWORD __fastcall(RBX::BallEdgeConnector *__hidden this)
#[doc(alias = "RBX::BallEdgeConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX17BallEdgeConnector16getConnectorTypeEv")]
pub fn stub_8912bc() -> ! {
    todo!("0x8912bc RBX::BallEdgeConnector::getConnectorType(void)const")
}

// 0x8912c0 — __ZN3RBX9AllocatorINS_17BallEdgeConnectorEE13releaseMemoryEv
// type: 
#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17BallEdgeConnectorEE13releaseMemoryEv")]
pub fn stub_8912c0() -> ! {
    todo!("0x8912c0 RBX::Allocator<RBX::BallEdgeConnector>::releaseMemory(void)")
}

// 0x8912dc — __ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_8912dc() -> ! {
    todo!("0x8912dc boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x89130c — __ZN3RBX9AllocatorINS_18BallPlaneConnectorEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_18BallPlaneConnectorEEC2Ev")]
pub fn stub_89130c() -> ! {
    todo!("0x89130c RBX::Allocator<RBX::BallPlaneConnector>::Allocator(void)")
}

// 0x891370 — __ZNK3RBX18BallPlaneConnector16getConnectorTypeEv
// type: int __fastcall(RBX::BallPlaneConnector *this)
#[doc(alias = "RBX::BallPlaneConnector::getConnectorType(void)const")]
#[doc(alias = "__ZNK3RBX18BallPlaneConnector16getConnectorTypeEv")]
pub fn stub_891370() -> ! {
    todo!("0x891370 RBX::BallPlaneConnector::getConnectorType(void)const")
}

// 0x891374 — __ZN3RBX9AllocatorINS_18BallPlaneConnectorEE13releaseMemoryEv
// type: 
#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_18BallPlaneConnectorEE13releaseMemoryEv")]
pub fn stub_891374() -> ! {
    todo!("0x891374 RBX::Allocator<RBX::BallPlaneConnector>::releaseMemory(void)")
}

// 0x891390 — __ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_891390() -> ! {
    todo!("0x891390 boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x891ae8 — __ZN3RBX21PersonalServerService27setPersonalServerGetRankUrlESs
// type: int __fastcall(int)
#[doc(alias = "RBX::PersonalServerService::setPersonalServerGetRankUrl(std::string)")]
#[doc(alias = "__ZN3RBX21PersonalServerService27setPersonalServerGetRankUrlESs")]
pub fn stub_891ae8() -> ! {
    todo!("0x891ae8 RBX::PersonalServerService::setPersonalServerGetRankUrl(std::string)")
}

// 0x891af0 — __ZN3RBX21PersonalServerService27setPersonalServerSetRankUrlESs
// type: int __fastcall(int)
#[doc(alias = "RBX::PersonalServerService::setPersonalServerSetRankUrl(std::string)")]
#[doc(alias = "__ZN3RBX21PersonalServerService27setPersonalServerSetRankUrlESs")]
pub fn stub_891af0() -> ! {
    todo!("0x891af0 RBX::PersonalServerService::setPersonalServerSetRankUrl(std::string)")
}

// 0x891af8 — __ZN3RBX21PersonalServerService28setPersonalServerRoleSetsUrlESs
// type: int __fastcall(int)
#[doc(alias = "RBX::PersonalServerService::setPersonalServerRoleSetsUrl(std::string)")]
#[doc(alias = "__ZN3RBX21PersonalServerService28setPersonalServerRoleSetsUrlESs")]
pub fn stub_891af8() -> ! {
    todo!("0x891af8 RBX::PersonalServerService::setPersonalServerRoleSetsUrl(std::string)")
}

// 0x891bc8 — __ZN3RBX21PersonalServerService14getWebRoleSetsEiN5boost8functionIFvSsEEES4_
// type: void __fastcall(const char **, int, _BOOL4, int)
#[doc(alias = "RBX::PersonalServerService::getWebRoleSets(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX21PersonalServerService14getWebRoleSetsEiN5boost8functionIFvSsEEES4_")]
pub fn stub_891bc8() -> ! {
    todo!("0x891bc8 RBX::PersonalServerService::getWebRoleSets(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x8920c4 — __ZN3RBX21PersonalServerServiceC1Ev
// type: int __fastcall(RBX::PersonalServerService *this)
#[doc(alias = "RBX::PersonalServerService::PersonalServerService(void)")]
#[doc(alias = "__ZN3RBX21PersonalServerServiceC1Ev")]
pub fn stub_8920c4() -> ! {
    todo!("0x8920c4 RBX::PersonalServerService::PersonalServerService(void)")
}

// 0x8920c8 — __ZN3RBX21PersonalServerServiceC2Ev
// type: RBX::Instance *__fastcall(RBX::PersonalServerService *this)
#[doc(alias = "RBX::PersonalServerService::PersonalServerService(void)")]
#[doc(alias = "__ZN3RBX21PersonalServerServiceC2Ev")]
pub fn stub_8920c8() -> ! {
    todo!("0x8920c8 RBX::PersonalServerService::PersonalServerService(void)")
}

// 0x892788 — __ZN3RBX21PersonalServerService19getCurrentPrivilegeEi
// type: int __fastcall(RBX::PersonalServerService *this, int)
#[doc(alias = "RBX::PersonalServerService::getCurrentPrivilege(int)")]
#[doc(alias = "__ZN3RBX21PersonalServerService19getCurrentPrivilegeEi")]
pub fn stub_892788() -> ! {
    todo!("0x892788 RBX::PersonalServerService::getCurrentPrivilege(int)")
}

// 0x892a94 — __ZNK3RBX21PersonalServerService11getRoleSetsEv
// type: int __fastcall(RBX::PersonalServerService *this, int)
#[doc(alias = "RBX::PersonalServerService::getRoleSets(void)const")]
#[doc(alias = "__ZNK3RBX21PersonalServerService11getRoleSetsEv")]
pub fn stub_892a94() -> ! {
    todo!("0x892a94 RBX::PersonalServerService::getRoleSets(void)const")
}

// 0x892aa0 — __ZN3RBX21PersonalServerService11setRoleSetsESs
// type: int __fastcall(int)
#[doc(alias = "RBX::PersonalServerService::setRoleSets(std::string)")]
#[doc(alias = "__ZN3RBX21PersonalServerService11setRoleSetsESs")]
pub fn stub_892aa0() -> ! {
    todo!("0x892aa0 RBX::PersonalServerService::setRoleSets(std::string)")
}

// 0x892e2c — __ZN3RBX21PersonalServerService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// type: void __fastcall(int, int, int, int)
#[doc(alias = "void RBX::PersonalServerService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX21PersonalServerService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")]
pub fn stub_892e2c() -> ! {
    todo!("0x892e2c void RBX::PersonalServerService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x8930d4 — __ZN3RBX21PersonalServerService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// type: void __fastcall(int, int, int, int)
#[doc(alias = "void RBX::PersonalServerService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX21PersonalServerService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")]
pub fn stub_8930d4() -> ! {
    todo!("0x8930d4 void RBX::PersonalServerService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

// 0x89337c — __ZN3RBX21PersonalServerServiceD1Ev
// type: void __fastcall(RBX::PersonalServerService *__hidden this)
#[doc(alias = "RBX::PersonalServerService::~PersonalServerService()")]
#[doc(alias = "__ZN3RBX21PersonalServerServiceD1Ev")]
pub fn stub_89337c() -> ! {
    todo!("0x89337c RBX::PersonalServerService::~PersonalServerService()")
}

// 0x8933d0 — __ZN3RBX21PersonalServerServiceD0Ev
// type: void __fastcall(RBX::PersonalServerService *__hidden this)
#[doc(alias = "RBX::PersonalServerService::~PersonalServerService()")]
#[doc(alias = "__ZN3RBX21PersonalServerServiceD0Ev")]
pub fn stub_8933d0() -> ! {
    todo!("0x8933d0 RBX::PersonalServerService::~PersonalServerService()")
}

// 0x8934e4 — __ZThn32_N3RBX21PersonalServerServiceD1Ev
// type: void __fastcall(RBX::PersonalServerService *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::PersonalServerService::~PersonalServerService()")]
#[doc(alias = "__ZThn32_N3RBX21PersonalServerServiceD1Ev")]
pub fn stub_8934e4() -> ! {
    todo!("0x8934e4 `non-virtual thunk toRBX::PersonalServerService::~PersonalServerService()")
}

// 0x893540 — __ZThn32_N3RBX21PersonalServerServiceD0Ev
// type: void __fastcall(RBX::PersonalServerService *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::PersonalServerService::~PersonalServerService()")]
#[doc(alias = "__ZThn32_N3RBX21PersonalServerServiceD0Ev")]
pub fn stub_893540() -> ! {
    todo!("0x893540 `non-virtual thunk toRBX::PersonalServerService::~PersonalServerService()")
}

// 0x893654 — __ZThn36_N3RBX21PersonalServerServiceD1Ev
// type: void __fastcall(RBX::PersonalServerService *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::PersonalServerService::~PersonalServerService()")]
#[doc(alias = "__ZThn36_N3RBX21PersonalServerServiceD1Ev")]
pub fn stub_893654() -> ! {
    todo!("0x893654 `non-virtual thunk toRBX::PersonalServerService::~PersonalServerService()")
}

// 0x8936b0 — __ZThn36_N3RBX21PersonalServerServiceD0Ev
// type: void __fastcall(RBX::PersonalServerService *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::PersonalServerService::~PersonalServerService()")]
#[doc(alias = "__ZThn36_N3RBX21PersonalServerServiceD0Ev")]
pub fn stub_8936b0() -> ! {
    todo!("0x8936b0 `non-virtual thunk toRBX::PersonalServerService::~PersonalServerService()")
}

// 0x893998 — __ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::resize(unsigned long,RBX::PersonalServerService::PrivilegeType)")]
#[doc(alias = "__ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE6resizeEmS2_")]
pub fn stub_893998() -> ! {
    todo!("0x893998 std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::resize(unsigned long,RBX::PersonalServerService::PrivilegeType)")
}

// 0x8939cc — __ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::push_back(RBX::PersonalServerService::PrivilegeType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE9push_backERKS2_")]
pub fn stub_8939cc() -> ! {
    todo!("0x8939cc std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::push_back(RBX::PersonalServerService::PrivilegeType const&)")
}

// 0x8939f4 — __ZNSt3mapIPKN3RBX4NameENS0_21PersonalServerService13PrivilegeTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::PersonalServerService::PrivilegeType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_21PersonalServerService13PrivilegeTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_8939f4() -> ! {
    todo!("0x8939f4 std::map<RBX::Name const*,RBX::PersonalServerService::PrivilegeType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::operator[](RBX::Name const* const&)")
}

// 0x893a4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_893a4c() -> ! {
    todo!("0x893a4c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType> const&)")
}

// 0x893b00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_893b00() -> ! {
    todo!("0x893b00 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType> const&)")
}

// 0x893b58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_893b58() -> ! {
    todo!("0x893b58 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType> const&)")
}

// 0x893bc0 — __ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PersonalServerService::PrivilegeType*,std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>>,RBX::PersonalServerService::PrivilegeType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_893bc0() -> ! {
    todo!("0x893bc0 std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PersonalServerService::PrivilegeType*,std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>>,RBX::PersonalServerService::PrivilegeType const&)")
}

// 0x893ca4 — __ZNSt12_Vector_baseIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE11_M_allocateEm")]
pub fn stub_893ca4() -> ! {
    todo!("0x893ca4 std::_Vector_base<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::_M_allocate(unsigned long)")
}

// 0x893cbc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX21PersonalServerService13PrivilegeTypeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::PersonalServerService::PrivilegeType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PersonalServerService::PrivilegeType *,RBX::PersonalServerService::PrivilegeType *>(RBX::PersonalServerService::PrivilegeType *,RBX::PersonalServerService::PrivilegeType *,RBX::PersonalServerService::PrivilegeType *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX21PersonalServerService13PrivilegeTypeES6_EET0_T_S8_S7_")]
pub fn stub_893cbc() -> ! {
    todo!("0x893cbc RBX::PersonalServerService::PrivilegeType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PersonalServerService::PrivilegeType *,RBX::PersonalServerService::PrivilegeType *>(RBX::PersonalServerService::PrivilegeType *,RBX::PersonalServerService::PrivilegeType *,RBX::PersonalServerService::PrivilegeType *)")
}

// 0x893cf8 — __ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PersonalServerService::PrivilegeType*,std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>>,unsigned long,RBX::PersonalServerService::PrivilegeType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_893cf8() -> ! {
    todo!("0x893cf8 std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PersonalServerService::PrivilegeType*,std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>>,unsigned long,RBX::PersonalServerService::PrivilegeType const&)")
}

// 0x897658 — __ZN3RBX6CellIDC1Ev
// type: int __fastcall(int this)
#[doc(alias = "RBX::CellID::CellID(void)")]
#[doc(alias = "__ZN3RBX6CellIDC1Ev")]
pub fn stub_897658() -> ! {
    todo!("0x897658 RBX::CellID::CellID(void)")
}

// 0x897678 — __ZN3RBX6CellIDD1Ev
// type: void __fastcall(RBX::CellID *__hidden this)
#[doc(alias = "RBX::CellID::~CellID()")]
#[doc(alias = "__ZN3RBX6CellIDD1Ev")]
pub fn stub_897678() -> ! {
    todo!("0x897678 RBX::CellID::~CellID()")
}

// 0x8979bc — __ZN3RBX12Region3int16C1Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "RBX::Region3int16::Region3int16(void)")]
#[doc(alias = "__ZN3RBX12Region3int16C1Ev")]
pub fn stub_8979bc() -> ! {
    todo!("0x8979bc RBX::Region3int16::Region3int16(void)")
}

// 0x8979e4 — __ZNK3RBX12Region3int169getMinPosEv
// type: int __fastcall(int this, int *)
#[doc(alias = "RBX::Region3int16::getMinPos(void)const")]
#[doc(alias = "__ZNK3RBX12Region3int169getMinPosEv")]
pub fn stub_8979e4() -> ! {
    todo!("0x8979e4 RBX::Region3int16::getMinPos(void)const")
}

// 0x8979f0 — __ZNK3RBX12Region3int169getMaxPosEv
// type: int __fastcall(int this, int)
#[doc(alias = "RBX::Region3int16::getMaxPos(void)const")]
#[doc(alias = "__ZNK3RBX12Region3int169getMaxPosEv")]
pub fn stub_8979f0() -> ! {
    todo!("0x8979f0 RBX::Region3int16::getMaxPos(void)const")
}

// 0x897b78 — __ZN3RBX24CacheableContentProviderC2ENS_22CacheSizeEnforceMethodEm
// type: RBX::Instance *__fastcall(RBX::Instance *, int, int, int, int, int, int, int, int, int, char, RBX::Instance *, RBX::HeartbeatInstance *, pthread_mutex_t *, int, int, void *, void *, int, int, int, int)
#[doc(alias = "RBX::CacheableContentProvider::CacheableContentProvider(RBX::CacheSizeEnforceMethod,unsigned long)")]
#[doc(alias = "__ZN3RBX24CacheableContentProviderC2ENS_22CacheSizeEnforceMethodEm")]
pub fn stub_897b78() -> ! {
    todo!("0x897b78 RBX::CacheableContentProvider::CacheableContentProvider(RBX::CacheSizeEnforceMethod,unsigned long)")
}

// 0x897efc — __ZN3RBX24CacheableContentProviderD0Ev
// type: void __fastcall(RBX::CacheableContentProvider *__hidden this)
#[doc(alias = "RBX::CacheableContentProvider::~CacheableContentProvider()")]
#[doc(alias = "__ZN3RBX24CacheableContentProviderD0Ev")]
pub fn stub_897efc() -> ! {
    todo!("0x897efc RBX::CacheableContentProvider::~CacheableContentProvider()")
}

// 0x897f9c — __ZN3RBX24CacheableContentProviderD1Ev
// type: void __fastcall(RBX::CacheableContentProvider *__hidden this)
#[doc(alias = "RBX::CacheableContentProvider::~CacheableContentProvider()")]
#[doc(alias = "__ZN3RBX24CacheableContentProviderD1Ev")]
pub fn stub_897f9c() -> ! {
    todo!("0x897f9c RBX::CacheableContentProvider::~CacheableContentProvider()")
}

// 0x897fa0 — __ZThn32_N3RBX24CacheableContentProviderD0Ev
// type: void __fastcall(RBX::CacheableContentProvider *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")]
#[doc(alias = "__ZThn32_N3RBX24CacheableContentProviderD0Ev")]
pub fn stub_897fa0() -> ! {
    todo!("0x897fa0 `non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")
}

// 0x897fa8 — __ZThn36_N3RBX24CacheableContentProviderD0Ev
// type: void __fastcall(RBX::CacheableContentProvider *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")]
#[doc(alias = "__ZThn36_N3RBX24CacheableContentProviderD0Ev")]
pub fn stub_897fa8() -> ! {
    todo!("0x897fa8 `non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")
}

// 0x897fb0 — __ZThn96_N3RBX24CacheableContentProviderD0Ev
// type: void __fastcall(RBX::CacheableContentProvider *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")]
#[doc(alias = "__ZThn96_N3RBX24CacheableContentProviderD0Ev")]
pub fn stub_897fb0() -> ! {
    todo!("0x897fb0 `non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")
}

// 0x897fb8 — __ZN3RBX24CacheableContentProviderD2Ev
// type: void __fastcall(RBX::CacheableContentProvider *__hidden this)
#[doc(alias = "RBX::CacheableContentProvider::~CacheableContentProvider()")]
#[doc(alias = "__ZN3RBX24CacheableContentProviderD2Ev")]
pub fn stub_897fb8() -> ! {
    todo!("0x897fb8 RBX::CacheableContentProvider::~CacheableContentProvider()")
}

// 0x8981cc — __ZThn32_N3RBX24CacheableContentProviderD1Ev
// type: void __fastcall(RBX::CacheableContentProvider *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")]
#[doc(alias = "__ZThn32_N3RBX24CacheableContentProviderD1Ev")]
pub fn stub_8981cc() -> ! {
    todo!("0x8981cc `non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")
}

// 0x8981d4 — __ZThn36_N3RBX24CacheableContentProviderD1Ev
// type: void __fastcall(RBX::CacheableContentProvider *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")]
#[doc(alias = "__ZThn36_N3RBX24CacheableContentProviderD1Ev")]
pub fn stub_8981d4() -> ! {
    todo!("0x8981d4 `non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")
}

// 0x8981dc — __ZThn96_N3RBX24CacheableContentProviderD1Ev
// type: void __fastcall(RBX::CacheableContentProvider *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")]
#[doc(alias = "__ZThn96_N3RBX24CacheableContentProviderD1Ev")]
pub fn stub_8981dc() -> ! {
    todo!("0x8981dc `non-virtual thunk toRBX::CacheableContentProvider::~CacheableContentProvider()")
}

// 0x8981e4 — __ZN3RBX24CacheableContentProvider11onHeartbeatERKNS_9HeartbeatE
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::CacheableContentProvider::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider11onHeartbeatERKNS_9HeartbeatE")]
pub fn stub_8981e4() -> ! {
    todo!("0x8981e4 RBX::CacheableContentProvider::onHeartbeat(RBX::Heartbeat const&)")
}

// 0x898324 — __ZThn96_N3RBX24CacheableContentProvider11onHeartbeatERKNS_9HeartbeatE
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "`non-virtual thunk toRBX::CacheableContentProvider::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZThn96_N3RBX24CacheableContentProvider11onHeartbeatERKNS_9HeartbeatE")]
pub fn stub_898324() -> ! {
    todo!("0x898324 `non-virtual thunk toRBX::CacheableContentProvider::onHeartbeat(RBX::Heartbeat const&)")
}

// 0x89873c — __ZN3RBX24CacheableContentProvider17ProcessTaskHelperEN5boost8weak_ptrIS0_EERKSsNS1_10shared_ptrIS4_EE
// type: int __fastcall(int, const std::string *, const shared_count *, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CacheableContentProvider::ProcessTaskHelper(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider17ProcessTaskHelperEN5boost8weak_ptrIS0_EERKSsNS1_10shared_ptrIS4_EE")]
pub fn stub_89873c() -> ! {
    todo!("0x89873c RBX::CacheableContentProvider::ProcessTaskHelper(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&,rbx_core::SharedPtr<std::string const>)")
}

// 0x89887c — __ZN3RBX24CacheableContentProvider15ErrorTaskHelperEN5boost8weak_ptrIS0_EERKSs
// type: void __fastcall(int, int, int, int)
#[doc(alias = "RBX::CacheableContentProvider::ErrorTaskHelper(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider15ErrorTaskHelperEN5boost8weak_ptrIS0_EERKSs")]
pub fn stub_89887c() -> ! {
    todo!("0x89887c RBX::CacheableContentProvider::ErrorTaskHelper(rbx_core::WeakPtr<RBX::CacheableContentProvider>,std::string const&)")
}

// 0x898944 — __ZN3RBX24CacheableContentProvider12setCacheSizeEi
// type: void __fastcall(RBX::CacheableContentProvider *this, int)
#[doc(alias = "RBX::CacheableContentProvider::setCacheSize(int)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider12setCacheSizeEi")]
pub fn stub_898944() -> ! {
    todo!("0x898944 RBX::CacheableContentProvider::setCacheSize(int)")
}

// 0x898a18 — __ZN3RBX24CacheableContentProvider14isAssetContentENS_9ContentIdE
// type: bool __fastcall(int, RBX::ContentId *)
#[doc(alias = "RBX::CacheableContentProvider::isAssetContent(RBX::ContentId)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider14isAssetContentENS_9ContentIdE")]
pub fn stub_898a18() -> ! {
    todo!("0x898a18 RBX::CacheableContentProvider::isAssetContent(RBX::ContentId)")
}

// 0x898a50 — __ZN3RBX24CacheableContentProvider10hasContentERKNS_9ContentIdE
// type: int __fastcall(RBX::CacheableContentProvider *this, const RBX::ContentId *)
#[doc(alias = "RBX::CacheableContentProvider::hasContent(RBX::ContentId const&)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider10hasContentERKNS_9ContentIdE")]
pub fn stub_898a50() -> ! {
    todo!("0x898a50 RBX::CacheableContentProvider::hasContent(RBX::ContentId const&)")
}

// 0x898bc0 — __ZN3RBX24CacheableContentProvider16getContentStatusERKSs
// type: int __fastcall(RBX::CacheableContentProvider *this, const std::string *)
#[doc(alias = "RBX::CacheableContentProvider::getContentStatus(std::string const&)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider16getContentStatusERKSs")]
pub fn stub_898bc0() -> ! {
    todo!("0x898bc0 RBX::CacheableContentProvider::getContentStatus(std::string const&)")
}

// 0x898cdc — __ZN3RBX24CacheableContentProvider13isAssetFailedERKNS_9ContentIdE
// type: int __fastcall(RBX::CacheableContentProvider *this, const RBX::ContentId *)
#[doc(alias = "RBX::CacheableContentProvider::isAssetFailed(RBX::ContentId const&)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider13isAssetFailedERKNS_9ContentIdE")]
pub fn stub_898cdc() -> ! {
    todo!("0x898cdc RBX::CacheableContentProvider::isAssetFailed(RBX::ContentId const&)")
}

// 0x898e68 — __ZN3RBX24CacheableContentProvider12fetchContentERKNS_9ContentIdE
// type: void __fastcall(RBX::CacheableContentProvider *this, const RBX::ContentId *, _DWORD *)
#[doc(alias = "RBX::CacheableContentProvider::fetchContent(RBX::ContentId const&)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider12fetchContentERKNS_9ContentIdE")]
pub fn stub_898e68() -> ! {
    todo!("0x898e68 RBX::CacheableContentProvider::fetchContent(RBX::ContentId const&)")
}

// 0x899020 — __ZN3RBX24CacheableContentProvider14requestContentERKNS_9ContentIdEfbRNS_14AsyncHttpQueue13RequestResultE
// type: void __fastcall(_QWORD *, int32_t *, const std::string *, int, int, _DWORD *)
#[doc(alias = "RBX::CacheableContentProvider::requestContent(RBX::ContentId const&,float,bool,RBX::AsyncHttpQueue::RequestResult &)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider14requestContentERKNS_9ContentIdEfbRNS_14AsyncHttpQueue13RequestResultE")]
pub fn stub_899020() -> ! {
    todo!("0x899020 RBX::CacheableContentProvider::requestContent(RBX::ContentId const&,float,bool,RBX::AsyncHttpQueue::RequestResult &)")
}
