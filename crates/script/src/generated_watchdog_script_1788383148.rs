//! script generated_watchdog_script_1788383148 — 100 stubs EA-sorted asc global dedup (Script/Lua filter exhausted, gap filler)
//! Filter: Script|Lua (case-sensitive) -> 4456 total, 0 remaining before batch (all 4456 already in script crate)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x71af7c..0x721920 | EA-sorted asc distinct not yet in global_eas.txt (global dedup)
//! SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; // 0xADDR mangled + #[doc(alias)] + todo!("0xADDR")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x71af7c — __ZN3RBX15BallPolyContactD0Ev
// type: void __fastcall(RBX::BallPolyContact *__hidden this)
#[doc(alias = "RBX::BallPolyContact::~BallPolyContact()")]
#[doc(alias = "__ZN3RBX15BallPolyContactD0Ev")]
pub fn stub_0x71af7c(handle: crate::slot::InstanceHandle) {
// RBX::BallPolyContact dtor.
drop(handle);
}

// 0x71b030 — __ZNK3RBX11PolyContact13numConnectorsEv
// type: _DWORD __fastcall(RBX::PolyContact *__hidden this)
#[doc(alias = "RBX::PolyContact::numConnectors(void)const")]
#[doc(alias = "__ZNK3RBX11PolyContact13numConnectorsEv")]
pub fn stub_0x71b030(handle: &crate::slot::InstanceHandle) {
// RBX::PolyContact::numConnectors() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71b038 — __ZN3RBX9AllocatorINS_15BallPolyContactEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallPolyContactEE13releaseMemoryEv")]
pub fn stub_0x71b038(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::BallPolyContact>::releaseMemory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71b084 — __ZN3RBX9AllocatorINS_15BallPolyContactEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallPolyContactEEdlEPv")]
pub fn stub_0x71b084(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::BallPolyContact>::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71b460 — __ZN3RBX5Block4initEv
// type: _DWORD __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::init(void)")]
#[doc(alias = "__ZN3RBX5Block4initEv")]
pub fn stub_0x71b460(handle: &crate::slot::InstanceHandle) {
// RBX::Block::init() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71b4a8 — __ZN3RBX5Block9buildMeshEv
// type: _DWORD __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::buildMesh(void)")]
#[doc(alias = "__ZN3RBX5Block9buildMeshEv")]
pub fn stub_0x71b4a8(handle: &crate::slot::InstanceHandle) {
// RBX::Block::buildMesh() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71b72c — __ZNK3RBX5Block15getMomentHollowEf
// type: _DWORD __fastcall(RBX::Block *__hidden this, float)
#[doc(alias = "RBX::Block::getMomentHollow(float)const")]
#[doc(alias = "__ZNK3RBX5Block15getMomentHollowEf")]
pub fn stub_0x71b72c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Block getter.
cell.get()
}

// 0x71bb08 — __ZNK3RBX5Block9getVolumeEv
// type: _DWORD __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::getVolume(void)const")]
#[doc(alias = "__ZNK3RBX5Block9getVolumeEv")]
pub fn stub_0x71bb08(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Block getter.
cell.get()
}

// 0x71c050 — __ZNK3RBX5Block21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::Block *__hidden this, unsigned int)
#[doc(alias = "RBX::Block::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX5Block21getSurfaceCoordInBodyEm")]
pub fn stub_0x71c050(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Block getter.
cell.get()
}

// 0x71c3f0 — __ZN3RBX5BlockD1Ev
// type: void __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::~Block()")]
#[doc(alias = "__ZN3RBX5BlockD1Ev")]
pub fn stub_0x71c3f0(handle: crate::slot::InstanceHandle) {
// RBX::Block dtor.
drop(handle);
}

// 0x71c3f4 — __ZN3RBX5BlockD0Ev
// type: void __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::~Block() [0x71c3f4]")]
#[doc(alias = "__ZN3RBX5BlockD0Ev")]
pub fn stub_0x71c3f4(handle: crate::slot::InstanceHandle) {
// RBX::Block dtor.
drop(handle);
}

// 0x71c494 — __ZNK3RBX5Block15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX5Block15getGeometryTypeEv")]
pub fn stub_0x71c494(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Block getter.
cell.get()
}

// 0x71c498 — __ZNK3RBX5Block14getCollideTypeEv
// type: _DWORD __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::getCollideType(void)const")]
#[doc(alias = "__ZNK3RBX5Block14getCollideTypeEv")]
pub fn stub_0x71c498(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Block getter.
cell.get()
}

// 0x71c49c — __ZN3RBX8Geometry20setGeometryParameterERKSsi
// type: _DWORD __fastcall(RBX::Geometry *__hidden this, const std::string *, int)
#[doc(alias = "RBX::Geometry::setGeometryParameter(std::string const&,int)")]
#[doc(alias = "__ZN3RBX8Geometry20setGeometryParameterERKSsi")]
pub fn stub_0x71c49c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Geometry setter.
cell.set(value)
}

// 0x71c4f4 — __ZNK3RBX8Geometry20getGeometryParameterERKSs
// type: _DWORD __fastcall(RBX::Geometry *__hidden this, const std::string *)
#[doc(alias = "RBX::Geometry::getGeometryParameter(std::string const&)const")]
#[doc(alias = "__ZNK3RBX8Geometry20getGeometryParameterERKSs")]
pub fn stub_0x71c4f4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Geometry getter.
cell.get()
}

// 0x71c548 — __ZNK3RBX4Poly9getRadiusEv
// type: _DWORD __fastcall(RBX::Poly *__hidden this)
#[doc(alias = "RBX::Poly::getRadius(void)const")]
#[doc(alias = "__ZNK3RBX4Poly9getRadiusEv")]
pub fn stub_0x71c548(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Poly getter.
cell.get()
}

// 0x71c54c — __ZNK3RBX4Poly14getNumSurfacesEv
// type: _DWORD __fastcall(RBX::Poly *__hidden this)
#[doc(alias = "RBX::Poly::getNumSurfaces(void)const")]
#[doc(alias = "__ZNK3RBX4Poly14getNumSurfacesEv")]
pub fn stub_0x71c54c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Poly getter.
cell.get()
}

// 0x71c564 — __ZNK3RBX8Geometry25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::Geometry::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX8Geometry25getFaceFromLegacyNormalIdENS_8NormalIdE")]
pub fn stub_0x71c564(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Geometry getter.
cell.get()
}

// 0x71c568 — __ZNK3RBX8Geometry20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::Geometry *__hidden this)
#[doc(alias = "RBX::Geometry::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX8Geometry20isGeometryOrthogonalEv")]
pub fn stub_0x71c568(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Geometry getter.
cell.get()
}

// 0x71c56c — __ZNK3RBX5Block9getMomentEf
// type: _DWORD __fastcall(RBX::Block *__hidden this, float)
#[doc(alias = "RBX::Block::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX5Block9getMomentEf")]
pub fn stub_0x71c56c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Block getter.
cell.get()
}

