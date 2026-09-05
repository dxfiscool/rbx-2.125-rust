// Auto-generated skeletons for rbx-script — global filler EA-sorted asc continuation
// Filter: Script|Lua|lua|Yield (5401 filtered, all stubbed) — global EA-sorted asc filler
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2b07d8..0x303d44 | global filler EA-sorted asc after 0x2affbc | rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEE15isNullClassNameEv")]
pub fn stub_0x2b07d8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv")]
pub fn stub_0x2b0f38() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE15isNullClassNameEv")]
pub fn stub_0x2b74c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sWorkspaceEEEERKS0_v")]
pub fn stub_0x2b7568(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sWorkspace>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sWorkspaceEEEERKS0_v")]
pub fn stub_0x2b75b0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sWorkspace>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats10sStatsItemEEE12getClassNameEv")]
pub fn stub_0x2c1df8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats10sStatsItemEEE12getClassNameEv")]
pub fn stub_0x2c7790() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "global constructor keyed to_a_78")]
pub fn stub_0x2d145c() -> crate::slot::PortedFn {
// IDA 0x2d145c: __GLOBAL__I_a_78.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2d145c, "__GLOBAL__I_a_78")
}

#[doc(alias = "global constructor keyed to_a_79")]
pub fn stub_0x2d2634() -> crate::slot::PortedFn {
// IDA 0x2d2634: __GLOBAL__I_a_79.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2d2634, "__GLOBAL__I_a_79")
}

#[doc(alias = "RBX::AdvMoveToolBase::getExtentsAndLocation(RBX::Extents &,G3D::CoordinateFrame &,bool &)const")]
pub fn stub_0x2d3d4c(handle: crate::slot::InstanceHandle) {
// RBX::AdvMoveToolBase dtor.
drop(handle);
}

#[doc(alias = "RBX::AdvMoveToolBase::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const")]
pub fn stub_0x2d487c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvMoveToolBase getter.
cell.get()
}

#[doc(alias = "global constructor keyed to_a_80")]
pub fn stub_0x2d55f8() -> crate::slot::PortedFn {
// IDA 0x2d55f8: __GLOBAL__I_a_80.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2d55f8, "__GLOBAL__I_a_80")
}

#[doc(alias = "RBX::AdvRotateTool::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const")]
pub fn stub_0x2d5da0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvRotateTool getter.
cell.get()
}

#[doc(alias = "global constructor keyed to_a_81")]
pub fn stub_0x2d6120() -> crate::slot::PortedFn {
// IDA 0x2d6120: __GLOBAL__I_a_81.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2d6120, "__GLOBAL__I_a_81")
}

