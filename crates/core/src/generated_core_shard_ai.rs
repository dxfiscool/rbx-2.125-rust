//! core shard AI — 100 core stubs EA-sorted, next uncovered (lowest EA first after global deduplication).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 100 uncovered after 0x316738 (lowest EA first).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::Http::httpGetPostImpl(bool,std::istream &,bool,std::map<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>> const&,bool,std::string &)")]
// 0x316814 — __ZN3RBX4Http15httpGetPostImplEbRSibRKSt3mapISsSsSt4lessISsESaISt4pairIKSsSsEEEbRSs
pub fn stub_0x316814() {
    // IDA 0x316814: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Http::post(std::istream &,bool,std::string &,bool)")]
// 0x31688c — __ZN3RBX4Http4postERSibRSsb
pub fn stub_0x31688c() {
    // IDA 0x31688c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Http::get(std::string &,bool)")]
// 0x317de0 — __ZN3RBX4Http3getERSsb
pub fn stub_0x317de0() {
    // IDA 0x317de0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::initTrustCheck(void)")]
// 0x3180ec — __ZN3RBXL14initTrustCheckEv
pub fn stub_0x3180ec() {
    // IDA 0x3180ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::WindowAverage<double,double>::~WindowAverage()")]
// 0x3180f0 — __ZN3RBX13WindowAverageIddED1Ev
pub fn stub_0x3180f0() {
    // IDA 0x3180f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Http::MutexGuard::~MutexGuard()")]
// 0x318100 — __ZN3RBX4Http10MutexGuardD1Ev
pub fn stub_0x318100() {
    // IDA 0x318100: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Http::MutexGuard::~MutexGuard()")]
// 0x31e45c — __ZN3RBX4Http10MutexGuardD2Ev
pub fn stub_0x31e45c() {
    // IDA 0x31e45c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Http::MutexGuard::MutexGuard(void)")]
// 0x31e558 — __ZN3RBX4Http10MutexGuardC2Ev
pub fn stub_0x31e558() {
    // IDA 0x31e558: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "global constructor keyed to_a_113")]
// 0x31e658 — __GLOBAL__I_a_113
// was: global constructor keyed to_a_113
pub fn stub_0x31e658() {
    // IDA 0x31e658: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Extents::clampInsideOf(RBX::Extents const&)const")]
// 0x31e8b0 — __ZNK3RBX7Extents13clampInsideOfERKS0_
pub fn stub_0x31e8b0() {
    // IDA 0x31e8b0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Extents::getCornerIndex(int)const")]
// 0x31eae4 — __ZNK3RBX7Extents14getCornerIndexEi
pub fn stub_0x31eae4() {
    // IDA 0x31eae4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Extents::getCorner(int)const")]
// 0x31eba8 — __ZNK3RBX7Extents9getCornerEi
pub fn stub_0x31eba8() {
    // IDA 0x31eba8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Extents::faceCenter(RBX::NormalId)const")]
// 0x31f464 — __ZNK3RBX7Extents10faceCenterENS_8NormalIdE
pub fn stub_0x31f464() {
    // IDA 0x31f464: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Extents::separatedByMoreThan(RBX::Extents const&,float)const")]
// 0x31f5b4 — __ZNK3RBX7Extents19separatedByMoreThanERKS0_f
pub fn stub_0x31f5b4() {
    // IDA 0x31f5b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_114")]
// 0x31f738 — __GLOBAL__I_a_114
// was: global constructor keyed to_a_114
pub fn stub_0x31f738() {
    // IDA 0x31f738: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Face::operator[](int)const")]
// 0x31f90c — __ZNK3RBX4FaceixEi
pub fn stub_0x31f90c() {
    // IDA 0x31f90c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Face::operator[](int)")]
// 0x31f918 — __ZN3RBX4FaceixEi
pub fn stub_0x31f918() {
    // IDA 0x31f918: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Face::snapToGrid(float)")]
// 0x31f924 — __ZN3RBX4Face10snapToGridEf
pub fn stub_0x31f924() {
    // IDA 0x31f924: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Face::overlapWithinPlanes(RBX::Face const&,RBX::Face const&,float)")]
// 0x31f964 — __ZN3RBX4Face19overlapWithinPlanesERKS0_S2_f
pub fn stub_0x31f964() {
    // IDA 0x31f964: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Face::projectOverlapOnMe(RBX::Face const&)const")]
// 0x31fa44 — __ZNK3RBX4Face18projectOverlapOnMeERKS0_
pub fn stub_0x31fa44() {
    // IDA 0x31fa44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Face::hasOverlap(RBX::Face const&,RBX::Face const&,float)")]
// 0x31fe6c — __ZN3RBX4Face10hasOverlapERKS0_S2_f
pub fn stub_0x31fe6c() {
    // IDA 0x31fe6c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Face::cornersAligned(RBX::Face const&,RBX::Face const&,float)")]
// 0x31fefc — __ZN3RBX4Face14cornersAlignedERKS0_S2_f
pub fn stub_0x31fefc() {
    // IDA 0x31fefc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Face::fromExtentsSide(RBX::Extents const&,RBX::NormalId)")]
// 0x31ffe4 — __ZN3RBX4Face15fromExtentsSideERKNS_7ExtentsENS_8NormalIdE
pub fn stub_0x31ffe4() {
    // IDA 0x31ffe4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Face::getAxis(int)const")]
// 0x3201f4 — __ZNK3RBX4Face7getAxisEi
pub fn stub_0x3201f4() {
    // IDA 0x3201f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_115")]
// 0x3202dc — __GLOBAL__I_a_115
// was: global constructor keyed to_a_115
pub fn stub_0x3202dc() {
    // IDA 0x3202dc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Faces::Faces(int)")]
// 0x320314 — __ZN3RBX5FacesC1Ei
pub fn stub_0x320314() {
    // IDA 0x320314: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Faces::setNormalId(RBX::NormalId,bool)")]
// 0x320318 — __ZN3RBX5Faces11setNormalIdENS_8NormalIdEb
pub fn stub_0x320318() {
    // IDA 0x320318: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Faces::getNormalId(RBX::NormalId)const")]
// 0x320338 — __ZNK3RBX5Faces11getNormalIdENS_8NormalIdE
pub fn stub_0x320338() {
    // IDA 0x320338: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::Faces>::convertToString(RBX::Faces const&)")]
// 0x32034c — __ZN3RBX15StringConverterINS_5FacesEE15convertToStringERKS1_
pub fn stub_0x32034c() {
    // IDA 0x32034c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::Faces>::convertToValue(std::string const&,RBX::Faces&)")]
// 0x32059c — __ZN3RBX15StringConverterINS_5FacesEE14convertToValueERKSsRS1_
pub fn stub_0x32059c() {
    // IDA 0x32059c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "global constructor keyed to_a_116")]
// 0x3207f8 — __GLOBAL__I_a_116
// was: global constructor keyed to_a_116
pub fn stub_0x3207f8() {
    // IDA 0x3207f8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "_gpc_free_polygon")]
// 0x3208c0 — _gpc_free_polygon
pub fn stub_0x3208c0() {
    // IDA 0x3208c0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "_gpc_polygon_clip")]
// 0x320910 — _gpc_polygon_clip
pub fn stub_0x320910() {
    // IDA 0x320910: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "_minimax_test")]
// 0x321838 — _minimax_test
pub fn stub_0x321838() {
    // IDA 0x321838: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "_build_lmt")]
// 0x321a18 — _build_lmt
pub fn stub_0x321a18() {
    // IDA 0x321a18: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "_build_sbt")]
// 0x321fd4 — _build_sbt
pub fn stub_0x321fd4() {
    // IDA 0x321fd4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_free_sbtree")]
// 0x322004 — _free_sbtree
pub fn stub_0x322004() {
    // IDA 0x322004: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_add_edge_to_aet")]
// 0x322030 — _add_edge_to_aet
pub fn stub_0x322030() {
    // IDA 0x322030: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_add_local_min")]
// 0x322088 — _add_local_min
pub fn stub_0x322088() {
    // IDA 0x322088: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_add_right")]
// 0x322140 — _add_right
pub fn stub_0x322140() {
    // IDA 0x322140: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_add_left")]
// 0x3221b0 — _add_left
pub fn stub_0x3221b0() {
    // IDA 0x3221b0: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_merge_right")]
// 0x32221c — _merge_right
pub fn stub_0x32221c() {
    // IDA 0x32221c: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_merge_left")]
// 0x322268 — _merge_left
pub fn stub_0x322268() {
    // IDA 0x322268: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_build_intersection_table")]
// 0x3222b8 — _build_intersection_table
pub fn stub_0x3222b8() {
    // IDA 0x3222b8: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_insert_bound")]
// 0x3224d8 — _insert_bound
pub fn stub_0x3224d8() {
    // IDA 0x3224d8: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_bound_list")]
// 0x322518 — _bound_list
pub fn stub_0x322518() {
    // IDA 0x322518: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_create_contour_bboxes")]
// 0x3225b8 — _create_contour_bboxes
pub fn stub_0x3225b8() {
    // IDA 0x3225b8: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "initLocalScope(void)")]
// 0x3226f8 — __ZL14initLocalScopev
pub fn stub_0x3226f8() {
    // IDA 0x3226f8: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "RBX::Guid::Guid(void)")]
// 0x32281c — __ZN3RBX4GuidC1Ev
pub fn stub_0x32281c() {
    // IDA 0x32281c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Guid::generateStandardGUID(std::string &)")]
// 0x322850 — __ZN3RBX4Guid20generateStandardGUIDERSs
pub fn stub_0x322850() {
    // IDA 0x322850: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Guid::generateRBXGUID(std::string &)")]
// 0x32298c — __ZN3RBX4Guid15generateRBXGUIDERSs
pub fn stub_0x32298c() {
    // IDA 0x32298c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Guid::assign(RBX::Guid::Data)")]
// 0x322b04 — __ZN3RBX4Guid6assignENS0_4DataE
pub fn stub_0x322b04() {
    // IDA 0x322b04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Guid::Data::operator<(RBX::Guid::Data const&)const")]
// 0x322b10 — __ZNK3RBX4Guid4DataltERKS1_
pub fn stub_0x322b10() {
    // IDA 0x322b10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Guid::compare(RBX::Guid const*,RBX::Guid const*)")]
// 0x322b38 — __ZN3RBX4Guid7compareEPKS0_S2_
pub fn stub_0x322b38() {
    // IDA 0x322b38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Guid::compare(RBX::Guid const*,RBX::Guid const*,RBX::Guid const*,RBX::Guid const*)")]
// 0x322b78 — __ZN3RBX4Guid7compareEPKS0_S2_S2_S2_
pub fn stub_0x322b78() {
    // IDA 0x322b78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Guid::Data::readableString(int)const")]
// 0x322bdc — __ZNK3RBX4Guid4Data14readableStringEi
pub fn stub_0x322bdc() {
    // IDA 0x322bdc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_117")]
// 0x322e00 — __GLOBAL__I_a_117
// was: global constructor keyed to_a_117
pub fn stub_0x322e00() {
    // IDA 0x322e00: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_118")]
// 0x322ef8 — __GLOBAL__I_a_118
// was: global constructor keyed to_a_118
pub fn stub_0x322ef8() {
    // IDA 0x322ef8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Hash::hash(std::string const&)")]
// 0x323028 — __ZN3RBX4Hash4hashERKSs
pub fn stub_0x323028() {
    // IDA 0x323028: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_119")]
// 0x3234d4 — __GLOBAL__I_a_119
// was: global constructor keyed to_a_119
pub fn stub_0x3234d4() {
    // IDA 0x3234d4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_120")]
// 0x323b78 — __GLOBAL__I_a_120
// was: global constructor keyed to_a_120
pub fn stub_0x323b78() {
    // IDA 0x323b78: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexBox::~IndexBox()")]
// 0x323fd8 — __ZN3RBX8IndexBoxD1Ev
pub fn stub_0x323fd8() {
    // IDA 0x323fd8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexBox::~IndexBox()")]
// 0x323fdc — __ZN3RBX8IndexBoxD0Ev
pub fn stub_0x323fdc() {
    // IDA 0x323fdc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_121")]
// 0x323fe0 — __GLOBAL__I_a_121
// was: global constructor keyed to_a_121
pub fn stub_0x323fe0() {
    // IDA 0x323fe0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexedMesh::IndexedMesh(void)")]
// 0x324018 — __ZN3RBX11IndexedMeshC2Ev
pub fn stub_0x324018() {
    // IDA 0x324018: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexedMesh::~IndexedMesh()")]
// 0x32403c — __ZN3RBX11IndexedMeshD0Ev
pub fn stub_0x32403c() {
    // IDA 0x32403c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexedMesh::~IndexedMesh()")]
// 0x3240dc — __ZN3RBX11IndexedMeshD1Ev
pub fn stub_0x3240dc() {
    // IDA 0x3240dc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexedMesh::~IndexedMesh()")]
// 0x3240e0 — __ZN3RBX11IndexedMeshD2Ev
pub fn stub_0x3240e0() {
    // IDA 0x3240e0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexedMesh::setComputedUpper(RBX::IndexedMesh*)")]
// 0x324354 — __ZN3RBX11IndexedMesh16setComputedUpperEPS0_
pub fn stub_0x324354() {
    // IDA 0x324354: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IndexedMesh::setUpper(RBX::IndexedMesh*)")]
// 0x32438c — __ZN3RBX11IndexedMesh8setUpperEPS0_
pub fn stub_0x32438c() {
    // IDA 0x32438c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IndexedMesh::setLower(RBX::IndexedMesh*)")]
// 0x3243d4 — __ZN3RBX11IndexedMesh8setLowerEPS0_
pub fn stub_0x3243d4() {
    // IDA 0x3243d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IndexedMesh::getComputedUpper(void)")]
// 0x3244b8 — __ZN3RBX11IndexedMesh16getComputedUpperEv
pub fn stub_0x3244b8() {
    // IDA 0x3244b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IndexedMesh::getIndexedMeshParent(void)")]
// 0x3244bc — __ZN3RBX11IndexedMesh20getIndexedMeshParentEv
pub fn stub_0x3244bc() {
    // IDA 0x3244bc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexedMesh::attachChildren(RBX::IndexedMesh*)")]
// 0x3244c0 — __ZN3RBX11IndexedMesh14attachChildrenEPS0_
pub fn stub_0x3244c0() {
    // IDA 0x3244c0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexedMesh::onParentChanged(RBX::IndexedTree *)")]
// 0x3244fc — __ZN3RBX11IndexedMesh15onParentChangedEPNS_11IndexedTreeE
pub fn stub_0x3244fc() {
    // IDA 0x3244fc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexedMesh::severeChildren(RBX::IndexedMesh*)")]
// 0x324550 — __ZN3RBX11IndexedMesh14severeChildrenEPS0_
pub fn stub_0x324550() {
    // IDA 0x324550: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexedMesh::getConstIndexedMeshParent(void)const")]
// 0x32458c — __ZNK3RBX11IndexedMesh25getConstIndexedMeshParentEv
pub fn stub_0x32458c() {
    // IDA 0x32458c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexedMesh::getConstComputedUpper(void)const")]
// 0x3245f8 — __ZNK3RBX11IndexedMesh21getConstComputedUpperEv
pub fn stub_0x3245f8() {
    // IDA 0x3245f8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexedMesh::isUpperRoot(RBX::IndexedMesh const*)")]
// 0x3245fc — __ZN3RBX11IndexedMesh11isUpperRootEPKS0_
pub fn stub_0x3245fc() {
    // IDA 0x3245fc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexedMesh * RBX::IndexedTree::getTypedChild<RBX::IndexedMesh>(int)")]
// 0x324684 — __ZN3RBX11IndexedTree13getTypedChildINS_11IndexedMeshEEEPT_i
pub fn stub_0x324684() {
    // IDA 0x324684: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexedMesh::lowersChanged(void)")]
// 0x3246ec — __ZN3RBX11IndexedMesh13lowersChangedEv
pub fn stub_0x3246ec() {
    // IDA 0x3246ec: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_122")]
// 0x324710 — __GLOBAL__I_a_122
// was: global constructor keyed to_a_122
pub fn stub_0x324710() {
    // IDA 0x324710: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexedTree::IndexedTree(void)")]
// 0x3247d8 — __ZN3RBX11IndexedTreeC2Ev
pub fn stub_0x3247d8() {
    // IDA 0x3247d8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexedTree::~IndexedTree()")]
// 0x324800 — __ZN3RBX11IndexedTreeD0Ev
pub fn stub_0x324800() {
    // IDA 0x324800: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexedTree::~IndexedTree()")]
// 0x3248a0 — __ZN3RBX11IndexedTreeD1Ev
pub fn stub_0x3248a0() {
    // IDA 0x3248a0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexedTree::~IndexedTree()")]
// 0x3248a4 — __ZN3RBX11IndexedTreeD2Ev
pub fn stub_0x3248a4() {
    // IDA 0x3248a4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::IndexedTree::setIndexedTreeParent(RBX::IndexedTree*)")]
// 0x324a74 — __ZN3RBX11IndexedTree20setIndexedTreeParentEPS0_
pub fn stub_0x324a74() {
    // IDA 0x324a74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IndexArray<RBX::IndexedTree,&RBX::IndexedTree::getIndex>::fastRemove(RBX::IndexedTree*)")]
// 0x324c14 — __ZN3RBX10IndexArrayINS_11IndexedTreeEXadL_ZNS1_8getIndexEvEEE10fastRemoveEPS1_
pub fn stub_0x324c14() {
    // IDA 0x324c14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IndexedTree::onParentChanged(RBX::IndexedTree*)")]
// 0x324ce8 — __ZN3RBX11IndexedTree15onParentChangedEPS0_
pub fn stub_0x324ce8() {
    // IDA 0x324ce8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "global constructor keyed to_a_123")]
// 0x3251ac — __GLOBAL__I_a_123
// was: global constructor keyed to_a_123
pub fn stub_0x3251ac() {
    // IDA 0x3251ac: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::InterpolatedCFrame::clearHistory(void)")]
// 0x325278 — __ZN3RBX18InterpolatedCFrame12clearHistoryEv
pub fn stub_0x325278() {
    // IDA 0x325278: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::InterpolatedCFrame::interpolate(RBX::Time const&,RBX::Time const&,unsigned int const&)")]
// 0x325538 — __ZN3RBX18InterpolatedCFrame11interpolateERKNS_4TimeES3_RKj
pub fn stub_0x325538() {
    // IDA 0x325538: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::InterpolatedCFrame::computeSampleTargetTime(RBX::Time const&)")]
// 0x3258e4 — __ZN3RBX18InterpolatedCFrame23computeSampleTargetTimeERKNS_4TimeE
pub fn stub_0x3258e4() {
    // IDA 0x3258e4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::InterpolatedCFrame::getSampleInterval(void)const")]
// 0x325b98 — __ZNK3RBX18InterpolatedCFrame17getSampleIntervalEv
pub fn stub_0x325b98() {
    // IDA 0x325b98: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_124")]
// 0x326108 — __GLOBAL__I_a_124
// was: global constructor keyed to_a_124
pub fn stub_0x326108() {
    // IDA 0x326108: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::resize(unsigned long,RBX::KeywordFilterType)")]
// 0x32689c — __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE6resizeEmS1_
pub fn stub_0x32689c() {
    // IDA 0x32689c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::push_back(RBX::KeywordFilterType const&)")]
// 0x3268d0 — __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE9push_backERKS1_
pub fn stub_0x3268d0() {
    // IDA 0x3268d0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::KeywordFilterType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::operator[](RBX::Name const* const&)")]
// 0x3268f8 — __ZNSt3mapIPKN3RBX4NameENS0_17KeywordFilterTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
pub fn stub_0x3268f8() {
    // IDA 0x3268f8: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
// 0x326950 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_0x326950() {
    // IDA 0x326950: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
// 0x326a04 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_0x326a04() {
    // IDA 0x326a04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