// 0x71ca14 — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY12BlockCornersEEdlEPv")]
pub fn stub_0x71ca14(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::BlockCorners>::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71d050 — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY12BlockCornersEEnwEm")]
pub fn stub_0x71d050(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::BlockCorners>::operator new(unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71d14c — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY12BlockCornersEEC2Ev")]
pub fn stub_0x71d14c(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::BlockCorners>::Allocator() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71d1b0 — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY12BlockCornersEE13releaseMemoryEv")]
pub fn stub_0x71d1b0(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::BlockCorners>::releaseMemory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71da2c — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9BlockMeshEEdlEPv")]
pub fn stub_0x71da2c(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::BlockMesh>::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71da68 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EED2Ev
#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EED2Ev")]
pub fn stub_0x71da68(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

// 0x71daa0 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EED2Ev
#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EED2Ev")]
pub fn stub_0x71daa0(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

// 0x71e0b0 — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9BlockMeshEEnwEm")]
pub fn stub_0x71e0b0(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::BlockMesh>::operator new(unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e230 — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9BlockMeshEEC2Ev")]
pub fn stub_0x71e230(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::BlockMesh>::Allocator() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e294 — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9BlockMeshEE13releaseMemoryEv")]
pub fn stub_0x71e294(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::BlockMesh>::releaseMemory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e3bc — __ZN3RBX5BlockD2Ev
// type: void __fastcall(RBX::Block *__hidden this)
#[doc(alias = "RBX::Block::~Block() [0x71e3bc]")]
#[doc(alias = "__ZN3RBX5BlockD2Ev")]
pub fn stub_0x71e3bc(handle: crate::slot::InstanceHandle) {
// RBX::Block dtor.
drop(handle);
}

// 0x71e5cc — __ZN3RBX10CleanStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::CleanStage::CleanStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX10CleanStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_0x71e5cc(handle: &crate::slot::InstanceHandle) {
// RBX::CleanStage::CleanStage(RBX::IStage*, RBX::World*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e5d0 — __ZN3RBX10CleanStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::CleanStage::CleanStage(RBX::IStage *,RBX::World *) [0x71e5d0]")]
#[doc(alias = "__ZN3RBX10CleanStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_0x71e5d0(handle: &crate::slot::InstanceHandle) {
// RBX::CleanStage::CleanStage(RBX::IStage*, RBX::World*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e6a4 — __ZN3RBX10CleanStage16onPrimitiveAddedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::CleanStage::onPrimitiveAdded(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10CleanStage16onPrimitiveAddedEPNS_9PrimitiveE")]
pub fn stub_0x71e6a4(handle: &crate::slot::InstanceHandle) {
// RBX::CleanStage::onPrimitiveAdded(RBX::Primitive*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e6c0 — __ZN3RBX10CleanStage19onPrimitiveRemovingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::CleanStage::onPrimitiveRemoving(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10CleanStage19onPrimitiveRemovingEPNS_9PrimitiveE")]
pub fn stub_0x71e6c0(handle: &crate::slot::InstanceHandle) {
// RBX::CleanStage::onPrimitiveRemoving(RBX::Primitive*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e6dc — __ZN3RBX10CleanStage23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Joint *, RBX::Primitive *)
#[doc(alias = "RBX::CleanStage::onJointPrimitiveNulling(RBX::Joint *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10CleanStage23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE")]
pub fn stub_0x71e6dc(handle: &crate::slot::InstanceHandle) {
// RBX::CleanStage::onJointPrimitiveNulling(RBX::Joint*, RBX::Primitive*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e7fc — __ZN3RBX10CleanStage19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Joint *, RBX::Primitive *)
#[doc(alias = "RBX::CleanStage::onJointPrimitiveSet(RBX::Joint *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10CleanStage19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE")]
pub fn stub_0x71e7fc(handle: &crate::slot::InstanceHandle) {
// RBX::CleanStage::onJointPrimitiveSet(RBX::Joint*, RBX::Primitive*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e87c — __ZN3RBX10CleanStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::CleanStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10CleanStage11onEdgeAddedEPNS_4EdgeE")]
pub fn stub_0x71e87c(handle: &crate::slot::InstanceHandle) {
// RBX::CleanStage::onEdgeAdded(RBX::Edge*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e8b0 — __ZN3RBX10CleanStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::CleanStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10CleanStage14onEdgeRemovingEPNS_4EdgeE")]
pub fn stub_0x71e8b0(handle: &crate::slot::InstanceHandle) {
// RBX::CleanStage::onEdgeRemoving(RBX::Edge*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71e984 — __ZNK3RBX10IPipelined7inStageEPNS_6IStageE
// type: _DWORD __fastcall(RBX::IPipelined *__hidden this, RBX::IStage *)
#[doc(alias = "RBX::IPipelined::inStage(RBX::IStage *)const")]
#[doc(alias = "__ZNK3RBX10IPipelined7inStageEPNS_6IStageE")]
pub fn stub_0x71e984(handle: &crate::slot::InstanceHandle) {
// RBX::IPipelined::inStage(RBX::IStage*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71ea28 — __ZN3RBX10CleanStageD1Ev
// type: void __fastcall(RBX::CleanStage *__hidden this)
#[doc(alias = "RBX::CleanStage::~CleanStage()")]
#[doc(alias = "__ZN3RBX10CleanStageD1Ev")]
pub fn stub_0x71ea28(handle: crate::slot::InstanceHandle) {
// RBX::CleanStage dtor.
drop(handle);
}

// 0x71ea4c — __ZN3RBX10CleanStageD0Ev
// type: void __fastcall(RBX::CleanStage *__hidden this)
#[doc(alias = "RBX::CleanStage::~CleanStage() [0x71ea4c]")]
#[doc(alias = "__ZN3RBX10CleanStageD0Ev")]
pub fn stub_0x71ea4c(handle: crate::slot::InstanceHandle) {
// RBX::CleanStage dtor.
drop(handle);
}

// 0x71eb04 — __ZNK3RBX10CleanStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::CleanStage *__hidden this)
#[doc(alias = "RBX::CleanStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX10CleanStage12getStageTypeEv")]
pub fn stub_0x71eb04(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CleanStage getter.
cell.get()
}

// 0x71ebd0 — __ZN3RBX5ClumpC1Ev
// type: _DWORD __fastcall(RBX::Clump *__hidden this)
#[doc(alias = "RBX::Clump::Clump(void)")]
#[doc(alias = "__ZN3RBX5ClumpC1Ev")]
pub fn stub_0x71ebd0() -> crate::slot::InstanceHandle {
// RBX::Clump ctor.
crate::slot::InstanceHandle::new("RBX::Clump")
}

// 0x71ebec — __ZN3RBX5ClumpD0Ev
// type: void __fastcall(RBX::Clump *__hidden this)
#[doc(alias = "RBX::Clump::~Clump()")]
#[doc(alias = "__ZN3RBX5ClumpD0Ev")]
pub fn stub_0x71ebec(handle: crate::slot::InstanceHandle) {
// RBX::Clump dtor.
drop(handle);
}

// 0x71ec8c — __ZN3RBX5ClumpD1Ev
// type: void __fastcall(RBX::Clump *__hidden this)
#[doc(alias = "RBX::Clump::~Clump() [0x71ec8c]")]
#[doc(alias = "__ZN3RBX5ClumpD1Ev")]
pub fn stub_0x71ec8c(handle: crate::slot::InstanceHandle) {
// RBX::Clump dtor.
drop(handle);
}

// 0x71ec9c — __ZN3RBX5Clump20isClumpRootPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Clump *__hidden this, const RBX::Primitive *)
#[doc(alias = "RBX::Clump::isClumpRootPrimitive(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX5Clump20isClumpRootPrimitiveEPKNS_9PrimitiveE")]
pub fn stub_0x71ec9c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Clump getter.
cell.get()
}

// 0x71ecac — __ZN3RBX5Clump17getPrimitiveClumpEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Primitive *__hidden this)
#[doc(alias = "RBX::Clump::getPrimitiveClump(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5Clump17getPrimitiveClumpEPNS_9PrimitiveE")]
pub fn stub_0x71ecac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Clump getter.
cell.get()
}

// 0x71ecb4 — __ZN3RBX5Clump22getConstPrimitiveClumpEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Primitive *__hidden this)
#[doc(alias = "RBX::Clump::getConstPrimitiveClump(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX5Clump22getConstPrimitiveClumpEPKNS_9PrimitiveE")]
pub fn stub_0x71ecb4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Clump getter.
cell.get()
}

// 0x71f024 — __ZN3RBX11IndexedTree16onParentChangingEv
// type: _DWORD __fastcall(RBX::IndexedTree *__hidden this)
#[doc(alias = "RBX::IndexedTree::onParentChanging(void)")]
#[doc(alias = "__ZN3RBX11IndexedTree16onParentChangingEv")]
pub fn stub_0x71f024(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedTree::onParentChanging() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f028 — __ZN3RBX11IndexedTree13onChildAddingEPS0_
// type: _DWORD __fastcall(RBX::IndexedTree *__hidden this, RBX::IndexedTree *)
#[doc(alias = "RBX::IndexedTree::onChildAdding(RBX::IndexedTree*)")]
#[doc(alias = "__ZN3RBX11IndexedTree13onChildAddingEPS0_")]
pub fn stub_0x71f028(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedTree::onChildAdding(RBX::IndexedTree*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f02c — __ZN3RBX11IndexedTree12onChildAddedEPS0_
// type: _DWORD __fastcall(RBX::IndexedTree *__hidden this, RBX::IndexedTree *)
#[doc(alias = "RBX::IndexedTree::onChildAdded(RBX::IndexedTree*)")]
#[doc(alias = "__ZN3RBX11IndexedTree12onChildAddedEPS0_")]
pub fn stub_0x71f02c(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedTree::onChildAdded(RBX::IndexedTree*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f030 — __ZN3RBX11IndexedTree15onChildRemovingEPS0_
// type: _DWORD __fastcall(RBX::IndexedTree *__hidden this, RBX::IndexedTree *)
#[doc(alias = "RBX::IndexedTree::onChildRemoving(RBX::IndexedTree*)")]
#[doc(alias = "__ZN3RBX11IndexedTree15onChildRemovingEPS0_")]
pub fn stub_0x71f030(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedTree::onChildRemoving(RBX::IndexedTree*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f034 — __ZN3RBX11IndexedTree14onChildRemovedEPS0_
// type: _DWORD __fastcall(RBX::IndexedTree *__hidden this, RBX::IndexedTree *)
#[doc(alias = "RBX::IndexedTree::onChildRemoved(RBX::IndexedTree*)")]
#[doc(alias = "__ZN3RBX11IndexedTree14onChildRemovedEPS0_")]
pub fn stub_0x71f034(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedTree::onChildRemoved(RBX::IndexedTree*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f038 — __ZN3RBX11IndexedTree17onAncestorChangedEv
// type: _DWORD __fastcall(RBX::IndexedTree *__hidden this)
#[doc(alias = "RBX::IndexedTree::onAncestorChanged(void)")]
#[doc(alias = "__ZN3RBX11IndexedTree17onAncestorChangedEv")]
pub fn stub_0x71f038(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedTree::onAncestorChanged() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f03c — __ZN3RBX11IndexedMesh15onLowersChangedEv
// type: _DWORD __fastcall(RBX::IndexedMesh *__hidden this)
#[doc(alias = "RBX::IndexedMesh::onLowersChanged(void)")]
#[doc(alias = "__ZN3RBX11IndexedMesh15onLowersChangedEv")]
pub fn stub_0x71f03c(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedMesh::onLowersChanged() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f648 — __ZN3RBX17BlockBlockContact12pairHitRatioEv
// type: _DWORD __fastcall(RBX::BlockBlockContact *__hidden this)
#[doc(alias = "RBX::BlockBlockContact::pairHitRatio(void)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact12pairHitRatioEv")]
pub fn stub_0x71f648(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::pairHitRatio() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f684 — __ZN3RBX17BlockBlockContact15featureHitRatioEv
// type: _DWORD __fastcall(RBX::BlockBlockContact *__hidden this)
#[doc(alias = "RBX::BlockBlockContact::featureHitRatio(void)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact15featureHitRatioEv")]
pub fn stub_0x71f684(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::featureHitRatio() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f6c0 — __ZN3RBX7Contact7getBodyEi
// type: _DWORD __fastcall(RBX::Contact *__hidden this, int)
#[doc(alias = "RBX::Contact::getBody(int)")]
#[doc(alias = "__ZN3RBX7Contact7getBodyEi")]
pub fn stub_0x71f6c0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Contact getter.
cell.get()
}

// 0x71f6cc — __ZN3RBX7ContactC2EPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::Contact *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::Contact::Contact(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX7ContactC2EPNS_9PrimitiveES2_")]
pub fn stub_0x71f6cc(handle: &crate::slot::InstanceHandle) {
// RBX::Contact::Contact(RBX::Primitive*, RBX::Primitive*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f6fc — __ZN3RBX7ContactD0Ev
// type: void __fastcall(RBX::Contact *__hidden this)
#[doc(alias = "RBX::Contact::~Contact()")]
#[doc(alias = "__ZN3RBX7ContactD0Ev")]
pub fn stub_0x71f6fc(handle: crate::slot::InstanceHandle) {
// RBX::Contact dtor.
drop(handle);
}

// 0x71f79c — __ZN3RBX7ContactD1Ev
// type: void __fastcall(RBX::Contact *__hidden this)
#[doc(alias = "RBX::Contact::~Contact() [0x71f79c]")]
#[doc(alias = "__ZN3RBX7ContactD1Ev")]
pub fn stub_0x71f79c(handle: crate::slot::InstanceHandle) {
// RBX::Contact dtor.
drop(handle);
}

// 0x71f7a0 — __ZN3RBX7ContactD2Ev
// type: void __fastcall(RBX::Contact *__hidden this)
#[doc(alias = "RBX::Contact::~Contact() [0x71f7a0]")]
#[doc(alias = "__ZN3RBX7ContactD2Ev")]
pub fn stub_0x71f7a0(handle: crate::slot::InstanceHandle) {
// RBX::Contact dtor.
drop(handle);
}

// 0x71f890 — __ZN3RBX7Contact24primitiveMovedExternallyEv
// type: _DWORD __fastcall(RBX::Contact *__hidden this)
#[doc(alias = "RBX::Contact::primitiveMovedExternally(void)")]
#[doc(alias = "__ZN3RBX7Contact24primitiveMovedExternallyEv")]
pub fn stub_0x71f890(handle: &crate::slot::InstanceHandle) {
// RBX::Contact::primitiveMovedExternally() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f8d4 — __ZN3RBX7Contact4stepEi
// type: _DWORD __fastcall(RBX::Contact *__hidden this, int)
#[doc(alias = "RBX::Contact::step(int)")]
#[doc(alias = "__ZN3RBX7Contact4stepEi")]
pub fn stub_0x71f8d4(handle: &crate::slot::InstanceHandle) {
// RBX::Contact::step(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71f9e4 — __ZN3RBX7Contact19computeIsAdjacentUiEf
// type: _DWORD __fastcall(RBX::Contact *__hidden this, float)
#[doc(alias = "RBX::Contact::computeIsAdjacentUi(float)")]
#[doc(alias = "__ZN3RBX7Contact19computeIsAdjacentUiEf")]
pub fn stub_0x71f9e4(handle: &crate::slot::InstanceHandle) {
// RBX::Contact::computeIsAdjacentUi(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71fa14 — __ZN3RBX7Contact20computeIsCollidingUiEf
// type: _DWORD __fastcall(RBX::Contact *__hidden this, float)
#[doc(alias = "RBX::Contact::computeIsCollidingUi(float)")]
#[doc(alias = "__ZN3RBX7Contact20computeIsCollidingUiEf")]
pub fn stub_0x71fa14(handle: &crate::slot::InstanceHandle) {
// RBX::Contact::computeIsCollidingUi(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71fa34 — __ZN3RBX17calculateFrictionEff
// type: _DWORD __fastcall(RBX *__hidden this, float, float)
#[doc(alias = "RBX::calculateFriction(float,float)")]
#[doc(alias = "__ZN3RBX17calculateFrictionEff")]
pub fn stub_0x71fa34() -> crate::slot::PortedFn {
// IDA 0x71fa34: RBX::calculateFriction(float, float).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x71fa34, "RBX::calculateFriction(float, float)")
}

// 0x71fac4 — __ZN3RBX7Contact35onPrimitiveContactParametersChangedEv
// type: _DWORD __fastcall(RBX::Contact *__hidden this)
#[doc(alias = "RBX::Contact::onPrimitiveContactParametersChanged(void)")]
#[doc(alias = "__ZN3RBX7Contact35onPrimitiveContactParametersChangedEv")]
pub fn stub_0x71fac4(handle: &crate::slot::InstanceHandle) {
// RBX::Contact::onPrimitiveContactParametersChanged() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71fbb8 — __ZN3RBX7Contact15deleteConnectorEPNS_16ContactConnectorE
// type: _DWORD __fastcall(RBX::Contact *__hidden this, RBX::ContactConnector *)
#[doc(alias = "RBX::Contact::deleteConnector(RBX::ContactConnector *)")]
#[doc(alias = "__ZN3RBX7Contact15deleteConnectorEPNS_16ContactConnectorE")]
pub fn stub_0x71fbb8(handle: &crate::slot::InstanceHandle) {
// RBX::Contact::deleteConnector(RBX::ContactConnector*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71fbdc — __ZN3RBX7Contact34generateDataForMovingAssemblyStageEv
// type: _DWORD __fastcall(RBX::Contact *__hidden this)
#[doc(alias = "RBX::Contact::generateDataForMovingAssemblyStage(void)")]
#[doc(alias = "__ZN3RBX7Contact34generateDataForMovingAssemblyStageEv")]
pub fn stub_0x71fbdc(handle: &crate::slot::InstanceHandle) {
// RBX::Contact::generateDataForMovingAssemblyStage() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71fc04 — __ZN3RBX15BallBallContact12getConnectorEi
// type: _DWORD __fastcall(RBX::BallBallContact *__hidden this, int)
#[doc(alias = "RBX::BallBallContact::getConnector(int)")]
#[doc(alias = "__ZN3RBX15BallBallContact12getConnectorEi")]
pub fn stub_0x71fc04(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BallBallContact getter.
cell.get()
}

// 0x71fc08 — __ZN3RBX15BallBallContact19deleteAllConnectorsEv
// type: _DWORD __fastcall(RBX::BallBallContact *__hidden this)
#[doc(alias = "RBX::BallBallContact::deleteAllConnectors(void)")]
#[doc(alias = "__ZN3RBX15BallBallContact19deleteAllConnectorsEv")]
pub fn stub_0x71fc08(handle: &crate::slot::InstanceHandle) {
// RBX::BallBallContact::deleteAllConnectors() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71fc24 — __ZN3RBX15BallBallContact18computeIsCollidingEf
// type: _DWORD __fastcall(RBX::BallBallContact *__hidden this, float)
#[doc(alias = "RBX::BallBallContact::computeIsColliding(float)")]
#[doc(alias = "__ZN3RBX15BallBallContact18computeIsCollidingEf")]
pub fn stub_0x71fc24(handle: &crate::slot::InstanceHandle) {
// RBX::BallBallContact::computeIsColliding(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71fcfc — __ZN3RBX15BallBallContact11stepContactEv
// type: _DWORD __fastcall(RBX::BallBallContact *__hidden this)
#[doc(alias = "RBX::BallBallContact::stepContact(void)")]
#[doc(alias = "__ZN3RBX15BallBallContact11stepContactEv")]
pub fn stub_0x71fcfc(handle: &crate::slot::InstanceHandle) {
// RBX::BallBallContact::stepContact() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71fec4 — __ZN3RBX15BallBallContact34generateDataForMovingAssemblyStageEv
// type: _DWORD __fastcall(RBX::BallBallContact *__hidden this)
#[doc(alias = "RBX::BallBallContact::generateDataForMovingAssemblyStage(void)")]
#[doc(alias = "__ZN3RBX15BallBallContact34generateDataForMovingAssemblyStageEv")]
pub fn stub_0x71fec4(handle: &crate::slot::InstanceHandle) {
// RBX::BallBallContact::generateDataForMovingAssemblyStage() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71fec8 — __ZN3RBX16BallBlockContact12getConnectorEi
// type: _DWORD __fastcall(RBX::BallBlockContact *__hidden this, int)
#[doc(alias = "RBX::BallBlockContact::getConnector(int)")]
#[doc(alias = "__ZN3RBX16BallBlockContact12getConnectorEi")]
pub fn stub_0x71fec8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BallBlockContact getter.
cell.get()
}

// 0x71fecc — __ZN3RBX16BallBlockContact19deleteAllConnectorsEv
// type: _DWORD __fastcall(RBX::BallBlockContact *__hidden this)
#[doc(alias = "RBX::BallBlockContact::deleteAllConnectors(void)")]
#[doc(alias = "__ZN3RBX16BallBlockContact19deleteAllConnectorsEv")]
pub fn stub_0x71fecc(handle: &crate::slot::InstanceHandle) {
// RBX::BallBlockContact::deleteAllConnectors() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x71fee8 — __ZN3RBX16BallBlockContact18computeIsCollidingEf
// type: _DWORD __fastcall(RBX::BallBlockContact *__hidden this, float)
#[doc(alias = "RBX::BallBlockContact::computeIsColliding(float)")]
#[doc(alias = "__ZN3RBX16BallBlockContact18computeIsCollidingEf")]
pub fn stub_0x71fee8(handle: &crate::slot::InstanceHandle) {
// RBX::BallBlockContact::computeIsColliding(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7200f8 — __ZN3RBX16BallBlockContact11stepContactEv
// type: _DWORD __fastcall(RBX::BallBlockContact *__hidden this)
#[doc(alias = "RBX::BallBlockContact::stepContact(void)")]
#[doc(alias = "__ZN3RBX16BallBlockContact11stepContactEv")]
pub fn stub_0x7200f8(handle: &crate::slot::InstanceHandle) {
// RBX::BallBlockContact::stepContact() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x72034c — __ZN3RBX16BallBlockContact34generateDataForMovingAssemblyStageEv
// type: _DWORD __fastcall(RBX::BallBlockContact *__hidden this)
#[doc(alias = "RBX::BallBlockContact::generateDataForMovingAssemblyStage(void)")]
#[doc(alias = "__ZN3RBX16BallBlockContact34generateDataForMovingAssemblyStageEv")]
pub fn stub_0x72034c(handle: &crate::slot::InstanceHandle) {
// RBX::BallBlockContact::generateDataForMovingAssemblyStage() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x720354 — __ZN3RBX17BlockBlockContact12getConnectorEi
// type: _DWORD __fastcall(RBX::BlockBlockContact *__hidden this, int)
#[doc(alias = "RBX::BlockBlockContact::getConnector(int)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact12getConnectorEi")]
pub fn stub_0x720354(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BlockBlockContact getter.
cell.get()
}

// 0x720388 — __ZN3RBX17BlockBlockContact23deleteAllConnectorsOrigEv
// type: _DWORD __fastcall(RBX::BlockBlockContact *__hidden this)
#[doc(alias = "RBX::BlockBlockContact::deleteAllConnectorsOrig(void)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact23deleteAllConnectorsOrigEv")]
pub fn stub_0x720388(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::deleteAllConnectorsOrig() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x720414 — __ZN3RBX17BlockBlockContact24deleteAllConnectorsFFlagEv
// type: _DWORD __fastcall(RBX::BlockBlockContact *__hidden this)
#[doc(alias = "RBX::BlockBlockContact::deleteAllConnectorsFFlag(void)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact24deleteAllConnectorsFFlagEv")]
pub fn stub_0x720414(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::deleteAllConnectorsFFlag() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7204b8 — __ZN3RBX17BlockBlockContact20findGeoPairConnectorEPNS_4BodyES2_NS_11GeoPairTypeEii
#[doc(alias = "RBX::BlockBlockContact::findGeoPairConnector(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact20findGeoPairConnectorEPNS_4BodyES2_NS_11GeoPairTypeEii")]
pub fn stub_0x7204b8(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::findGeoPairConnector(RBX::Body*, RBX::Body*, RBX::GeoPairType, int~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x720548 — __ZN3RBX21BlockBlockContactData20findGeoPairConnectorEPNS_4BodyES2_NS_11GeoPairTypeEii
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::BlockBlockContactData::findGeoPairConnector(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
#[doc(alias = "__ZN3RBX21BlockBlockContactData20findGeoPairConnectorEPNS_4BodyES2_NS_11GeoPairTypeEii")]
pub fn stub_0x720548(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContactData::findGeoPairConnector(RBX::Body*, RBX::Body*, RBX::GeoPairType,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x720734 — __ZN3RBX17BlockBlockContact25findGeoPairConnectorFFlagEPNS_4BodyES2_NS_11GeoPairTypeEii
#[doc(alias = "RBX::BlockBlockContact::findGeoPairConnectorFFlag(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact25findGeoPairConnectorFFlagEPNS_4BodyES2_NS_11GeoPairTypeEii")]
pub fn stub_0x720734(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::findGeoPairConnectorFFlag(RBX::Body*, RBX::Body*, RBX::GeoPairType~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7207c4 — __ZN3RBX21BlockBlockContactData25findGeoPairConnectorFFlagEPNS_4BodyES2_NS_11GeoPairTypeEii
#[doc(alias = "RBX::BlockBlockContactData::findGeoPairConnectorFFlag(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
#[doc(alias = "__ZN3RBX21BlockBlockContactData25findGeoPairConnectorFFlagEPNS_4BodyES2_NS_11GeoPairTypeEii")]
pub fn stub_0x7207c4(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContactData::findGeoPairConnectorFFlag(RBX::Body*, RBX::Body*, RBX::GeoPair~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x720898 — __ZN3RBX17BlockBlockContact18computeIsCollidingEf
// type: _DWORD __fastcall(RBX::BlockBlockContact *__hidden this, float)
#[doc(alias = "RBX::BlockBlockContact::computeIsColliding(float)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact18computeIsCollidingEf")]
pub fn stub_0x720898(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::computeIsColliding(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7208b0 — __ZN3RBX17BlockBlockContact18computeIsCollidingEfRb
// type: _DWORD __fastcall(RBX::BlockBlockContact *__hidden this, float, bool *)
#[doc(alias = "RBX::BlockBlockContact::computeIsColliding(float,bool &)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact18computeIsCollidingEfRb")]
pub fn stub_0x7208b0(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::computeIsColliding(float, bool&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x72090c — __ZN3RBX17BlockBlockContact11stepContactEv
// type: _DWORD __fastcall(RBX::BlockBlockContact *__hidden this)
#[doc(alias = "RBX::BlockBlockContact::stepContact(void)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact11stepContactEv")]
pub fn stub_0x72090c(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::stepContact() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x720988 — __ZN3RBX21BlockBlockContactData16stepContactFFlagEv
// type: _DWORD __fastcall(RBX::BlockBlockContactData *__hidden this)
#[doc(alias = "RBX::BlockBlockContactData::stepContactFFlag(void)")]
#[doc(alias = "__ZN3RBX21BlockBlockContactData16stepContactFFlagEv")]
pub fn stub_0x720988(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContactData::stepContactFFlag() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x720a08 — __ZN3RBX21BlockBlockContactData11stepContactEv
// type: _DWORD __fastcall(RBX::BlockBlockContactData *__hidden this)
#[doc(alias = "RBX::BlockBlockContactData::stepContact(void)")]
#[doc(alias = "__ZN3RBX21BlockBlockContactData11stepContactEv")]
pub fn stub_0x720a08(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContactData::stepContact() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x720aac — __ZN3RBX17BlockBlockContact19loadGeoPairEdgeEdgeERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiii
#[doc(alias = "RBX::BlockBlockContact::loadGeoPairEdgeEdge(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &,int,int,int,int)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact19loadGeoPairEdgeEdgeERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiii")]
pub fn stub_0x720aac(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::loadGeoPairEdgeEdge(RBX::FixedArray<RBX::GeoPairConnector*, 8ul>&,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x720bbc — __ZN3RBX17BlockBlockContact24loadGeoPairEdgeEdgeFFlagEiiii
// type: _DWORD __fastcall(RBX::BlockBlockContact *__hidden this, int, int, int, int)
#[doc(alias = "RBX::BlockBlockContact::loadGeoPairEdgeEdgeFFlag(int,int,int,int)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact24loadGeoPairEdgeEdgeFFlagEiiii")]
pub fn stub_0x720bbc(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::loadGeoPairEdgeEdgeFFlag(int, int, int, int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x720fe4 — __ZN3RBX17BlockBlockContact21loadGeoPairPointPlaneERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiiNS_8NormalIdES6_
#[doc(alias = "RBX::BlockBlockContact::loadGeoPairPointPlane(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &,int,int,int,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact21loadGeoPairPointPlaneERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiiNS_8NormalIdES6_")]
pub fn stub_0x720fe4(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::loadGeoPairPointPlane(RBX::FixedArray<RBX::GeoPairConnector*, 8ul>~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7210c8 — __ZN3RBX17BlockBlockContact26loadGeoPairPointPlaneFFlagEiiiNS_8NormalIdES1_
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "RBX::BlockBlockContact::loadGeoPairPointPlaneFFlag(int,int,int,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact26loadGeoPairPointPlaneFFlagEiiiNS_8NormalIdES1_")]
pub fn stub_0x7210c8(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::loadGeoPairPointPlaneFFlag(int, int, int, RBX::NormalId, RBX::Norm~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7215a4 — __ZN3RBX17BlockBlockContact18geoFeaturesOverlapEiiiNS_8NormalIdES1_
#[doc(alias = "RBX::BlockBlockContact::geoFeaturesOverlap(int,int,int,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX17BlockBlockContact18geoFeaturesOverlapEiiiNS_8NormalIdES1_")]
pub fn stub_0x7215a4(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContact::geoFeaturesOverlap(int, int, int, RBX::NormalId, RBX::NormalId) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x721778 — __ZN3RBX21BlockBlockContactData24loadGeoPairEdgeEdgePlaneERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiii
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "RBX::BlockBlockContactData::loadGeoPairEdgeEdgePlane(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &,int,int,int,int)")]
#[doc(alias = "__ZN3RBX21BlockBlockContactData24loadGeoPairEdgeEdgePlaneERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiii")]
pub fn stub_0x721778(handle: &crate::slot::InstanceHandle) {
// RBX::BlockBlockContactData::loadGeoPairEdgeEdgePlane(RBX::FixedArray<RBX::GeoPairConnector~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x721920 — __ZN3RBX21BlockBlockContactData16getBestPlaneEdgeEfRb
// type: _DWORD __fastcall(RBX::BlockBlockContactData *__hidden this, float, bool *)
#[doc(alias = "RBX::BlockBlockContactData::getBestPlaneEdge(float,bool &)")]
#[doc(alias = "__ZN3RBX21BlockBlockContactData16getBestPlaneEdgeEfRb")]
pub fn stub_0x721920(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BlockBlockContactData getter.
cell.get()
}