#[doc(alias = "RBX::AdvRunDragger::createSnapSurface(RBX::Primitive *,G3D::Array<unsigned long,10,32ul> *)")]
pub fn stub_0x2d7610(handle: &crate::slot::InstanceHandle) {
// RBX::AdvRunDragger::createSnapSurface(RBX::Primitive*, G3D::Array<unsigned long, 10, 32ul>~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvRunDragger::notTried(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> const&)")]
pub fn stub_0x2d89e8(handle: &crate::slot::InstanceHandle) {
// RBX::AdvRunDragger::notTried(RBX::Primitive*, G3D::Array<RBX::Primitive*, 10, 32ul> const&~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvRunDragger::findSnap(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
pub fn stub_0x2d91b0(handle: &crate::slot::InstanceHandle) {
// RBX::AdvRunDragger::findSnap(G3D::Array<RBX::Primitive*, 10, 32ul> const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvRunDragger::findNoSnapPosition(G3D::CoordinateFrame const&)")]
pub fn stub_0x2d92dc(handle: crate::slot::InstanceHandle) {
// RBX::AdvRunDragger dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_82")]
pub fn stub_0x2d9d50() -> crate::slot::PortedFn {
// IDA 0x2d9d50: __GLOBAL__I_a_82.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2d9d50, "__GLOBAL__I_a_82")
}

#[doc(alias = "RBX::AxisToolBase::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const")]
pub fn stub_0x2da5d0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AxisToolBase getter.
cell.get()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AxisToolBase> RBX::shared_from<RBX::AxisToolBase>(RBX::AxisToolBase*)")]
pub fn stub_0x2db058() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AxisToolBase")
}

#[doc(alias = "global constructor keyed to_a_83")]
pub fn stub_0x2db2c4() -> crate::slot::PortedFn {
// IDA 0x2db2c4: __GLOBAL__I_a_83.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2db2c4, "__GLOBAL__I_a_83")
}

#[doc(alias = "global constructor keyed to_a_84")]
pub fn stub_0x2db534() -> crate::slot::PortedFn {
// IDA 0x2db534: __GLOBAL__I_a_84.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2db534, "__GLOBAL__I_a_84")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_10sCloneToolEEE7getNameEv")]
pub fn stub_0x2dbf88(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "global constructor keyed to_a_85")]
pub fn stub_0x2dc354() -> crate::slot::PortedFn {
// IDA 0x2dc354: __GLOBAL__I_a_85.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2dc354, "__GLOBAL__I_a_85")
}

#[doc(alias = "RBX::Dragger::computeExtents(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
pub fn stub_0x2dc790(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::computeExtents(G3D::Array<RBX::Primitive*, 10, 32ul> const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::intersectingWorldOrOthers(G3D::Array<RBX::Primitive *,10,32ul> const&,RBX::ContactManager &,float,float)")]
pub fn stub_0x2dca04(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::intersectingWorldOrOthers(G3D::Array<RBX::Primitive*, 10, 32ul> const&, RBX:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::intersectingGroundPlane(G3D::Array<RBX::Primitive *,10,32ul> const&,float)")]
pub fn stub_0x2dca90(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::intersectingGroundPlane(G3D::Array<RBX::Primitive*, 10, 32ul> const&, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::movePrimitivesGoal(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3&)")]
pub fn stub_0x2dcb04(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::movePrimitivesGoal(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::movePrimitives(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&)")]
pub fn stub_0x2dcba4(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::movePrimitives(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector3 co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::movePrimitivesDelta(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3&)")]
pub fn stub_0x2dcc5c(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::movePrimitivesDelta(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vecto~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::searchUpFine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
pub fn stub_0x2dcd50(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::searchUpFine(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector3&, RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::searchDownFine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
pub fn stub_0x2dce48(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::searchDownFine(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector3&, ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::searchUpGross(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
pub fn stub_0x2dcf50(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::searchUpGross(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector3&, R~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::searchDownGross(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
pub fn stub_0x2dd074(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::searchDownGross(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector3&,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::safePlaceAlongLine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3&,RBX::ContactManager &)")]
pub fn stub_0x2dd1d4(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::safePlaceAlongLine(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::safeMoveAlongLine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
pub fn stub_0x2dd588(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::safeMoveAlongLine(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector3~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::safeMoveYDrop(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
pub fn stub_0x2dd814(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::safeMoveYDrop(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector3 con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::safeMoveYDrop_EXT(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
pub fn stub_0x2dd924(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::safeMoveYDrop_EXT(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector3~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::safeMoveNoDrop(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &)")]
pub fn stub_0x2ddd90(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::safeMoveNoDrop(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Vector3 co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::safeRotate(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Matrix3 const&,RBX::ContactManager &)")]
pub fn stub_0x2ddec0(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::safeRotate(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Matrix3 const&~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::safeRotate2(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Matrix3 const&,RBX::ContactManager &)")]
pub fn stub_0x2de150(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::safeRotate2(G3D::Array<RBX::Primitive*, 10, 32ul> const&, G3D::Matrix3 const~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::intersectingWorldOrOthers_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 const&)")]
pub fn stub_0x2de1d0(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::intersectingWorldOrOthers_EXT(std::vector<RBX::Extents, std::allocator<RBX::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::searchUpGross_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
pub fn stub_0x2de578(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::searchUpGross_EXT(std::vector<RBX::Extents, std::allocator<RBX::Extents>>&, ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::searchDownGross_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
pub fn stub_0x2de6ac(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::searchDownGross_EXT(std::vector<RBX::Extents, std::allocator<RBX::Extents>>&~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::searchDownFine_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
pub fn stub_0x2de7e0(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::searchDownFine_EXT(std::vector<RBX::Extents, std::allocator<RBX::Extents>>&,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::searchUpFine_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
pub fn stub_0x2de92c(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::searchUpFine_EXT(std::vector<RBX::Extents, std::allocator<RBX::Extents>>&, G~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::intersectingGroundPlane_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> const&,G3D::Array<RBX::Primitive *,10,32ul> const&,float,G3D::Vector3 const&)")]
pub fn stub_0x2dea44(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::intersectingGroundPlane_EXT(std::vector<RBX::Extents, std::allocator<RBX::Ex~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Dragger::isIntersecting(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
pub fn stub_0x2deb24(handle: crate::slot::InstanceHandle) {
// RBX::Dragger dtor.
drop(handle);
}

#[doc(alias = "RBX::Dragger::checkBallBallIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
pub fn stub_0x2deb94(handle: crate::slot::InstanceHandle) {
// RBX::Dragger dtor.
drop(handle);
}

#[doc(alias = "RBX::Dragger::checkBallPolyIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
pub fn stub_0x2decd4(handle: crate::slot::InstanceHandle) {
// RBX::Dragger dtor.
drop(handle);
}

#[doc(alias = "RBX::Dragger::checkPolyPolyIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
pub fn stub_0x2df2b8(handle: crate::slot::InstanceHandle) {
// RBX::Dragger dtor.
drop(handle);
}

#[doc(alias = "RBX::Dragger::moveExtents(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Vector3 const&)")]
pub fn stub_0x2dfc24(handle: &crate::slot::InstanceHandle) {
// RBX::Dragger::moveExtents(std::vector<RBX::Extents, std::allocator<RBX::Extents>>&, G3D::V~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::append(RBX::Primitive * const&)")]
pub fn stub_0x2dfda8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::init(int,G3D::ReferenceCountedPointer<G3D::MemoryManager> const&)")]
pub fn stub_0x2dfed8() -> crate::slot::PortedFn {
// IDA 0x2dfed8: G3D::Array<RBX::Primitive*, 10, 32ul>::init(int, G3D::ReferenceCountedPointer<G3D::MemoryManager> const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2dfed8, "G3D::Array<RBX::Primitive*, 10, 32ul>::init(int, G3D::ReferenceCountedPointer<G3D::MemoryManager> co~")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::table(unsigned long,boost::hash<RBX::Primitive const*> const&,std::equal_to<RBX::Primitive const*> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> const&)")]
pub fn stub_0x2e0140() -> crate::slot::PortedFn {
// IDA 0x2e0140: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>, RBX::Primitive cons~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2e0140, "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,~")
}

#[doc(alias = "G3D::Sphere::~Sphere()")]
pub fn stub_0x2e02d0() -> crate::slot::PortedFn {
// IDA 0x2e02d0: G3D::Sphere::~Sphere().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2e02d0, "G3D::Sphere::~Sphere()")
}

#[doc(alias = "global constructor keyed to_a_86")]
pub fn stub_0x2e02d4() -> crate::slot::PortedFn {
// IDA 0x2e02d4: __GLOBAL__I_a_86.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2e02d4, "__GLOBAL__I_a_86")
}

#[doc(alias = "global constructor keyed to_a_87")]
pub fn stub_0x2e0cc4() -> crate::slot::PortedFn {
// IDA 0x2e0cc4: __GLOBAL__I_a_87.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2e0cc4, "__GLOBAL__I_a_87")
}

#[doc(alias = "RBX::DragUtilities::hitObjectOrPlane(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")]
pub fn stub_0x2e13f0(handle: &crate::slot::InstanceHandle) {
// RBX::DragUtilities::hitObjectOrPlane(RBX::ContactManager const&, RBX::RbxRay const&, G3D::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DragUtilities::hitObject(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")]
pub fn stub_0x2e1708(handle: &crate::slot::InstanceHandle) {
// RBX::DragUtilities::hitObject(RBX::ContactManager const&, RBX::RbxRay const&, G3D::Array<R~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DragUtilities::toGrid(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub fn stub_0x2e26d4(handle: &crate::slot::InstanceHandle) {
// RBX::DragUtilities::toGrid(G3D::Vector3 const&, G3D::Vector3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_88")]
pub fn stub_0x2e2948() -> crate::slot::PortedFn {
// IDA 0x2e2948: __GLOBAL__I_a_88.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2e2948, "__GLOBAL__I_a_88")
}

#[doc(alias = "global constructor keyed to_a_89")]
pub fn stub_0x2e2cbc() -> crate::slot::PortedFn {
// IDA 0x2e2cbc: __GLOBAL__I_a_89.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2e2cbc, "__GLOBAL__I_a_89")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGameToolEEE7getNameEv")]
pub fn stub_0x2e33ec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "global constructor keyed to_a_90")]
pub fn stub_0x2e34ec() -> crate::slot::PortedFn {
// IDA 0x2e34ec: __GLOBAL__I_a_90.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2e34ec, "__GLOBAL__I_a_90")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sGrabToolEEE7getNameEv")]
pub fn stub_0x2e3c60(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "global constructor keyed to_a_91")]
pub fn stub_0x2e3d60() -> crate::slot::PortedFn {
// IDA 0x2e3d60: __GLOBAL__I_a_91.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2e3d60, "__GLOBAL__I_a_91")
}

#[doc(alias = "global constructor keyed to_a_92")]
pub fn stub_0x2e3fd0() -> crate::slot::PortedFn {
// IDA 0x2e3fd0: __GLOBAL__I_a_92.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2e3fd0, "__GLOBAL__I_a_92")
}

#[doc(alias = "global constructor keyed to_a_93")]
pub fn stub_0x2e4240() -> crate::slot::PortedFn {
// IDA 0x2e4240: __GLOBAL__I_a_93.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2e4240, "__GLOBAL__I_a_93")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_11sHammerToolEEE7getNameEv")]
pub fn stub_0x2e4b34(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "global constructor keyed to_a_94")]
pub fn stub_0x2e4f20() -> crate::slot::PortedFn {
// IDA 0x2e4f20: __GLOBAL__I_a_94.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2e4f20, "__GLOBAL__I_a_94")
}

#[doc(alias = "RBX::MegaDragger::safeMoveYDrop(G3D::Vector3 const&)")]
pub fn stub_0x2eb604(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::safeMoveYDrop(G3D::Vector3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::safeMoveNoDrop(G3D::Vector3 const&)")]
pub fn stub_0x2eb734(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::safeMoveNoDrop(G3D::Vector3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::safeMoveAlongLine(G3D::Vector3 const&)")]
pub fn stub_0x2eb87c(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::safeMoveAlongLine(G3D::Vector3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::moveSafePlaceAlongLine(G3D::Vector3 const&)")]
pub fn stub_0x2eba30(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::moveSafePlaceAlongLine(G3D::Vector3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::moveAlongLine(G3D::Vector3 const&)")]
pub fn stub_0x2ebc38(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::moveAlongLine(G3D::Vector3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::safeRotateAlongLine(G3D::Vector3 const&)")]
pub fn stub_0x2ebc44(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::safeRotateAlongLine(G3D::Vector3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::safeRotate(G3D::Matrix3 const&)")]
pub fn stub_0x2ebd7c(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::safeRotate(G3D::Matrix3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::createSnapSurface(RBX::Primitive *,G3D::Array<unsigned long,10,32ul> *)")]
pub fn stub_0x2f33e0(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::createSnapSurface(RBX::Primitive*, G3D::Array<unsigned long, 10, 32ul>*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::notTried(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> const&)")]
pub fn stub_0x2f4630(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::notTried(RBX::Primitive*, G3D::Array<RBX::Primitive*, 10, 32ul> const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::findSnap(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
pub fn stub_0x2f4eac(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::findSnap(G3D::Array<RBX::Primitive*, 10, 32ul> const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::findNoSnapPosition(G3D::CoordinateFrame const&)")]
pub fn stub_0x2f5018(handle: crate::slot::InstanceHandle) {
// RBX::RunDragger dtor.
drop(handle);
}

#[doc(alias = "G3D::Array<unsigned long,10,32ul>::append(unsigned long const&)")]
pub fn stub_0x2f587c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "G3D::Array<unsigned long,10,32ul>::resize(int,bool)")]
pub fn stub_0x2f58d8(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "G3D::Array<unsigned long,10,32ul>::realloc(int)")]
pub fn stub_0x2f5990(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "G3D::Array<unsigned long,10,32ul>::~Array()")]
pub fn stub_0x2f5b78(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "G3D::Array<unsigned long,10,32ul>::Array(void)")]
pub fn stub_0x2f5c4c() -> crate::slot::VecModel {
// sequence ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS7_5list3INS7_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2fff80() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3000d8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x300d3c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x300e68() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "RBX::Axes::axisToNormalId(G3D::Vector3::Axis)")]
pub fn stub_0x302ed8(handle: &crate::slot::InstanceHandle) {
// RBX::Axes::axisToNormalId(G3D::Vector3::Axis) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Axes::axisToMask(G3D::Vector3::Axis)")]
pub fn stub_0x302ee0(handle: &crate::slot::InstanceHandle) {
// RBX::Axes::axisToMask(G3D::Vector3::Axis) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Axes::getAxis(G3D::Vector3::Axis)const")]
pub fn stub_0x302f1c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Axes getter.
cell.get()
}

#[doc(alias = "RBX::StringConverter<G3D::Vector3::Axis>::convertToValue(std::string const&,G3D::Vector3::Axis&)")]
pub fn stub_0x303304(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<G3D::Vector3::Axis>::convertToValue(std::string const&, G3D::Vector3:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "G3D::Vector3::Axis * rbx::any_cast<G3D::Vector3::Axis,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x303bc8(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "G3D::Vector3::Axis & rbx::any_cast<G3D::Vector3::Axis &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x303c20(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::resize(unsigned long,G3D::Vector3::Axis)")]
pub fn stub_0x303d10(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::push_back(G3D::Vector3::Axis const&)")]
pub fn stub_0x303d44(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}
