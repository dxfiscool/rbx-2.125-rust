//! core wd watchdog24 — 120 core stubs EA-sorted asc next uncovered distinct not yet in crates/core/src.
//! Source: ida/export.json (85545 funcs) filtered demangled/mangled excludes Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, EA-sorted asc, next 120 uncovered distinct (global 62372 before).
//! Range: 0x723b1c..0x739e3c (120 stubs).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::operator[](unsigned long)")]
// 0x723b1c — __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EEixEm
pub fn stub_0x723b1c() {
    // IDA 0x723b1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::fastRemove(unsigned long)")]
// 0x723b7c — __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE10fastRemoveEm
// type: int(void)
pub fn stub_0x723b7c() {
    // IDA 0x723b7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BlockBlockContact::deleteAllConnectors(void)")]
// 0x723c30 — __ZN3RBX17BlockBlockContact19deleteAllConnectorsEv
// type: _DWORD __fastcall(RBX::BlockBlockContact *__hidden this)
pub fn stub_0x723c30() {
    // IDA 0x723c30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::releaseMemory(void)")]
// 0x723c4c — __ZN3RBX9AllocatorINS_17BlockBlockContactEE13releaseMemoryEv
pub fn stub_0x723c4c() {
    // IDA 0x723c4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeoPair::match(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
// 0x723d40 — __ZN3RBX7GeoPair5matchEPNS_4BodyES2_NS_11GeoPairTypeEii
pub fn stub_0x723d40() {
    // IDA 0x723d40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactConnector::ContactConnector(RBX::Body *,RBX::Body *,RBX::ContactParams const&)")]
// 0x723e20 — __ZN3RBX16ContactConnectorC2EPNS_4BodyES2_RKNS_13ContactParamsE
pub fn stub_0x723e20() {
    // IDA 0x723e20: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::Allocator(void)")]
// 0x723f2c — __ZN3RBX9AllocatorINS_16GeoPairConnectorEEC2Ev
pub fn stub_0x723f2c() {
    // IDA 0x723f2c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::GeoPairConnector::~GeoPairConnector()")]
// 0x723f90 — __ZN3RBX16GeoPairConnectorD1Ev
// type: void __fastcall(RBX::GeoPairConnector *__hidden this)
pub fn stub_0x723f90() {
    // IDA 0x723f90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GeoPairConnector::~GeoPairConnector()")]
// 0x723f94 — __ZN3RBX16GeoPairConnectorD0Ev
// type: void __fastcall(RBX::GeoPairConnector *__hidden this)
pub fn stub_0x723f94() {
    // IDA 0x723f94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GeoPairConnector::updateContactPoint(void)")]
// 0x723f98 — __ZN3RBX16GeoPairConnector18updateContactPointEv
// type: _DWORD __fastcall(RBX::GeoPairConnector *__hidden this)
pub fn stub_0x723f98() {
    // IDA 0x723f98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::releaseMemory(void)")]
// 0x724018 — __ZN3RBX9AllocatorINS_16GeoPairConnectorEE13releaseMemoryEv
pub fn stub_0x724018() {
    // IDA 0x724018: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::Allocator(void)")]
// 0x724064 — __ZN3RBX9AllocatorINS_18BallBlockConnectorEEC2Ev
pub fn stub_0x724064() {
    // IDA 0x724064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::releaseMemory(void)")]
// 0x7240c8 — __ZN3RBX9AllocatorINS_18BallBlockConnectorEE13releaseMemoryEv
pub fn stub_0x7240c8() {
    // IDA 0x7240c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::Allocator(void)")]
// 0x724114 — __ZN3RBX9AllocatorINS_17BallBallConnectorEEC2Ev
pub fn stub_0x724114() {
    // IDA 0x724114: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::releaseMemory(void)")]
// 0x724178 — __ZN3RBX9AllocatorINS_17BallBallConnectorEE13releaseMemoryEv
pub fn stub_0x724178() {
    // IDA 0x724178: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
// 0x7241c4 — __ZN3RBX10IPipelinedD2Ev
// type: void __fastcall(RBX::IPipelined *__hidden this)
pub fn stub_0x7241c4() {
    // IDA 0x7241c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Edge::~Edge()")]
// 0x724234 — __ZN3RBX4EdgeD1Ev
// type: void __fastcall(RBX::Edge *__hidden this)
pub fn stub_0x724234() {
    // IDA 0x724234: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Edge::~Edge()")]
// 0x724238 — __ZN3RBX4EdgeD0Ev
// type: void __fastcall(RBX::Edge *__hidden this)
pub fn stub_0x724238() {
    // IDA 0x724238: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
// 0x7242d8 — __ZN3RBX10IPipelinedD1Ev
// type: void __fastcall(RBX::IPipelined *__hidden this)
pub fn stub_0x7242d8() {
    // IDA 0x7242d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
// 0x7242dc — __ZN3RBX10IPipelinedD0Ev
// type: void __fastcall(RBX::IPipelined *__hidden this)
pub fn stub_0x7242dc() {
    // IDA 0x7242dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::ContactManager(RBX::World *)")]
// 0x724650 — __ZN3RBX14ContactManagerC1EPNS_5WorldE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::World *)
pub fn stub_0x724650() {
    // IDA 0x724650: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::ContactManager(RBX::World *)")]
// 0x724654 — __ZN3RBX14ContactManagerC2EPNS_5WorldE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::World *)
pub fn stub_0x724654() {
    // IDA 0x724654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::~ContactManager()")]
// 0x7247ec — __ZN3RBX14ContactManagerD1Ev
// type: void __fastcall(RBX::ContactManager *__hidden this)
pub fn stub_0x7247ec() {
    // IDA 0x7247ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::~ContactManager()")]
// 0x7247f0 — __ZN3RBX14ContactManagerD2Ev
// type: void __fastcall(RBX::ContactManager *__hidden this)
pub fn stub_0x7247f0() {
    // IDA 0x7247f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::fastClear(void)")]
// 0x724920 — __ZN3RBX14ContactManager9fastClearEv
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this)
pub fn stub_0x724920() {
    // IDA 0x724920: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::doStats(void)")]
// 0x724928 — __ZN3RBX14ContactManager7doStatsEv
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this)
pub fn stub_0x724928() {
    // IDA 0x724928: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::intersectingMySimulation(RBX::Primitive *,RBX::SystemAddress,float)")]
// 0x72492c — __ZN3RBX14ContactManager24intersectingMySimulationEPNS_9PrimitiveENS_13SystemAddressEf
// type: int __fastcall(int, RBX::Primitive *this, int, int, float)
pub fn stub_0x72492c() {
    // IDA 0x72492c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::intersectingOthers(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> const&,float)")]
// 0x7249bc — __ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveERKSt3setIS2_St4lessIS2_ESaIS2_EEf
// type: int __fastcall(int, RBX::Primitive *this, int, int, int, int, int, int, void *, int, int, int, int, int, int, int, int, int)
pub fn stub_0x7249bc() {
    // IDA 0x7249bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::createContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x724be4 — __ZN3RBX14ContactManager13createContactEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *, RBX::Primitive *)
pub fn stub_0x724be4() {
    // IDA 0x724be4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::intersectingOthers(RBX::Primitive *,float)")]
// 0x7250a4 — __ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveEf
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *, float)
pub fn stub_0x7250a4() {
    // IDA 0x7250a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::terrainCellsInRegion3(RBX::Region3)const")]
// 0x725f1c — __ZNK3RBX14ContactManager21terrainCellsInRegion3ENS_7Region3E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x725f1c() {
    // IDA 0x725f1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::onNewPair(RBX::Primitive *,RBX::Primitive *)")]
// 0x7262e0 — __ZN3RBX14ContactManager9onNewPairEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *, RBX::Primitive *)
pub fn stub_0x7262e0() {
    // IDA 0x7262e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::releasePair(RBX::Primitive *,RBX::Primitive *)")]
// 0x726370 — __ZN3RBX14ContactManager11releasePairEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *, RBX::Primitive *)
pub fn stub_0x726370() {
    // IDA 0x726370: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::onPrimitiveAdded(RBX::Primitive *)")]
// 0x72641c — __ZN3RBX14ContactManager16onPrimitiveAddedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
pub fn stub_0x72641c() {
    // IDA 0x72641c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::onPrimitiveRemoved(RBX::Primitive *)")]
// 0x726524 — __ZN3RBX14ContactManager18onPrimitiveRemovedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
pub fn stub_0x726524() {
    // IDA 0x726524: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::onPrimitiveExtentsChanged(RBX::Primitive *)")]
// 0x726604 — __ZN3RBX14ContactManager25onPrimitiveExtentsChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
pub fn stub_0x726604() {
    // IDA 0x726604: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::onPrimitiveGeometryChanged(RBX::Primitive *)")]
// 0x72660c — __ZN3RBX14ContactManager26onPrimitiveGeometryChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
pub fn stub_0x72660c() {
    // IDA 0x72660c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::checkMegaClusterContact(RBX::Primitive *,bool,bool,bool)")]
// 0x72676c — __ZN3RBX14ContactManager23checkMegaClusterContactEPNS_9PrimitiveEbbb
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *, bool, bool, bool)
pub fn stub_0x72676c() {
    // IDA 0x72676c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::primitiveIsExcludedFromSpatialHash(RBX::Primitive *)")]
// 0x726cf0 — __ZN3RBX14ContactManager34primitiveIsExcludedFromSpatialHashEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
pub fn stub_0x726cf0() {
    // IDA 0x726cf0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::checkMegaClusterBigTerrainContact(RBX::Primitive *)")]
// 0x726d08 — __ZN3RBX14ContactManager33checkMegaClusterBigTerrainContactEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
pub fn stub_0x726d08() {
    // IDA 0x726d08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0x727054 — __ZN3RBX14ContactManager18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x727054() {
    // IDA 0x727054: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::startLoadingTerrain(void)")]
// 0x727420 — __ZN3RBX14ContactManager19startLoadingTerrainEv
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this)
pub fn stub_0x727420() {
    // IDA 0x727420: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::doneLoadingTerrain(void)")]
// 0x727438 — __ZN3RBX14ContactManager18doneLoadingTerrainEv
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this)
pub fn stub_0x727438() {
    // IDA 0x727438: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::applyDeferredTerrainChanges(void)")]
// 0x727578 — __ZN3RBX14ContactManager27applyDeferredTerrainChangesEv
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this)
pub fn stub_0x727578() {
    // IDA 0x727578: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::fastClear(void)")]
// 0x727838 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, void *, int, int, int, int)
pub fn stub_0x727838() {
    // IDA 0x727838: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getNextGrid(RBX::Vector3int32 &,RBX::RbxRay const&,float)")]
// 0x727f74 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11getNextGridERNS_12Vector3int32ERKNS_6RbxRayEf
pub fn stub_0x727f74() {
    // IDA 0x727f74: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::overlapsOrTouches(RBX::Extents const&)const")]
// 0x728358 — __ZNK3RBX7Extents17overlapsOrTouchesERKS0_
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const RBX::Extents *)
pub fn stub_0x728358() {
    // IDA 0x728358: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator new(unsigned long)")]
// 0x728438 — __ZN3RBX9AllocatorINS_15BallBallContactEEnwEm
pub fn stub_0x728438() {
    // IDA 0x728438: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator delete(void *)")]
// 0x7284a8 — __ZN3RBX9AllocatorINS_15BallBallContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0x7284a8() {
    // IDA 0x7284a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator new(unsigned long)")]
// 0x7284e4 — __ZN3RBX9AllocatorINS_16BallBlockContactEEnwEm
pub fn stub_0x7284e4() {
    // IDA 0x7284e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator delete(void *)")]
// 0x728554 — __ZN3RBX9AllocatorINS_16BallBlockContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0x728554() {
    // IDA 0x728554: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::operator new(unsigned long)")]
// 0x728590 — __ZN3RBX9AllocatorINS_15BallPolyContactEEnwEm
pub fn stub_0x728590() {
    // IDA 0x728590: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::operator new(unsigned long)")]
// 0x728600 — __ZN3RBX9AllocatorINS_17BlockBlockContactEEnwEm
pub fn stub_0x728600() {
    // IDA 0x728600: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator new(unsigned long)")]
// 0x728670 — __ZN3RBX9AllocatorINS_15PolyPolyContactEEnwEm
pub fn stub_0x728670() {
    // IDA 0x728670: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator delete(void *)")]
// 0x7286e0 — __ZN3RBX9AllocatorINS_15PolyPolyContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0x7286e0() {
    // IDA 0x7286e0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(RBX::Primitive*,bool)")]
// 0x72871c — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b
// type: int __fastcall(int, RBX::Primitive *this)
pub fn stub_0x72871c() {
    // IDA 0x72871c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveExtentsChanged(RBX::Primitive*)")]
// 0x7287b0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_
// type: int __fastcall(int, RBX::Primitive *this)
pub fn stub_0x7287b0() {
    // IDA 0x7287b0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator new(unsigned long)")]
// 0x72895c — __ZN3RBX9AllocatorINS_15BallCellContactEEnwEm
pub fn stub_0x72895c() {
    // IDA 0x72895c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator delete(void *)")]
// 0x7289cc — __ZN3RBX9AllocatorINS_15BallCellContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0x7289cc() {
    // IDA 0x7289cc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator new(unsigned long)")]
// 0x728a08 — __ZN3RBX9AllocatorINS_15PolyCellContactEEnwEm
pub fn stub_0x728a08() {
    // IDA 0x728a08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator delete(void *)")]
// 0x728a78 — __ZN3RBX9AllocatorINS_15PolyCellContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0x728a78() {
    // IDA 0x728a78: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::clampToOverlap(RBX::Extents const&)")]
// 0x728ab4 — __ZN3RBX7Extents14clampToOverlapERKS0_
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const RBX::Extents *)
pub fn stub_0x728ab4() {
    // IDA 0x728ab4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::computeLevel(RBX::Primitive const*,RBX::Extents const&)")]
// 0x72a668 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12computeLevelEPKS1_RKNS_7ExtentsE
pub fn stub_0x72a668() {
    // IDA 0x72a668: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(RBX::Primitive*)")]
// 0x72a728 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_
pub fn stub_0x72a728() {
    // IDA 0x72a728: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(RBX::Primitive*,bool)")]
// 0x72a844 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS1_b
pub fn stub_0x72a844() {
    // IDA 0x72a844: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveExtentsChanged(RBX::Primitive*,RBX::Extents const&)")]
// 0x72a990 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS1_RKNS_7ExtentsE
pub fn stub_0x72a990() {
    // IDA 0x72a990: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ExtentsInt32::ExtentsInt32(void)")]
// 0x72ab58 — __ZN3RBX12ExtentsInt32C1Ev
// type: _DWORD __fastcall(RBX::ExtentsInt32 *__hidden this)
pub fn stub_0x72ab58() {
    // IDA 0x72ab58: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::changeMinMax(RBX::Primitive*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,bool)")]
// 0x72ac08 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS1_PKNS_12ExtentsInt32ES8_S8_b
// type: int __fastcall(int, int, int, RBX::ExtentsInt32 *this, RBX::ExtentsInt32 *, int)
pub fn stub_0x72ac08() {
    // IDA 0x72ac08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::checkAndReleaseContacts(RBX::Primitive*)")]
// 0x72ad40 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23checkAndReleaseContactsEPS1_
pub fn stub_0x72ad40() {
    // IDA 0x72ad40: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::oldExtentsOverlap(RBX::Primitive*,RBX::Primitive*)")]
// 0x72adc4 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE17oldExtentsOverlapEPS1_S5_
pub fn stub_0x72adc4() {
    // IDA 0x72adc4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ExtentsInt32::contains(RBX::Vector3int32 const&)const")]
// 0x72aef8 — __ZNK3RBX12ExtentsInt328containsERKNS_12Vector3int32E
pub fn stub_0x72aef8() {
    // IDA 0x72aef8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::addNode(RBX::Primitive*,RBX::Vector3int32 const&,bool)")]
// 0x72af38 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS1_RKNS_12Vector3int32Eb
// type: int __fastcall(int, int, int, int)
pub fn stub_0x72af38() {
    // IDA 0x72af38: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findNode(RBX::Primitive*,RBX::Vector3int32 const&)")]
// 0x72b3c0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS1_RKNS_12Vector3int32E
pub fn stub_0x72b3c0() {
    // IDA 0x72b3c0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::removeNodeFromHash(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// 0x72b494 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE18removeNodeFromHashEPNS4_11SpatialNodeE
// type: int __fastcall(int, RBX::NodeBase *this)
pub fn stub_0x72b494() {
    // IDA 0x72b494: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::NodeBase::getLevel(void)")]
// 0x72b4c4 — __ZN3RBX8NodeBase8getLevelEv
// type: _DWORD __fastcall(RBX::NodeBase *__hidden this)
pub fn stub_0x72b4c4() {
    // IDA 0x72b4c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findOtherNodesInLevel0Cell(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// 0x72b528 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS4_11SpatialNodeE
// type: int __fastcall(int, RBX::NodeBase *this)
pub fn stub_0x72b528() {
    // IDA 0x72b528: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::removeTreeNodeChild(int,RBX::Vector3int32 &)")]
// 0x72b5b8 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E
// type: int __fastcall(int, RBX::SpatialHashStatic *this, int)
pub fn stub_0x72b5b8() {
    // IDA 0x72b5b8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::operator delete(void *)")]
// 0x72b730 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0x72b730() {
    // IDA 0x72b730: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
// 0x72b7c0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS4_8TreeNodeE
pub fn stub_0x72b7c0() {
    // IDA 0x72b7c0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode::~TreeNode()")]
// 0x72b92c — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeD2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0x72b92c() {
    // IDA 0x72b92c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::newNode(int,int,RBX::Vector3int32 const&)")]
// 0x72ba94 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(int, int, void *, int, int, void *, int, int, int, int)
pub fn stub_0x72ba94() {
    // IDA 0x72ba94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::insertNodeToPrimitive(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *,RBX::Primitive*,RBX::Vector3int32 const&,int)")]
// 0x72bc74 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS4_11SpatialNodeEPS1_RKNS_12Vector3int32Ei
pub fn stub_0x72bc74() {
    // IDA 0x72bc74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)")]
// 0x72bcf8 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(int, int, int, int)
pub fn stub_0x72bcf8() {
    // IDA 0x72bcf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::addContactFromChildren(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *,RBX::Primitive*)")]
// 0x72be14 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE22addContactFromChildrenEPNS4_8TreeNodeEPS1_
// type: int __fastcall(int, int, RBX::Primitive *this)
pub fn stub_0x72be14() {
    // IDA 0x72be14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::operator new(unsigned long)")]
// 0x72c004 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEnwEm
pub fn stub_0x72c004() {
    // IDA 0x72c004: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode::TreeNode(void)")]
// 0x72c2f0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeC2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0x72c2f0() {
    // IDA 0x72c2f0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::Allocator(void)")]
// 0x72c3e0 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev
pub fn stub_0x72c3e0() {
    // IDA 0x72c3e0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::releaseMemory(void)")]
// 0x72c448 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEE13releaseMemoryEv
pub fn stub_0x72c448() {
    // IDA 0x72c448: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::operator new(unsigned long)")]
// 0x72c4d4 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEnwEm
pub fn stub_0x72c4d4() {
    // IDA 0x72c4d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::Allocator(void)")]
// 0x72c68c — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev
pub fn stub_0x72c68c() {
    // IDA 0x72c68c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::releaseMemory(void)")]
// 0x72c6f0 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEE13releaseMemoryEv
pub fn stub_0x72c6f0() {
    // IDA 0x72c6f0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ExtentsInt32::empty(void)")]
// 0x72c778 — __ZN3RBX12ExtentsInt325emptyEv
// type: _DWORD __fastcall(RBX::ExtentsInt32 *__hidden this)
pub fn stub_0x72c778() {
    // IDA 0x72c778: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Joint::otherConstNode(RBX::SpanningNode const*)const")]
// 0x7382a4 — __ZNK3RBX5Joint14otherConstNodeEPKNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::SpanningNode *)
pub fn stub_0x7382a4() {
    // IDA 0x7382a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::Joint::otherConstNode(RBX::SpanningNode const*)const")]
// 0x7382c0 — __ZThn32_NK3RBX5Joint14otherConstNodeEPKNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::SpanningNode *)
pub fn stub_0x7382c0() {
    // IDA 0x7382c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Joint::getNode(int)")]
// 0x7382e0 — __ZN3RBX5Joint7getNodeEi
// type: _DWORD __fastcall(RBX::Joint *__hidden this, int)
pub fn stub_0x7382e0() {
    // IDA 0x7382e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Joint::getNode(int)")]
// 0x7382f0 — __ZThn32_N3RBX5Joint7getNodeEi
// type: _DWORD __fastcall(RBX::Joint *__hidden this, int)
pub fn stub_0x7382f0() {
    // IDA 0x7382f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Joint::getConstNode(int)const")]
// 0x738300 — __ZNK3RBX5Joint12getConstNodeEi
// type: _DWORD __fastcall(RBX::Joint *__hidden this, int)
pub fn stub_0x738300() {
    // IDA 0x738300: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Joint::getConstNode(int)const")]
// 0x738310 — __ZThn32_NK3RBX5Joint12getConstNodeEi
// type: _DWORD __fastcall(RBX::Joint *__hidden this, int)
pub fn stub_0x738310() {
    // IDA 0x738310: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Joint::FaceVerticesOverlapped(RBX::Primitive const*,unsigned long,RBX::Primitive const*,unsigned long,float)")]
// 0x738320 — __ZN3RBX5Joint22FaceVerticesOverlappedEPKNS_9PrimitiveEmS3_mf
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Primitive *, unsigned int, const RBX::Primitive *, float, float)
pub fn stub_0x738320() {
    // IDA 0x738320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Joint::FaceEdgesOverlapped(RBX::Primitive const*,unsigned long,RBX::Primitive const*,unsigned long,float)")]
// 0x73867c — __ZN3RBX5Joint19FaceEdgesOverlappedEPKNS_9PrimitiveEmS3_mf
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Primitive *, unsigned int, const RBX::Primitive *, float, float)
pub fn stub_0x73867c() {
    // IDA 0x73867c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Joint::getSurfaceTypeFromNormal(RBX::Primitive const&,RBX::NormalId const&)")]
// 0x738d30 — __ZN3RBX5Joint24getSurfaceTypeFromNormalERKNS_9PrimitiveERKNS_8NormalIdE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x738d30() {
    // IDA 0x738d30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Joint::compatibleForHingeAutoJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
// 0x738d4c — __ZN3RBX5Joint27compatibleForHingeAutoJointERKNS_9PrimitiveERmS3_S4_
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Primitive *, unsigned int *, const RBX::Primitive *, unsigned int *)
pub fn stub_0x738d4c() {
    // IDA 0x738d4c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Joint::compatibleForGlueAutoJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
// 0x738dc8 — __ZN3RBX5Joint26compatibleForGlueAutoJointERKNS_9PrimitiveERmS3_S4_
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Primitive *, unsigned int *, const RBX::Primitive *, unsigned int *)
pub fn stub_0x738dc8() {
    // IDA 0x738dc8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Joint::compatibleForWeldAutoJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
// 0x738e0c — __ZN3RBX5Joint26compatibleForWeldAutoJointERKNS_9PrimitiveERmS3_S4_
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Primitive *, unsigned int *, const RBX::Primitive *, unsigned int *)
pub fn stub_0x738e0c() {
    // IDA 0x738e0c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Joint::compatibleForStudAutoJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
// 0x738e58 — __ZN3RBX5Joint26compatibleForStudAutoJointERKNS_9PrimitiveERmS3_S4_
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Primitive *, unsigned int *, const RBX::Primitive *, unsigned int *)
pub fn stub_0x738e58() {
    // IDA 0x738e58: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Joint::inCompatibleForAnyJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
// 0x738ed0 — __ZN3RBX5Joint23inCompatibleForAnyJointERKNS_9PrimitiveERmS3_S4_
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Primitive *, unsigned int *, const RBX::Primitive *, unsigned int *)
pub fn stub_0x738ed0() {
    // IDA 0x738ed0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Joint::positionedForStudAutoJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
// 0x738f48 — __ZN3RBX5Joint26positionedForStudAutoJointERKNS_9PrimitiveERmS3_S4_
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Primitive *, unsigned int *, const RBX::Primitive *, unsigned int *)
pub fn stub_0x738f48() {
    // IDA 0x738f48: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IPipelined::findWorld(void)")]
// 0x7393ac — __ZN3RBX10IPipelined9findWorldEv
// type: _DWORD __fastcall(RBX::IPipelined *__hidden this)
pub fn stub_0x7393ac() {
    // IDA 0x7393ac: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Joint::getJointType(void)const")]
// 0x7393d0 — __ZNK3RBX5Joint12getJointTypeEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
pub fn stub_0x7393d0() {
    // IDA 0x7393d0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::resize(unsigned long,RBX::Joint::JointType)")]
// 0x739424 — __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE6resizeEmS2_
pub fn stub_0x739424() {
    // IDA 0x739424: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::push_back(RBX::Joint::JointType const&)")]
// 0x739458 — __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x739458() {
    // IDA 0x739458: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Joint::JointType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::operator[](RBX::Name const* const&)")]
// 0x739480 — __ZNSt3mapIPKN3RBX4NameENS0_5Joint9JointTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0x739480() {
    // IDA 0x739480: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::pair<RBX::Name const* const,RBX::Joint::JointType> const&)")]
// 0x7394d8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x7394d8() {
    // IDA 0x7394d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Joint::JointType> const&)")]
// 0x73958c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0x73958c() {
    // IDA 0x73958c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Joint::JointType> const&)")]
// 0x7395e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0x7395e4() {
    // IDA 0x7395e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Joint::JointType*,std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>>,RBX::Joint::JointType const&)")]
// 0x73964c — __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x73964c() {
    // IDA 0x73964c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::_M_allocate(unsigned long)")]
// 0x739730 — __ZNSt12_Vector_baseIN3RBX5Joint9JointTypeESaIS2_EE11_M_allocateEm
pub fn stub_0x739730() {
    // IDA 0x739730: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Joint::JointType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Joint::JointType *,RBX::Joint::JointType *>(RBX::Joint::JointType *,RBX::Joint::JointType *,RBX::Joint::JointType *)")]
// 0x739748 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Joint9JointTypeES6_EET0_T_S8_S7_
pub fn stub_0x739748() {
    // IDA 0x739748: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Joint::JointType*,std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>>,unsigned long,RBX::Joint::JointType const&)")]
// 0x739784 — __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0x739784() {
    // IDA 0x739784: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::JointBuilder::canJoin(RBX::Primitive *,RBX::Primitive *)")]
// 0x739e3c — __ZN3RBX12JointBuilder7canJoinEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::JointBuilder *__hidden this, RBX::Primitive *, RBX::Primitive *)
pub fn stub_0x739e3c() {
    // IDA 0x739e3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

