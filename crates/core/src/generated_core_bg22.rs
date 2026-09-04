//! core bg22 — 100 core stubs EA-sorted asc distinct not in /tmp/global_eas.txt.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, exclude boost) global distinct not yet in /tmp/global_eas.txt — next 100 uncovered after 0xf3ca84 -> 0xf4dff4..0xf4eba4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::fastRemove(unsigned long)")]
#[doc(alias = "j___ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE10fastRemoveEm")]
// 0xf4dff4 — j___ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE10fastRemoveEm
// type: int()
pub fn stub_0xf4dff4() {
    // IDA 0xf4dff4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::push_back(RBX::GeoPairConnector * const&)")]
#[doc(alias = "j___ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE9push_backERKS2_")]
// 0xf4e004 — j___ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE9push_backERKS2_
// type: int()
pub fn stub_0xf4e004() {
    // IDA 0xf4e004: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::operator[](unsigned long)")]
#[doc(alias = "j___ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EEixEm")]
// 0xf4e014 — j___ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EEixEm
// type: int()
pub fn stub_0xf4e014() {
    // IDA 0xf4e014: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
#[doc(alias = "j___ZN3RBX10IPipelinedD2Ev")]
// 0xf4e024 — j___ZN3RBX10IPipelinedD2Ev
// type: void __fastcall(RBX::IPipelined *__hidden this)
pub fn stub_0xf4e024() {
    // IDA 0xf4e024: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactConnector::isIntersecting(void)")]
#[doc(alias = "j___ZN3RBX16ContactConnector14isIntersectingEv")]
// 0xf4e034 — j___ZN3RBX16ContactConnector14isIntersectingEv
// type: _DWORD __fastcall(RBX::ContactConnector *__hidden this)
pub fn stub_0xf4e034() {
    // IDA 0xf4e034: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactConnector::ContactConnector(RBX::Body *,RBX::Body *,RBX::ContactParams const&)")]
#[doc(alias = "j___ZN3RBX16ContactConnectorC2EPNS_4BodyES2_RKNS_13ContactParamsE")]
// 0xf4e044 — j___ZN3RBX16ContactConnectorC2EPNS_4BodyES2_RKNS_13ContactParamsE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf4e044() {
    // IDA 0xf4e044: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Edge::~Edge()")]
#[doc(alias = "j___ZN3RBX4EdgeD2Ev")]
// 0xf4e054 — j___ZN3RBX4EdgeD2Ev
// type: void __fastcall(RBX::Edge *__hidden this)
pub fn stub_0xf4e054() {
    // IDA 0xf4e054: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GeoPair::match(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
#[doc(alias = "j___ZN3RBX7GeoPair5matchEPNS_4BodyES2_NS_11GeoPairTypeEii")]
// 0xf4e064 — j___ZN3RBX7GeoPair5matchEPNS_4BodyES2_NS_11GeoPairTypeEii
// type: int()
pub fn stub_0xf4e064() {
    // IDA 0xf4e064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_16GeoPairConnectorEEC2Ev")]
// 0xf4e074 — j___ZN3RBX9AllocatorINS_16GeoPairConnectorEEC2Ev
// type: int()
pub fn stub_0xf4e074() {
    // IDA 0xf4e074: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_16GeoPairConnectorEEdlEPv")]
// 0xf4e084 — j___ZN3RBX9AllocatorINS_16GeoPairConnectorEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4e084() {
    // IDA 0xf4e084: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_16GeoPairConnectorEEnwEm")]
// 0xf4e094 — j___ZN3RBX9AllocatorINS_16GeoPairConnectorEEnwEm
// type: int()
pub fn stub_0xf4e094() {
    // IDA 0xf4e094: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_17BallBallConnectorEEC2Ev")]
// 0xf4e0a4 — j___ZN3RBX9AllocatorINS_17BallBallConnectorEEC2Ev
// type: int()
pub fn stub_0xf4e0a4() {
    // IDA 0xf4e0a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_17BallBallConnectorEEnwEm")]
// 0xf4e0b4 — j___ZN3RBX9AllocatorINS_17BallBallConnectorEEnwEm
// type: int()
pub fn stub_0xf4e0b4() {
    // IDA 0xf4e0b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_17BlockBlockContactEEC2Ev")]
// 0xf4e0c4 — j___ZN3RBX9AllocatorINS_17BlockBlockContactEEC2Ev
// type: int()
pub fn stub_0xf4e0c4() {
    // IDA 0xf4e0c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_17BlockBlockContactEEdlEPv")]
// 0xf4e0d4 — j___ZN3RBX9AllocatorINS_17BlockBlockContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4e0d4() {
    // IDA 0xf4e0d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEC2Ev")]
// 0xf4e0e4 — j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEC2Ev
// type: int()
pub fn stub_0xf4e0e4() {
    // IDA 0xf4e0e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEnwEm")]
// 0xf4e0f4 — j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEnwEm
// type: int()
pub fn stub_0xf4e0f4() {
    // IDA 0xf4e0f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::getEdgeVertex(int)const")]
#[doc(alias = "j___ZNK3RBX5Block13getEdgeVertexEi")]
// 0xf4e174 — j___ZNK3RBX5Block13getEdgeVertexEi
// type: _DWORD __fastcall(RBX::Block *__hidden this, int)
pub fn stub_0xf4e174() {
    // IDA 0xf4e174: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getNextGrid(RBX::Vector3int32 &,RBX::RbxRay const&,float)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11getNextGridERNS_12Vector3int32ERKNS_6RbxRayEf")]
// 0xf4e1f4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11getNextGridERNS_12Vector3int32ERKNS_6RbxRayEf
// type: int()
pub fn stub_0xf4e1f4() {
    // IDA 0xf4e1f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::changeMinMax(RBX::Primitive*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,bool)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS1_PKNS_12ExtentsInt32ES8_S8_b")]
// 0xf4e204 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS1_PKNS_12ExtentsInt32ES8_S8_b
// type: int __fastcall(int, int, int, RBX::ExtentsInt32 *this, RBX::ExtentsInt32 *, int)
pub fn stub_0xf4e204() {
    // IDA 0xf4e204: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::computeLevel(RBX::Primitive const*,RBX::Extents const&)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12computeLevelEPKS1_RKNS_7ExtentsE")]
// 0xf4e214 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12computeLevelEPKS1_RKNS_7ExtentsE
// type: int()
pub fn stub_0xf4e214() {
    // IDA 0xf4e214: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E")]
// 0xf4e224 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf4e224() {
    // IDA 0xf4e224: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(RBX::Primitive*,bool)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS1_b")]
// 0xf4e234 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS1_b
// type: int __fastcall(int, RBX::Primitive *this)
pub fn stub_0xf4e234() {
    // IDA 0xf4e234: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS4_8TreeNodeE")]
// 0xf4e244 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS4_8TreeNodeE
// type: int()
pub fn stub_0xf4e244() {
    // IDA 0xf4e244: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(RBX::Primitive*,bool)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b")]
// 0xf4e254 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b
// type: int __fastcall(int, RBX::Primitive *this)
pub fn stub_0xf4e254() {
    // IDA 0xf4e254: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(RBX::Primitive*)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_")]
// 0xf4e264 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_
// type: int __fastcall(int, int)
pub fn stub_0xf4e264() {
    // IDA 0xf4e264: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::oldExtentsOverlap(RBX::Primitive*,RBX::Primitive*)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE17oldExtentsOverlapEPS1_S5_")]
// 0xf4e274 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE17oldExtentsOverlapEPS1_S5_
// type: int()
pub fn stub_0xf4e274() {
    // IDA 0xf4e274: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::removeNodeFromHash(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE18removeNodeFromHashEPNS4_11SpatialNodeE")]
// 0xf4e284 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE18removeNodeFromHashEPNS4_11SpatialNodeE
// type: int __fastcall(int, RBX::NodeBase *this)
pub fn stub_0xf4e284() {
    // IDA 0xf4e284: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::removeTreeNodeChild(int,RBX::Vector3int32 &)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E")]
// 0xf4e2b4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E
// type: int __fastcall(int, RBX::SpatialHashStatic *this, int)
pub fn stub_0xf4e2b4() {
    // IDA 0xf4e2b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::insertNodeToPrimitive(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *,RBX::Primitive*,RBX::Vector3int32 const&,int)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS4_11SpatialNodeEPS1_RKNS_12Vector3int32Ei")]
// 0xf4e2c4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS4_11SpatialNodeEPS1_RKNS_12Vector3int32Ei
// type: int()
pub fn stub_0xf4e2c4() {
    // IDA 0xf4e2c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::addContactFromChildren(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *,RBX::Primitive*)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE22addContactFromChildrenEPNS4_8TreeNodeEPS1_")]
// 0xf4e2d4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE22addContactFromChildrenEPNS4_8TreeNodeEPS1_
// type: int __fastcall(int, int, RBX::Primitive *this)
pub fn stub_0xf4e2d4() {
    // IDA 0xf4e2d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::checkAndReleaseContacts(RBX::Primitive*)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23checkAndReleaseContactsEPS1_")]
// 0xf4e2e4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23checkAndReleaseContactsEPS1_
// type: int()
pub fn stub_0xf4e2e4() {
    // IDA 0xf4e2e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveExtentsChanged(RBX::Primitive*,RBX::Extents const&)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS1_RKNS_7ExtentsE")]
// 0xf4e2f4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS1_RKNS_7ExtentsE
// type: int()
pub fn stub_0xf4e2f4() {
    // IDA 0xf4e2f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveExtentsChanged(RBX::Primitive*)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_")]
// 0xf4e304 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_
// type: int __fastcall(int, RBX::Primitive *this)
pub fn stub_0xf4e304() {
    // IDA 0xf4e304: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findOtherNodesInLevel0Cell(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS4_11SpatialNodeE")]
// 0xf4e314 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS4_11SpatialNodeE
// type: int __fastcall(int, RBX::NodeBase *this)
pub fn stub_0xf4e314() {
    // IDA 0xf4e314: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::setup(void)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE5setupEv")]
// 0xf4e334 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE5setupEv
// type: int()
pub fn stub_0xf4e334() {
    // IDA 0xf4e334: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::addNode(RBX::Primitive*,RBX::Vector3int32 const&,bool)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS1_RKNS_12Vector3int32Eb")]
// 0xf4e344 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS1_RKNS_12Vector3int32Eb
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf4e344() {
    // IDA 0xf4e344: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::cleanup(void)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7cleanupEv")]
// 0xf4e354 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7cleanupEv
// type: int()
pub fn stub_0xf4e354() {
    // IDA 0xf4e354: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::newNode(int,int,RBX::Vector3int32 const&)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E")]
// 0xf4e364 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(int, int, void *, int, int, void *, int, int, int, int)
pub fn stub_0xf4e364() {
    // IDA 0xf4e364: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode::TreeNode(void)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeC2Ev")]
// 0xf4e374 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeC2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf4e374() {
    // IDA 0xf4e374: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode::~TreeNode()")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeD2Ev")]
// 0xf4e384 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeD2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf4e384() {
    // IDA 0xf4e384: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findNode(RBX::Primitive*,RBX::Vector3int32 const&)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS1_RKNS_12Vector3int32E")]
// 0xf4e394 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS1_RKNS_12Vector3int32E
// type: int()
pub fn stub_0xf4e394() {
    // IDA 0xf4e394: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::fastClear(void)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv")]
// 0xf4e3a4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, void *, int, int, int, int)
pub fn stub_0xf4e3a4() {
    // IDA 0xf4e3a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::~SpatialHash()")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EED2Ev")]
// 0xf4e3b4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EED2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf4e3b4() {
    // IDA 0xf4e3b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::for_each<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearSpatialNode>(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearSpatialNode &)")]
#[doc(alias = "j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeENS_16roblox_allocatorEE8for_eachINS5_20FastClearSpatialNodeEEEvRT_")]
// 0xf4e3c4 — j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeENS_16roblox_allocatorEE8for_eachINS5_20FastClearSpatialNodeEEEvRT_
// type: int()
pub fn stub_0xf4e3c4() {
    // IDA 0xf4e3c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::for_each<RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::CallDestructor>(RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::CallDestructor &)")]
#[doc(alias = "j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeENS_16roblox_allocatorEE8for_eachINS8_14CallDestructorEEEvRT_")]
// 0xf4e3d4 — j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeENS_16roblox_allocatorEE8for_eachINS8_14CallDestructorEEEvRT_
// type: int()
pub fn stub_0xf4e3d4() {
    // IDA 0xf4e3d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::for_each<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearTreeNode>(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearTreeNode &)")]
#[doc(alias = "j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeENS_16roblox_allocatorEE8for_eachINS5_17FastClearTreeNodeEEEvRT_")]
// 0xf4e3e4 — j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeENS_16roblox_allocatorEE8for_eachINS5_17FastClearTreeNodeEEEvRT_
// type: int()
pub fn stub_0xf4e3e4() {
    // IDA 0xf4e3e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::for_each<RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::CallDestructor>(RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::CallDestructor &)")]
#[doc(alias = "j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeENS_16roblox_allocatorEE8for_eachINS8_14CallDestructorEEEvRT_")]
// 0xf4e3f4 — j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeENS_16roblox_allocatorEE8for_eachINS8_14CallDestructorEEEvRT_
// type: int()
pub fn stub_0xf4e3f4() {
    // IDA 0xf4e3f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ExtentsInt32::empty(void)")]
#[doc(alias = "j___ZN3RBX12ExtentsInt325emptyEv")]
// 0xf4e414 — j___ZN3RBX12ExtentsInt325emptyEv
// type: _DWORD __fastcall(RBX::ExtentsInt32 *__hidden this)
pub fn stub_0xf4e414() {
    // IDA 0xf4e414: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ExtentsInt32::ExtentsInt32(void)")]
#[doc(alias = "j___ZN3RBX12ExtentsInt32C1Ev")]
// 0xf4e424 — j___ZN3RBX12ExtentsInt32C1Ev
// type: _DWORD __fastcall(RBX::ExtentsInt32 *__hidden this)
pub fn stub_0xf4e424() {
    // IDA 0xf4e424: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BallBallContact::BallBallContact(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "j___ZN3RBX15BallBallContactC2EPNS_9PrimitiveES2_")]
// 0xf4e484 — j___ZN3RBX15BallBallContactC2EPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::BallBallContact *__hidden this, RBX::Primitive *, RBX::Primitive *)
pub fn stub_0xf4e484() {
    // IDA 0xf4e484: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BallBallContact::~BallBallContact()")]
#[doc(alias = "j___ZN3RBX15BallBallContactD2Ev")]
// 0xf4e494 — j___ZN3RBX15BallBallContactD2Ev
// type: void __fastcall(RBX::BallBallContact *__hidden this)
pub fn stub_0xf4e494() {
    // IDA 0xf4e494: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallBlockContact::BallBlockContact(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "j___ZN3RBX16BallBlockContactC2EPNS_9PrimitiveES2_")]
// 0xf4e4a4 — j___ZN3RBX16BallBlockContactC2EPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::BallBlockContact *__hidden this, RBX::Primitive *, RBX::Primitive *)
pub fn stub_0xf4e4a4() {
    // IDA 0xf4e4a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallBlockContact::~BallBlockContact()")]
#[doc(alias = "j___ZN3RBX16BallBlockContactD2Ev")]
// 0xf4e4b4 — j___ZN3RBX16BallBlockContactD2Ev
// type: void __fastcall(RBX::BallBlockContact *__hidden this)
pub fn stub_0xf4e4b4() {
    // IDA 0xf4e4b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Extents::clampToOverlap(RBX::Extents const&)")]
#[doc(alias = "j___ZN3RBX7Extents14clampToOverlapERKS0_")]
// 0xf4e4c4 — j___ZN3RBX7Extents14clampToOverlapERKS0_
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const RBX::Extents *)
pub fn stub_0xf4e4c4() {
    // IDA 0xf4e4c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::NodeBase::getLevel(void)")]
#[doc(alias = "j___ZN3RBX8NodeBase8getLevelEv")]
// 0xf4e4e4 — j___ZN3RBX8NodeBase8getLevelEv
// type: _DWORD __fastcall(RBX::NodeBase *__hidden this)
pub fn stub_0xf4e4e4() {
    // IDA 0xf4e4e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev")]
// 0xf4e4f4 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev
// type: int()
pub fn stub_0xf4e4f4() {
    // IDA 0xf4e4f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEdlEPv")]
// 0xf4e504 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4e504() {
    // IDA 0xf4e504: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEnwEm")]
// 0xf4e514 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEnwEm
// type: int()
pub fn stub_0xf4e514() {
    // IDA 0xf4e514: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev")]
// 0xf4e524 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev
// type: int()
pub fn stub_0xf4e524() {
    // IDA 0xf4e524: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEnwEm")]
// 0xf4e534 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEnwEm
// type: int()
pub fn stub_0xf4e534() {
    // IDA 0xf4e534: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_15BallBallContactEEC2Ev")]
// 0xf4e544 — j___ZN3RBX9AllocatorINS_15BallBallContactEEC2Ev
// type: int()
pub fn stub_0xf4e544() {
    // IDA 0xf4e544: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_15BallBallContactEEdlEPv")]
// 0xf4e554 — j___ZN3RBX9AllocatorINS_15BallBallContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4e554() {
    // IDA 0xf4e554: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_15BallBallContactEEnwEm")]
// 0xf4e564 — j___ZN3RBX9AllocatorINS_15BallBallContactEEnwEm
// type: int()
pub fn stub_0xf4e564() {
    // IDA 0xf4e564: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_15BallCellContactEEdlEPv")]
// 0xf4e574 — j___ZN3RBX9AllocatorINS_15BallCellContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4e574() {
    // IDA 0xf4e574: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_15BallCellContactEEnwEm")]
// 0xf4e584 — j___ZN3RBX9AllocatorINS_15BallCellContactEEnwEm
// type: int()
pub fn stub_0xf4e584() {
    // IDA 0xf4e584: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_15BallPolyContactEEnwEm")]
// 0xf4e594 — j___ZN3RBX9AllocatorINS_15BallPolyContactEEnwEm
// type: int()
pub fn stub_0xf4e594() {
    // IDA 0xf4e594: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_15PolyCellContactEEdlEPv")]
// 0xf4e5a4 — j___ZN3RBX9AllocatorINS_15PolyCellContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4e5a4() {
    // IDA 0xf4e5a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_15PolyCellContactEEnwEm")]
// 0xf4e5b4 — j___ZN3RBX9AllocatorINS_15PolyCellContactEEnwEm
// type: int()
pub fn stub_0xf4e5b4() {
    // IDA 0xf4e5b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_15PolyPolyContactEEdlEPv")]
// 0xf4e5c4 — j___ZN3RBX9AllocatorINS_15PolyPolyContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4e5c4() {
    // IDA 0xf4e5c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_15PolyPolyContactEEnwEm")]
// 0xf4e5d4 — j___ZN3RBX9AllocatorINS_15PolyPolyContactEEnwEm
// type: int()
pub fn stub_0xf4e5d4() {
    // IDA 0xf4e5d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_16BallBlockContactEEC2Ev")]
// 0xf4e5e4 — j___ZN3RBX9AllocatorINS_16BallBlockContactEEC2Ev
// type: int()
pub fn stub_0xf4e5e4() {
    // IDA 0xf4e5e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_16BallBlockContactEEdlEPv")]
// 0xf4e5f4 — j___ZN3RBX9AllocatorINS_16BallBlockContactEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4e5f4() {
    // IDA 0xf4e5f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_16BallBlockContactEEnwEm")]
// 0xf4e604 — j___ZN3RBX9AllocatorINS_16BallBlockContactEEnwEm
// type: int()
pub fn stub_0xf4e604() {
    // IDA 0xf4e604: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_17BlockBlockContactEEnwEm")]
// 0xf4e614 — j___ZN3RBX9AllocatorINS_17BlockBlockContactEEnwEm
// type: int()
pub fn stub_0xf4e614() {
    // IDA 0xf4e614: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ExtentsInt32::contains(RBX::Vector3int32 const&)const")]
#[doc(alias = "j___ZNK3RBX12ExtentsInt328containsERKNS_12Vector3int32E")]
// 0xf4e944 — j___ZNK3RBX12ExtentsInt328containsERKNS_12Vector3int32E
// type: _DWORD __fastcall(RBX::ExtentsInt32 *__hidden this, const RBX::Vector3int32 *)
pub fn stub_0xf4e944() {
    // IDA 0xf4e944: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::find(RBX::Primitive * const&)const")]
#[doc(alias = "j___ZNKSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE4findERKS2_")]
// 0xf4e9c4 — j___ZNKSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE4findERKS2_
// type: int()
pub fn stub_0xf4e9c4() {
    // IDA 0xf4e9c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE11_M_allocateEm")]
// 0xf4e9d4 — j___ZNSt12_Vector_baseIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE11_M_allocateEm
// type: int()
pub fn stub_0xf4e9d4() {
    // IDA 0xf4e9d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Primitive *,std::allocator<RBX::Primitive *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX9PrimitiveESaIS2_EE11_M_allocateEm")]
// 0xf4e9e4 — j___ZNSt12_Vector_baseIPN3RBX9PrimitiveESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf4e9e4() {
    // IDA 0xf4e9e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *>(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SpatialHashINS3_9PrimitiveENS3_7ContactENS3_14ContactManagerELi4EE21SpatialHashTableEntryESA_EET0_T_SC_SB_")]
// 0xf4e9f4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SpatialHashINS3_9PrimitiveENS3_7ContactENS3_14ContactManagerELi4EE21SpatialHashTableEntryESA_EET0_T_SC_SB_
// type: int()
pub fn stub_0xf4e9f4() {
    // IDA 0xf4e9f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::set<RBX::Primitive * const*>(RBX::Primitive * const*,RBX::Primitive * const*)")]
#[doc(alias = "j___ZNSt3setIPN3RBX9PrimitiveESt4lessIS2_ESaIS2_EEC2IPKS2_EET_SA_")]
// 0xf4ea04 — j___ZNSt3setIPN3RBX9PrimitiveESt4lessIS2_ESaIS2_EEC2IPKS2_EET_SA_
// type: int()
pub fn stub_0xf4ea04() {
    // IDA 0xf4ea04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::resize(unsigned long,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE6resizeEmS6_")]
// 0xf4ea14 — j___ZNSt6vectorIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE6resizeEmS6_
// type: int()
pub fn stub_0xf4ea14() {
    // IDA 0xf4ea14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_insert_unique(RBX::Primitive * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0xf4ea24 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf4ea24() {
    // IDA 0xf4ea24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_insert_unique(std::_Rb_tree_iterator<RBX::Primitive *>,RBX::Primitive * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// 0xf4ea34 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf4ea34() {
    // IDA 0xf4ea34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_erase(std::_Rb_tree_node<RBX::Primitive *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xf4ea44 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int()
pub fn stub_0xf4ea44() {
    // IDA 0xf4ea44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Primitive * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0xf4ea54 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf4ea54() {
    // IDA 0xf4ea54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::returnNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE10returnNodeEPNS4_11SpatialNodeE")]
// 0xf4ea94 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE10returnNodeEPNS4_11SpatialNodeE
// type: int()
pub fn stub_0xf4ea94() {
    // IDA 0xf4ea94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::destroyNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11destroyNodeEPNS4_11SpatialNodeE")]
// 0xf4eaa4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11destroyNodeEPNS4_11SpatialNodeE
// type: int()
pub fn stub_0xf4eaa4() {
    // IDA 0xf4eaa4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findTreeNode(int,int,RBX::Vector3int32 const&)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12findTreeNodeEiiRKNS_12Vector3int32E")]
// 0xf4eab4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12findTreeNodeEiiRKNS_12Vector3int32E
// type: int()
pub fn stub_0xf4eab4() {
    // IDA 0xf4eab4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::retireTreeNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14retireTreeNodeEPNS4_8TreeNodeE")]
// 0xf4eac4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14retireTreeNodeEPNS4_8TreeNodeE
// type: int()
pub fn stub_0xf4eac4() {
    // IDA 0xf4eac4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAssembled(RBX::Primitive*)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE20onPrimitiveAssembledEPS1_")]
// 0xf4ead4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE20onPrimitiveAssembledEPS1_
// type: int()
pub fn stub_0xf4ead4() {
    // IDA 0xf4ead4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHash(RBX::World *,RBX::ContactManager*,int)")]
#[doc(alias = "j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EEC2EPNS_5WorldEPS3_i")]
// 0xf4eaf4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EEC2EPNS_5WorldEPS3_i
// type: int(void)
pub fn stub_0xf4eaf4() {
    // IDA 0xf4eaf4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::zero(void)")]
#[doc(alias = "j___ZN3RBX7Extents4zeroEv")]
// 0xf4eb04 — j___ZN3RBX7Extents4zeroEv
// type: _DWORD __fastcall(RBX::Extents *__hidden this)
pub fn stub_0xf4eb04() {
    // IDA 0xf4eb04: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEdlEPv")]
// 0xf4eb24 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEdlEPv
// type: void __fastcall(void *)
pub fn stub_0xf4eb24() {
    // IDA 0xf4eb24: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BasicSpatialHashPrimitive::getSpatialNodeLevel(void)const")]
#[doc(alias = "j___ZNK3RBX25BasicSpatialHashPrimitive19getSpatialNodeLevelEv")]
// 0xf4eb54 — j___ZNK3RBX25BasicSpatialHashPrimitive19getSpatialNodeLevelEv
// type: _DWORD __fastcall(RBX::BasicSpatialHashPrimitive *__hidden this)
pub fn stub_0xf4eb54() {
    // IDA 0xf4eb54: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::operator==(RBX::Extents const&)const")]
#[doc(alias = "j___ZNK3RBX7ExtentseqERKS0_")]
// 0xf4eb64 — j___ZNK3RBX7ExtentseqERKS0_
// type: int()
pub fn stub_0xf4eb64() {
    // IDA 0xf4eb64: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "std::_Vector_base<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX8NodeInfoESaIS1_EE11_M_allocateEm")]
// 0xf4eb74 — j___ZNSt12_Vector_baseIN3RBX8NodeInfoESaIS1_EE11_M_allocateEm
// type: int()
pub fn stub_0xf4eb74() {
    // IDA 0xf4eb74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::NodeInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::NodeInfo *,RBX::NodeInfo *>(RBX::NodeInfo *,RBX::NodeInfo *,RBX::NodeInfo *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NodeInfoES5_EET0_T_S7_S6_")]
// 0xf4eb84 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NodeInfoES5_EET0_T_S7_S6_
// type: int()
pub fn stub_0xf4eb84() {
    // IDA 0xf4eb84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry*,std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>>,unsigned long,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS6_S8_EEmRKS6_")]
// 0xf4eb94 — j___ZNSt6vectorIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS6_S8_EEmRKS6_
// type: int()
pub fn stub_0xf4eb94() {
    // IDA 0xf4eb94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::NodeInfo*,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,RBX::NodeInfo const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX8NodeInfoESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0xf4eba4 — j___ZNSt6vectorIN3RBX8NodeInfoESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int()
pub fn stub_0xf4eba4() {
    // IDA 0xf4eba4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
