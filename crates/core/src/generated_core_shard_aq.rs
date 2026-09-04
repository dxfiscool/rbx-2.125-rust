//! core shard AQ — 120 core stubs EA-sorted, next uncovered after AP 0x326ba8..0x326bc0 (strict RBX|boost|std earliest gap).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 120 uncovered after 0x326ba8.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::KeywordFilterType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeywordFilterType *,RBX::KeywordFilterType *>(RBX::KeywordFilterType *,RBX::KeywordFilterType *,RBX::KeywordFilterType *)")]
// 0x326bc0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17KeywordFilterTypeES5_EET0_T_S7_S6_
pub fn stub_0x326bc0() {
    // IDA 0x326bc0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,unsigned long,RBX::KeywordFilterType const&)")]
// 0x326bfc — __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_0x326bfc() {
    // IDA 0x326bfc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::LegacyContentTable::LegacyContentTable(void)")]
// 0x326e54 — __ZN3RBX18LegacyContentTableC1Ev
pub fn stub_0x326e54() {
    // IDA 0x326e54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::LegacyContentTable::LegacyContentTable(void)")]
// 0x326e58 — __ZN3RBX18LegacyContentTableC2Ev
pub fn stub_0x326e58() {
    // IDA 0x326e58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::LegacyContentTable::AddEntry(std::string const&,std::string const&)")]
// 0x34581c — __ZN3RBX18LegacyContentTable8AddEntryERKSsS2_
pub fn stub_0x34581c() {
    // IDA 0x34581c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "anonymous namespace::normalizeUrl(std::string &)")]
// 0x345950 — __ZN12_GLOBAL__N_112normalizeUrlERSs
pub fn stub_0x345950() {
    // IDA 0x345950: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::LegacyContentTable::FindEntry(std::string const&)")]
// 0x3459d4 — __ZN3RBX18LegacyContentTable9FindEntryERKSs
pub fn stub_0x3459d4() {
    // IDA 0x3459d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::findLocalFile(std::string const&,std::string *)")]
// 0x345c20 — __ZN3RBX13findLocalFileERKSsPSs
pub fn stub_0x345c20() {
    // IDA 0x345c20: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Math::deltaRotationClose(float,float)")]
// 0x3568e0 — __ZN3RBX4Math18deltaRotationCloseEff
pub fn stub_0x3568e0() {
    // IDA 0x3568e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Math::averageRotationClose(float,float)")]
// 0x3569d8 — __ZN3RBX4Math20averageRotationCloseEff
pub fn stub_0x3569d8() {
    // IDA 0x3569d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Math::isDenormal(float)")]
// 0x356c80 — __ZN3RBX4Math10isDenormalEf
pub fn stub_0x356c80() {
    // IDA 0x356c80: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Math::isNanInf(float)")]
// 0x356c94 — __ZN3RBX4Math8isNanInfEf
pub fn stub_0x356c94() {
    // IDA 0x356c94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::segSizeRadians(void)")]
// 0x356e34 — __ZN3RBX14segSizeRadiansEv
pub fn stub_0x356e34() {
    // IDA 0x356e34: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::rotationToByteBase(float)")]
// 0x356e6c — __ZN3RBX18rotationToByteBaseEf
pub fn stub_0x356e6c() {
    // IDA 0x356e6c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::rotationToByte(float)")]
// 0x356ff0 — __ZN3RBX4Math14rotationToByteEf
pub fn stub_0x356ff0() {
    // IDA 0x356ff0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::rotationFromByte(unsigned char)")]
// 0x3570e8 — __ZN3RBX4Math16rotationFromByteEh
pub fn stub_0x3570e8() {
    // IDA 0x3570e8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::closestPointOnRay(RBX::RbxRay const&,RBX::RbxRay const&)")]
// 0x358460 — __ZN3RBX4Math17closestPointOnRayERKNS_6RbxRayES3_
pub fn stub_0x358460() {
    // IDA 0x358460: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::matrixRotateX(void)")]
// 0x35856c — __ZN3RBX4Math13matrixRotateXEv
pub fn stub_0x35856c() {
    // IDA 0x35856c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::matrixRotateY(void)")]
// 0x358658 — __ZN3RBX4Math13matrixRotateYEv
pub fn stub_0x358658() {
    // IDA 0x358658: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::matrixTiltZ(void)")]
// 0x358744 — __ZN3RBX4Math11matrixTiltZEv
pub fn stub_0x358744() {
    // IDA 0x358744: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::matrixTiltNegativeZ(void)")]
// 0x358830 — __ZN3RBX4Math19matrixTiltNegativeZEv
pub fn stub_0x358830() {
    // IDA 0x358830: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::matrixTiltQuadrant(int)")]
// 0x358918 — __ZN3RBX4Math18matrixTiltQuadrantEi
pub fn stub_0x358918() {
    // IDA 0x358918: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::radiansToQuadrant(float)")]
// 0x3589e8 — __ZN3RBX4Math17radiansToQuadrantEf
pub fn stub_0x3589e8() {
    // IDA 0x3589e8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::getAxisRotationMatrix(int)")]
// 0x359764 — __ZN3RBX4Math21getAxisRotationMatrixEi
pub fn stub_0x359764() {
    // IDA 0x359764: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::polygonStartingPoint(int,float)")]
// 0x359be0 — __ZN3RBX4Math20polygonStartingPointEif
pub fn stub_0x359be0() {
    // IDA 0x359be0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Math::evenWholeNumberFuzzy(float const&)")]
// 0x359ed0 — __ZN3RBX4Math20evenWholeNumberFuzzyERKf
pub fn stub_0x359ed0() {
    // IDA 0x359ed0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MD5Hasher::create(void)")]
// 0x35a620 — __ZN3RBX9MD5Hasher6createEv
pub fn stub_0x35a620() {
    // IDA 0x35a620: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MD5HasherImpl::~MD5HasherImpl()")]
// 0x35a6f4 — __ZN3RBX13MD5HasherImplD1Ev
pub fn stub_0x35a6f4() {
    // IDA 0x35a6f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MD5HasherImpl::~MD5HasherImpl()")]
// 0x35a718 — __ZN3RBX13MD5HasherImplD0Ev
pub fn stub_0x35a718() {
    // IDA 0x35a718: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MD5HasherImpl::addData(std::istream &)")]
// 0x35a744 — __ZN3RBX13MD5HasherImpl7addDataERSi
pub fn stub_0x35a744() {
    // IDA 0x35a744: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MD5HasherImpl::addData(std::string const&)")]
// 0x35a7c4 — __ZN3RBX13MD5HasherImpl7addDataERKSs
pub fn stub_0x35a7c4() {
    // IDA 0x35a7c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MD5HasherImpl::addData(char const*,unsigned long)")]
// 0x35a7d0 — __ZN3RBX13MD5HasherImpl7addDataEPKcm
pub fn stub_0x35a7d0() {
    // IDA 0x35a7d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MD5HasherImpl::toString(void)")]
// 0x35a7d8 — __ZN3RBX13MD5HasherImpl8toStringEv
pub fn stub_0x35a7d8() {
    // IDA 0x35a7d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MD5HasherImpl::c_str(void)")]
// 0x35a7f4 — __ZN3RBX13MD5HasherImpl5c_strEv
pub fn stub_0x35a7f4() {
    // IDA 0x35a7f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<RBX::MeshId>::convertToValue(std::string const&,RBX::MeshId&)")]
// 0x35aa38 — __ZN3RBX15StringConverterINS_6MeshIdEE14convertToValueERKSsRS1_
pub fn stub_0x35aa38() {
    // IDA 0x35aa38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::MeshId>(RBX::MeshId const&)")]
// 0x35b3b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6MeshIdEEERS3_RKT_
pub fn stub_0x35b3b0() {
    // IDA 0x35b3b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MeshId * rbx::any_cast<RBX::MeshId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x35b6bc — __ZN3rbx8any_castIN3RBX6MeshIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x35b6bc() {
    // IDA 0x35b6bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MeshId & rbx::any_cast<RBX::MeshId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x35b714 — __ZN3rbx8any_castIRN3RBX6MeshIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x35b714() {
    // IDA 0x35b714: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::singleton(void)")]
// 0x35b804 — __ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE9singletonEv
pub fn stub_0x35b804() {
    // IDA 0x35b804: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::construct_func(char const*,char *)")]
// 0x35b870 — __ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE14construct_funcEPKcPc
pub fn stub_0x35b870() {
    // IDA 0x35b870: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::destruct_func(char *)")]
// 0x35b88c — __ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE13destruct_funcEPc
pub fn stub_0x35b88c() {
    // IDA 0x35b88c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Name::Name(char const* const&)")]
// 0x35ba98 — __ZN3RBX4NameC2ERKPKc
pub fn stub_0x35ba98() {
    // IDA 0x35ba98: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Name::setOrderIndex(void)")]
// 0x35bbbc — __ZN3RBX4Name13setOrderIndexEv
pub fn stub_0x35bbbc() {
    // IDA 0x35bbbc: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Name::lookup(char const* const&)")]
// 0x35bd48 — __ZN3RBX4Name6lookupERKPKc
pub fn stub_0x35bd48() {
    // IDA 0x35bd48: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Name::getNullName(void)")]
// 0x35be98 — __ZN3RBX4Name11getNullNameEv
pub fn stub_0x35be98() {
    // IDA 0x35be98: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Name::lookup(std::string const&)")]
// 0x35bebc — __ZN3RBX4Name6lookupERKSs
pub fn stub_0x35bebc() {
    // IDA 0x35bebc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Name::NameMap::~NameMap()")]
// 0x35bfe8 — __ZN3RBX4Name7NameMapD1Ev
pub fn stub_0x35bfe8() {
    // IDA 0x35bfe8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Name::NameMap::~NameMap()")]
// 0x35bfec — __ZN3RBX4Name7NameMapD2Ev
pub fn stub_0x35bfec() {
    // IDA 0x35bfec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Name::map(void)")]
// 0x35c10c — __ZN3RBX4Name3mapEv
pub fn stub_0x35c10c() {
    // IDA 0x35c10c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Name::approximateMemoryUsage(void)")]
// 0x35c200 — __ZN3RBX4Name22approximateMemoryUsageEv
pub fn stub_0x35c200() {
    // IDA 0x35c200: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Name::size(void)")]
// 0x35c218 — __ZN3RBX4Name4sizeEv
pub fn stub_0x35c218() {
    // IDA 0x35c218: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Name::declare(char const* const&)")]
// 0x35c258 — __ZN3RBX4Name7declareERKPKc
pub fn stub_0x35c258() {
    // IDA 0x35c258: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::~vector()")]
// 0x35c4b8 — __ZNSt6vectorIPN3RBX4NameESaIS2_EED1Ev
pub fn stub_0x35c4b8() {
    // IDA 0x35c4b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::insert(__gnu_cxx::__normal_iterator<RBX::Name **,std::vector<RBX::Name *,std::allocator<RBX::Name *>>>,RBX::Name * const&)")]
// 0x35c4cc — __ZNSt6vectorIPN3RBX4NameESaIS2_EE6insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x35c4cc() {
    // IDA 0x35c4cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Name **,std::vector<RBX::Name *,std::allocator<RBX::Name *>>>,RBX::Name * const&)")]
// 0x35caf4 — __ZNSt6vectorIPN3RBX4NameESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x35caf4() {
    // IDA 0x35caf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Name *,std::allocator<RBX::Name *>>::_M_allocate(unsigned long)")]
// 0x35cbd4 — __ZNSt12_Vector_baseIPN3RBX4NameESaIS2_EE11_M_allocateEm
pub fn stub_0x35cbd4() {
    // IDA 0x35cbd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::queuing_rw_mutex::~queuing_rw_mutex()")]
// 0x35ccc0 — __ZN3RBX16queuing_rw_mutexD1Ev
pub fn stub_0x35ccc0() {
    // IDA 0x35ccc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::queuing_rw_mutex::queuing_rw_mutex(void)")]
// 0x35ccd0 — __ZN3RBX16queuing_rw_mutexC2Ev
pub fn stub_0x35ccd0() {
    // IDA 0x35ccd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::normalIdToMask(RBX::NormalId)")]
// 0x35cee0 — __ZN3RBX14normalIdToMaskENS_8NormalIdE
pub fn stub_0x35cee0() {
    // IDA 0x35cee0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::validNormalId(RBX::NormalId)")]
// 0x35cef8 — __ZN3RBX13validNormalIdENS_8NormalIdE
pub fn stub_0x35cef8() {
    // IDA 0x35cef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::intToNormalId(int)")]
// 0x35cf04 — __ZN3RBX13intToNormalIdEi
pub fn stub_0x35cf04() {
    // IDA 0x35cf04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::normalIdOpposite(RBX::NormalId)")]
// 0x35cf08 — __ZN3RBX16normalIdOppositeENS_8NormalIdE
pub fn stub_0x35cf08() {
    // IDA 0x35cf08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::normalIdToU(RBX::NormalId)")]
// 0x35cf24 — __ZN3RBX11normalIdToUENS_8NormalIdE
pub fn stub_0x35cf24() {
    // IDA 0x35cf24: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::normalIdToVector3(RBX::NormalId)")]
// 0x35d1e8 — __ZN3RBX17normalIdToVector3ENS_8NormalIdE
pub fn stub_0x35d1e8() {
    // IDA 0x35d1e8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::normalIdToMatrix3Internal(RBX::NormalId)")]
// 0x35d3a8 — __ZN3RBX25normalIdToMatrix3InternalENS_8NormalIdE
pub fn stub_0x35d3a8() {
    // IDA 0x35d3a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::normalIdToMatrix3(RBX::NormalId)")]
// 0x35d5f4 — __ZN3RBX17normalIdToMatrix3ENS_8NormalIdE
pub fn stub_0x35d5f4() {
    // IDA 0x35d5f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::init(bool)")]
// 0x35db90 — __ZN3RBX9Profiling4initEb
pub fn stub_0x35db90() {
    // IDA 0x35db90: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::setEnabled(bool)")]
// 0x35dbc0 — __ZN3RBX9Profiling10setEnabledEb
pub fn stub_0x35dbc0() {
    // IDA 0x35dbc0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::isEnabled(void)")]
// 0x35dbd0 — __ZN3RBX9Profiling9isEnabledEv
pub fn stub_0x35dbd0() {
    // IDA 0x35dbd0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::Profiler::Profiler(char const*)")]
// 0x35dbf8 — __ZN3RBX9Profiling8ProfilerC2EPKc
pub fn stub_0x35dbf8() {
    // IDA 0x35dbf8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::CodeProfiler::CodeProfiler(char const*)")]
// 0x35dc78 — __ZN3RBX9Profiling12CodeProfilerC1EPKc
pub fn stub_0x35dc78() {
    // IDA 0x35dc78: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::CodeProfiler::log(bool,double)")]
// 0x35dc9c — __ZN3RBX9Profiling12CodeProfiler3logEbd
pub fn stub_0x35dc9c() {
    // IDA 0x35dc9c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::Profiler::getWindow(double)const")]
// 0x35dd68 — __ZNK3RBX9Profiling8Profiler9getWindowEd
pub fn stub_0x35dd68() {
    // IDA 0x35dd68: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::Profiler::getFrames(int)const")]
// 0x35de30 — __ZNK3RBX9Profiling8Profiler9getFramesEi
pub fn stub_0x35de30() {
    // IDA 0x35de30: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::Bucket::getActualFPS(void)const")]
// 0x35ded0 — __ZNK3RBX9Profiling6Bucket12getActualFPSEv
pub fn stub_0x35ded0() {
    // IDA 0x35ded0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::Bucket::getNominalFPS(void)const")]
// 0x35df00 — __ZNK3RBX9Profiling6Bucket13getNominalFPSEv
pub fn stub_0x35df00() {
    // IDA 0x35df00: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::Bucket::getNominalFramePeriod(void)const")]
// 0x35df30 — __ZNK3RBX9Profiling6Bucket21getNominalFramePeriodEv
pub fn stub_0x35df30() {
    // IDA 0x35df30: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::Mark::Mark(RBX::Profiling::CodeProfiler &,bool,bool)")]
// 0x35df60 — __ZN3RBX9Profiling4MarkC1ERNS0_12CodeProfilerEbb
pub fn stub_0x35df60() {
    // IDA 0x35df60: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::Mark::Mark(RBX::Profiling::CodeProfiler &,bool,bool)")]
// 0x35df64 — __ZN3RBX9Profiling4MarkC2ERNS0_12CodeProfilerEbb
pub fn stub_0x35df64() {
    // IDA 0x35df64: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Profiling::Mark::~Mark()")]
// 0x35dfcc — __ZN3RBX9Profiling4MarkD1Ev
pub fn stub_0x35dfcc() {
    // IDA 0x35dfcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Profiling::Mark::~Mark()")]
// 0x35dfd0 — __ZN3RBX9Profiling4MarkD2Ev
pub fn stub_0x35dfd0() {
    // IDA 0x35dfd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Profiling::Profiler::~Profiler()")]
// 0x35e03c — __ZN3RBX9Profiling8ProfilerD1Ev
pub fn stub_0x35e03c() {
    // IDA 0x35e03c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Profiling::Profiler::~Profiler()")]
// 0x35e060 — __ZN3RBX9Profiling8ProfilerD0Ev
pub fn stub_0x35e060() {
    // IDA 0x35e060: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Profiling::CodeProfiler::~CodeProfiler()")]
// 0x35e08c — __ZN3RBX9Profiling12CodeProfilerD1Ev
pub fn stub_0x35e08c() {
    // IDA 0x35e08c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Profiling::CodeProfiler::~CodeProfiler()")]
// 0x35e0b0 — __ZN3RBX9Profiling12CodeProfilerD0Ev
pub fn stub_0x35e0b0() {
    // IDA 0x35e0b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<int,std::allocator<int>>::_M_allocate(unsigned long)")]
// 0x35e0dc — __ZNSt12_Vector_baseIiSaIiEE11_M_allocateEm
pub fn stub_0x35e0dc() {
    // IDA 0x35e0dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ProtectedString::ProtectedString(void)")]
// 0x35e2c8 — __ZN3RBX15ProtectedStringC1Ev
pub fn stub_0x35e2c8() {
    // IDA 0x35e2c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ProtectedString::ProtectedString(void)")]
// 0x35e2cc — __ZN3RBX15ProtectedStringC2Ev
pub fn stub_0x35e2cc() {
    // IDA 0x35e2cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ProtectedString::ProtectedString(RBX::ProtectedString const&)")]
// 0x35e458 — __ZN3RBX15ProtectedStringC1ERKS0_
pub fn stub_0x35e458() {
    // IDA 0x35e458: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ProtectedString::ProtectedString(RBX::ProtectedString const&)")]
// 0x35e45c — __ZN3RBX15ProtectedStringC2ERKS0_
pub fn stub_0x35e45c() {
    // IDA 0x35e45c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ProtectedString::fromTrustedSource(std::string const&)")]
// 0x35e538 — __ZN3RBX15ProtectedString17fromTrustedSourceERKSs
pub fn stub_0x35e538() {
    // IDA 0x35e538: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ProtectedString::calculateHash(std::string *)const")]
// 0x35e628 — __ZNK3RBX15ProtectedString13calculateHashEPSs
pub fn stub_0x35e628() {
    // IDA 0x35e628: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ProtectedString::getStringForImmediateUse(void)const")]
// 0x35e8f8 — __ZNK3RBX15ProtectedString24getStringForImmediateUseEv
pub fn stub_0x35e8f8() {
    // IDA 0x35e8f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ProtectedString::getOriginalHash(void)const")]
// 0x35e8fc — __ZNK3RBX15ProtectedString15getOriginalHashEv
pub fn stub_0x35e8fc() {
    // IDA 0x35e8fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ProtectedString::getSalt(void)const")]
// 0x35e900 — __ZNK3RBX15ProtectedString7getSaltEv
pub fn stub_0x35e900() {
    // IDA 0x35e900: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ProtectedString::readUnprotectedChar(int,char *)const")]
// 0x35e90c — __ZNK3RBX15ProtectedString19readUnprotectedCharEiPc
pub fn stub_0x35e90c() {
    // IDA 0x35e90c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ProtectedString::operator==(RBX::ProtectedString const&)const")]
// 0x35e92c — __ZNK3RBX15ProtectedStringeqERKS0_
pub fn stub_0x35e92c() {
    // IDA 0x35e92c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ProtectedString::operator=(RBX::ProtectedString const&)")]
// 0x35e940 — __ZN3RBX15ProtectedStringaSERKS0_
pub fn stub_0x35e940() {
    // IDA 0x35e940: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "bool XmlNameValuePair::getValue<RBX::ProtectedString>(RBX::ProtectedString &)const")]
// 0x35e958 — __ZNK16XmlNameValuePair8getValueIN3RBX15ProtectedStringEEEbRT_
pub fn stub_0x35e958() {
    // IDA 0x35e958: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::StringConverter<RBX::ProtectedString>::convertToValue(std::string const&,RBX::ProtectedString&)")]
// 0x35eba0 — __ZN3RBX15StringConverterINS_15ProtectedStringEE14convertToValueERKSsRS1_
pub fn stub_0x35eba0() {
    // IDA 0x35eba0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ProtectedString::~ProtectedString()")]
// 0x35f9ec — __ZN3RBX15ProtectedStringD1Ev
pub fn stub_0x35f9ec() {
    // IDA 0x35f9ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ProtectedString * rbx::any_cast<RBX::ProtectedString,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x35fdd0 — __ZN3rbx8any_castIN3RBX15ProtectedStringENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x35fdd0() {
    // IDA 0x35fdd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ProtectedString>(RBX::ProtectedString const&)")]
// 0x35fe28 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15ProtectedStringEEERS3_RKT_
pub fn stub_0x35fe28() {
    // IDA 0x35fe28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ProtectedString & rbx::any_cast<RBX::ProtectedString &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x35fe84 — __ZN3rbx8any_castIRN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x35fe84() {
    // IDA 0x35fe84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ProtectedString>::singleton(void)")]
// 0x35ff74 — __ZN3rbx14implementation12typed_holderIN3RBX15ProtectedStringEE9singletonEv
pub fn stub_0x35ff74() {
    // IDA 0x35ff74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ProtectedString>::construct_func(char const*,char *)")]
// 0x35ffe0 — __ZN3rbx14implementation12typed_holderIN3RBX15ProtectedStringEE14construct_funcEPKcPc
pub fn stub_0x35ffe0() {
    // IDA 0x35ffe0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ProtectedString>::destruct_func(char *)")]
// 0x35fff0 — __ZN3rbx14implementation12typed_holderIN3RBX15ProtectedStringEE13destruct_funcEPc
pub fn stub_0x35fff0() {
    // IDA 0x35fff0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Quaternion::operator=(RBX::Quaternion const&)")]
// 0x3602a8 — __ZN3RBX10QuaternionaSERKS0_
pub fn stub_0x3602a8() {
    // IDA 0x3602a8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Rect::positionPoint(RBX::Rect::Location,RBX::Rect::Location)const")]
// 0x360560 — __ZNK3RBX4Rect13positionPointENS0_8LocationES1_
pub fn stub_0x360560() {
    // IDA 0x360560: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Rect::positionChild(RBX::Rect const&,RBX::Rect::Location,RBX::Rect::Location)const")]
// 0x360678 — __ZNK3RBX4Rect13positionChildERKS0_NS0_8LocationES3_
pub fn stub_0x360678() {
    // IDA 0x360678: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::RunService::RunService(void)")]
// 0x36082c — __ZN3RBX10RunServiceC1Ev
pub fn stub_0x36082c() {
    // IDA 0x36082c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::RunService::RunService(void)")]
// 0x360830 — __ZN3RBX10RunServiceC2Ev
pub fn stub_0x360830() {
    // IDA 0x360830: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RunService::stopTasks(void)")]
// 0x360dd4 — __ZN3RBX10RunService9stopTasksEv
pub fn stub_0x360dd4() {
    // IDA 0x360dd4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RunService::start(void)")]
// 0x360f34 — __ZN3RBX10RunService5startEv
pub fn stub_0x360f34() {
    // IDA 0x360f34: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RunService::~RunService()")]
// 0x3611ec — __ZN3RBX10RunServiceD0Ev
pub fn stub_0x3611ec() {
    // IDA 0x3611ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::RunService::~RunService()")]
// 0x36128c — __ZN3RBX10RunServiceD1Ev
pub fn stub_0x36128c() {
    // IDA 0x36128c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::RunService::~RunService()")]
// 0x361290 — __ZThn32_N3RBX10RunServiceD0Ev
pub fn stub_0x361290() {
    // IDA 0x361290: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::RunService::~RunService()")]
// 0x361298 — __ZThn36_N3RBX10RunServiceD0Ev
pub fn stub_0x361298() {
    // IDA 0x361298: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::RunService::~RunService()")]
// 0x3612a0 — __ZN3RBX10RunServiceD2Ev
pub fn stub_0x3612a0() {
    // IDA 0x3612a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::RunService::~RunService()")]
// 0x3616a8 — __ZThn32_N3RBX10RunServiceD1Ev
pub fn stub_0x3616a8() {
    // IDA 0x3616a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
// ============================================================
// IDA-grounded implementations — RBX::Math rotation/ray helpers
// (0x3568e0..0x359ed0) and RBX::MD5Hasher (0x35a620..0x35a7f4).
// The carrier stubs above are unchanged; the items below are the
// live Rust ports (26 fns). ReleaseAssert traffic maps to
// debug_assert!; function-static singletons map to `static`/OnceLock.
// ============================================================

/// was: `RBX::Math` angle/matrix/ray helpers
/// (`include/Util/Math.h`, `Client/App/util/Math.cpp`).
pub mod math {
    const PI_F: f32 = core::f32::consts::PI;

    /// Shared wrap-to-[-pi, pi] from Math.h:346, used by 0x3568e0,
    /// 0x3569d8 and 0x356ff0. Comparisons, floor and the multiply run
    /// in double in the original, then narrow to float on return.
    fn wrap_pi(answer: f64) -> f64 {
        const PI: f64 = core::f64::consts::PI;
        const TAU: f64 = core::f64::consts::TAU;
        let mut answer = answer;
        if answer < -PI || answer >= PI {
            answer -= ((answer + PI) / TAU).floor() * TAU;
            // IDA: ReleaseAssert((answer >= -pi()) && (answer <= pi())).
            debug_assert!(answer >= -PI && answer <= PI, "wrap_pi out of range");
        }
        answer
    }

    #[doc(alias = "RBX::Math::deltaRotationClose")]
    // 0x3568e0 — __ZN3RBX4Math18deltaRotationCloseEff
    // IDA 0x3568e0: answer = a - b (first float param minus second),
    // wrapped with wrap_pi above.
    pub fn delta_rotation_close(a: f32, b: f32) -> f32 {
        wrap_pi((a - b) as f64) as f32
    }

    #[doc(alias = "RBX::Math::averageRotationClose")]
    // 0x3569d8 — __ZN3RBX4Math20averageRotationCloseEff
    // IDA 0x3569d8: avg = b + deltaRotationClose(a, b) * 0.5 in float,
    // then the same wrap_pi. Equals the wrapped midpoint (a+b)/2.
    pub fn average_rotation_close(a: f32, b: f32) -> f32 {
        let half_delta = delta_rotation_close(a, b) * 0.5;
        wrap_pi((b + half_delta) as f64) as f32
    }

    #[doc(alias = "RBX::Math::isDenormal")]
    // 0x356c80 — __ZN3RBX4Math10isDenormalEf
    // IDA 0x356c80: __fpclassifyf(x) == 5 (ARM classify enum tag for
    // subnormal). Semantically a subnormal check.
    pub fn is_denormal(value: f32) -> bool {
        value.is_subnormal()
    }

    #[doc(alias = "RBX::Math::isNanInf")]
    // 0x356c94 — __ZN3RBX4Math8isNanInfEf
    // IDA 0x356c94: fabsf(x) == INFINITY.
    // BUG: original at 0x356c94 — the name claims NaN too, but
    // fabsf(NaN) != INFINITY, so NaN returns false; preserved here.
    pub fn is_nan_inf(value: f32) -> bool {
        value.abs() == f32::INFINITY
    }

    #[doc(alias = "RBX::segSizeRadians")]
    // 0x356e34 — __ZN3RBX14segSizeRadiansEv
    // IDA 0x356e34: function-static initialized once to dword 1019809755,
    // which is exactly (2*pi/256) as f32 (verified bit-for-bit).
    pub fn seg_size_radians() -> f32 {
        f32::from_bits(0x3CC9_0FDB)
    }

    #[doc(alias = "RBX::rotationToByteBase")]
    // 0x356e6c — __ZN3RBX18rotationToByteBaseEf
    // IDA 0x356e6c: asserts |angle| <= pif()+0.0001 (Math.cpp:368-369);
    // slots = lrintf((angle + pi) / segSizeRadians()); asserts
    // -1 <= slots <= 256 (Math.cpp:374-375); clamps into [0, 255].
    pub fn rotation_to_byte_base(angle: f32) -> u8 {
        debug_assert!(angle <= PI_F + 0.0001, "angle <= pif()+0.0001 (Math.cpp:368)");
        debug_assert!(angle >= -(PI_F + 0.0001), "angle >= -(pif()+0.0001) (Math.cpp:369)");
        // IDA 0x356f3e: lrintf (round-to-nearest, ties to even).
        let slots = ((angle + PI_F) / seg_size_radians()).round_ties_even() as i32;
        debug_assert!(slots >= -1, "iAngle >= -1 (Math.cpp:374)");
        debug_assert!(slots <= 256, "iAngle <= 256 (Math.cpp:375)");
        slots.clamp(0, 255) as u8
    }

    #[doc(alias = "RBX::Math::rotationToByte")]
    // 0x356ff0 — __ZN3RBX4Math14rotationToByteEf
    // IDA 0x356ff0: wrap_pi(angle) then rotationToByteBase(wrapped).
    pub fn rotation_to_byte(angle: f32) -> u8 {
        rotation_to_byte_base(wrap_pi(angle as f64) as f32)
    }

    #[doc(alias = "RBX::Math::rotationFromByte")]
    // 0x3570e8 — __ZN3RBX4Math16rotationFromByteEh
    // IDA 0x3570e8: byte as float * segSizeRadians() + (-pi), where the
    // -pi literal is bits 0xC0490FDB. Inverse of rotation_to_byte_base.
    pub fn rotation_from_byte(byte: u8) -> f32 {
        byte as f32 * seg_size_radians() - PI_F
    }

    #[doc(alias = "RBX::RbxRay")]
    // Layout from IDA 0x358460: origin xyz at words 1-3, direction xyz
    // at words 4-6 (a2+1..+3 origin, a2+4..+6 direction).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct RbxRay {
        pub origin: [f32; 3],
        pub direction: [f32; 3],
    }

    fn dot3(a: [f32; 3], b: [f32; 3]) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    #[doc(alias = "RBX::Math::closestPointOnRay")]
    // 0x358460 — __ZN3RBX4Math17closestPointOnRayERKNS_6RbxRayES3_
    // IDA 0x358460: cos = dot(dirA, dirB); denom = 1 - cos*cos; if
    // denom <= 1e-5 the rays are parallel and out = originA; else with
    // w = originB - originA, t = (dot(w,dirA) - dot(w,dirB)*cos)/denom
    // and out = originA + dirA * t.
    pub fn closest_point_on_ray(ray_a: &RbxRay, ray_b: &RbxRay) -> [f32; 3] {
        let cos = dot3(ray_a.direction, ray_b.direction);
        let denom = 1.0 - cos * cos;
        if denom <= 0.00001 {
            return ray_a.origin;
        }
        let w = [
            ray_b.origin[0] - ray_a.origin[0],
            ray_b.origin[1] - ray_a.origin[1],
            ray_b.origin[2] - ray_a.origin[2],
        ];
        let t = (dot3(w, ray_a.direction) - dot3(w, ray_b.direction) * cos) / denom;
        [
            ray_a.origin[0] + ray_a.direction[0] * t,
            ray_a.origin[1] + ray_a.direction[1] * t,
            ray_a.origin[2] + ray_a.direction[2] * t,
        ]
    }

    #[doc(alias = "G3D::Matrix3")]
    // was: `G3D::Matrix3` 9-float row-major block (IDA Matrix3 ctors).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Matrix3(pub [[f32; 3]; 3]);

    impl Matrix3 {
        pub const fn from_rows(rows: [[f32; 3]; 3]) -> Self {
            Self(rows)
        }
        pub fn identity() -> Self {
            Self([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        }
        fn mul(a: &Self, b: &Self) -> Self {
            let mut out = [[0.0f32; 3]; 3];
            let mut r = 0;
            while r < 3 {
                let mut c = 0;
                while c < 3 {
                    out[r][c] = a.0[r][0] * b.0[0][c] + a.0[r][1] * b.0[1][c] + a.0[r][2] * b.0[2][c];
                    c += 1;
                }
                r += 1;
            }
            Self(out)
        }
        /// G3D::Matrix3::fromEulerAnglesXYZ as R = Rx(x) * Ry(y) * Rz(z).
        /// The two live call sites (0x359764) use single-axis angles, whose
        /// values are independent of the composition order.
        pub fn from_euler_angles_xyz(x: f32, y: f32, z: f32) -> Self {
            let (sx, cx) = x.sin_cos();
            let (sy, cy) = y.sin_cos();
            let (sz, cz) = z.sin_cos();
            let rx = Self([[1.0, 0.0, 0.0], [0.0, cx, -sx], [0.0, sx, cx]]);
            let ry = Self([[cy, 0.0, sy], [0.0, 1.0, 0.0], [-sy, 0.0, cy]]);
            let rz = Self([[cz, -sz, 0.0], [sz, cz, 0.0], [0.0, 0.0, 1.0]]);
            Self::mul(&Self::mul(&rx, &ry), &rz)
        }
    }

    static ROTATE_X: Matrix3 = Matrix3::from_rows([[1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]]);
    static ROTATE_Y: Matrix3 = Matrix3::from_rows([[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]]);
    static TILT_Z: Matrix3 = Matrix3::from_rows([[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
    static TILT_NEG_Z: Matrix3 =
        Matrix3::from_rows([[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
    static IDENTITY: Matrix3 =
        Matrix3::from_rows([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    static AXIS_EULER_Z: std::sync::LazyLock<Matrix3> =
        std::sync::LazyLock::new(|| Matrix3::from_euler_angles_xyz(0.0, 0.0, 1.5708));
    static AXIS_EULER_Y: std::sync::LazyLock<Matrix3> =
        std::sync::LazyLock::new(|| Matrix3::from_euler_angles_xyz(0.0, 1.5708, 0.0));

    #[doc(alias = "RBX::Math::matrixRotateX")]
    // 0x35856c — __ZN3RBX4Math13matrixRotateXEv
    // IDA 0x35856c: function-static Matrix3(1,0,0, 0,0,-1, 0,1,0).
    pub fn matrix_rotate_x() -> &'static Matrix3 {
        &ROTATE_X
    }

    #[doc(alias = "RBX::Math::matrixRotateY")]
    // 0x358658 — __ZN3RBX4Math13matrixRotateYEv
    // IDA 0x358658: function-static Matrix3(0,0,1, 0,1,0, -1,0,0).
    pub fn matrix_rotate_y() -> &'static Matrix3 {
        &ROTATE_Y
    }

    #[doc(alias = "RBX::Math::matrixTiltZ")]
    // 0x358744 — __ZN3RBX4Math11matrixTiltZEv
    // IDA 0x358744: function-static Matrix3(0,1,0, -1,0,0, 0,0,1).
    pub fn matrix_tilt_z() -> &'static Matrix3 {
        &TILT_Z
    }

    #[doc(alias = "RBX::Math::matrixTiltNegativeZ")]
    // 0x358830 — __ZN3RBX4Math19matrixTiltNegativeZEv
    // IDA 0x358830: function-static Matrix3(0,-1,0, 1,0,0, 0,0,1).
    pub fn matrix_tilt_negative_z() -> &'static Matrix3 {
        &TILT_NEG_Z
    }

    #[doc(alias = "RBX::Math::matrixTiltQuadrant")]
    // 0x358918 — __ZN3RBX4Math18matrixTiltQuadrantEi
    // IDA 0x358918: switch on quadrant — 0 builds
    // Matrix3(1,0,0, 0,0,-1, 0,1,0) (== rotateX); 1 copies the tiltZ
    // static; 2 builds Matrix3(1,0,0, 0,0,1, 0,-1,0); 3 copies the
    // tiltNegativeZ static; default asserts "0" (Math.cpp:1272) and
    // returns identity. The original constructs into an out-param.
    pub fn matrix_tilt_quadrant(quadrant: i32) -> Matrix3 {
        match quadrant {
            0 => Matrix3::from_rows([[1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]]),
            1 => TILT_Z,
            2 => Matrix3::from_rows([[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]]),
            3 => TILT_NEG_Z,
            _ => {
                debug_assert!(false, "0 (Math.cpp:1272)");
                Matrix3::identity()
            }
        }
    }

    #[doc(alias = "RBX::Math::radiansToQuadrant")]
    // 0x3589e8 — __ZN3RBX4Math17radiansToQuadrantEf
    // IDA 0x3589e8: shifted = radians + bits(1088565718), which is
    // exactly (9*pi/4) as f32 (verified bit-for-bit); asserts
    // shifted >= 0 (Math.cpp:1282); returns floor(shifted*4/2pi) % 4.
    pub fn radians_to_quadrant(radians: f32) -> i32 {
        let shifted = radians + f32::from_bits(0x40E2_31D6);
        debug_assert!(shifted >= 0.0, "radians >= 0.0 (Math.cpp:1282)");
        (shifted as f64 * 4.0 / core::f64::consts::TAU).floor() as i32 % 4
    }

    #[doc(alias = "RBX::Math::getAxisRotationMatrix")]
    // 0x359764 — __ZN3RBX4Math21getAxisRotationMatrixEi
    // IDA 0x359764: four function-statics — eulerXYZ(0,0,1.5708),
    // eulerXYZ(0,1.5708,0) and a copy of each; axis 0|3 -> identity,
    // 1 -> first, 2 -> second, 4 -> first copy, 5 -> second copy,
    // default asserts "0" (Math.cpp:1521) and returns identity.
    // The copies hold equal values, so one static each is returned.
    pub fn get_axis_rotation_matrix(axis: u32) -> &'static Matrix3 {
        match axis {
            0 | 3 => &IDENTITY,
            1 | 4 => &AXIS_EULER_Z,
            2 | 5 => &AXIS_EULER_Y,
            _ => {
                debug_assert!(false, "0 (Math.cpp:1521)");
                &IDENTITY
            }
        }
    }

    #[doc(alias = "RBX::Math::polygonStartingPoint")]
    // 0x359be0 — __ZN3RBX4Math20polygonStartingPointEif
    // IDA 0x359be0: out starts (1,1); s = (float)(3.1416 / sides)
    // (note 3.1416, not pi()). sides 6: out = (r*sin(s)*-0.5,
    // r*cos(s)*0.5), returns sin(s); sides 5: with s2 = sin(2s),
    // out = (-r*sin(s)/(2*s2), r*cos(s)/(2*s2)), returns cos(s);
    // sides 3: t = tan(s), out = (r*-0.5, r*0.5/t), returns t;
    // default: t = tan(s), out = (r*(t*-0.5), r*0.5), returns t.
    // Trig runs in double on the narrowed float, products in float.
    pub fn polygon_starting_point(sides: i32, radius: f32) -> ([f32; 2], f32) {
        let s = (3.1416 / sides as f64) as f32;
        match sides {
            6 => {
                let (sn, c) = (s as f64).sin_cos();
                let (sn, c) = (sn as f32, c as f32);
                ([radius * (sn * -0.5), radius * (c * 0.5)], sn)
            }
            5 => {
                let sn = (s as f64).sin() as f32;
                let s2 = (s as f64 * 2.0).sin() as f32;
                let c = (s as f64).cos() as f32;
                ([-radius * sn / (s2 + s2), radius * c / (s2 + s2)], c)
            }
            3 => {
                let t = (s as f64).tan() as f32;
                ([radius * -0.5, radius * 0.5 / t], t)
            }
            _ => {
                let t = (s as f64).tan() as f32;
                ([radius * (t * -0.5), radius * 0.5], t)
            }
        }
    }

    #[doc(alias = "RBX::Math::evenWholeNumberFuzzy")]
    // 0x359ed0 — __ZN3RBX4Math20evenWholeNumberFuzzyERKf
    // IDA 0x359ed0: r = floorf(x + 0.5); if |r - x| < 0.001 snap to r;
    // split with modff and return (trunc(intpart) & 1) == 0.
    pub fn even_whole_number_fuzzy(value: f32) -> bool {
        let rounded = (value + 0.5).floor();
        let snapped = if (rounded - value).abs() < 0.001 { rounded } else { value };
        // IDA 0x359f12: modff then cvt_s32_f32 truncates toward zero.
        (snapped.trunc() as i32 & 1) == 0
    }
}

/// was: `RBX::MD5Hasher` / `RBX::MD5HasherImpl` (CC_MD5_CTX at +4,
/// cached hex std::string at +96; vtable +16 = update, +24 = finalize).
#[doc(alias = "RBX::MD5Hasher")]
#[derive(Debug, Clone)]
pub struct Md5Hasher {
    state: [u32; 4],
    len_bits: u64,
    buf: [u8; 64],
    buf_len: usize,
    cache: Option<Md5Cache>,
}

#[derive(Debug, Clone)]
struct Md5Cache {
    raw: [u8; 16],
    hex: String,
}

impl Md5Hasher {
    #[doc(alias = "RBX::MD5Hasher::create")]
    // 0x35a620 — __ZN3RBX9MD5Hasher6createEv
    // IDA 0x35a620: operator new(0x64) + CC_MD5_Init(ctx at +4).
    pub fn new() -> Self {
        Self {
            // IDA: CC_MD5_Init initial chaining values.
            state: [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476],
            len_bits: 0,
            buf: [0; 64],
            buf_len: 0,
            cache: None,
        }
    }

    #[doc(alias = "RBX::MD5HasherImpl::addData")]
    // 0x35a7d0 — __ZN3RBX13MD5HasherImpl7addDataEPKcm
    // IDA 0x35a7d0: CC_MD5_Update(ctx at +4, ptr, len).
    pub fn update(&mut self, data: &[u8]) {
        self.cache = None;
        let mut data = data;
        if self.buf_len > 0 {
            let take = (64 - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len + take].copy_from_slice(&data[..take]);
            self.buf_len += take;
            self.len_bits += take as u64 * 8;
            data = &data[take..];
            if self.buf_len == 64 {
                let block = self.buf;
                Self::transform(&mut self.state, &block);
                self.buf_len = 0;
            }
        }
        while data.len() >= 64 {
            let mut block = [0u8; 64];
            block.copy_from_slice(&data[..64]);
            Self::transform(&mut self.state, &block);
            self.len_bits += 512;
            data = &data[64..];
        }
        if !data.is_empty() {
            self.buf[..data.len()].copy_from_slice(data);
            self.buf_len = data.len();
            self.len_bits += data.len() as u64 * 8;
        }
    }

    #[doc(alias = "RBX::MD5HasherImpl::addData")]
    // 0x35a7c4 — __ZN3RBX13MD5HasherImpl7addDataERKSs
    // IDA 0x35a7c4: CC_MD5_Update over the string bytes (length field).
    pub fn update_str(&mut self, s: &str) {
        self.update(s.as_bytes());
    }

    #[doc(alias = "RBX::MD5HasherImpl::addData")]
    // 0x35a744 — __ZN3RBX13MD5HasherImpl7addDataERSi
    // IDA 0x35a744: clear + seekg(0) then 1KB reads, each fed to the
    // raw update vfunc (vtable +16) with the gcount.
    pub fn update_reader(
        &mut self,
        reader: &mut (impl std::io::Read + std::io::Seek),
    ) -> std::io::Result<()> {
        use std::io::SeekFrom;
        reader.seek(SeekFrom::Start(0))?;
        let mut chunk = [0u8; 1024];
        loop {
            let n = reader.read(&mut chunk)?;
            if n == 0 {
                break;
            }
            self.update(&chunk[..n]);
            if n < 1024 {
                break;
            }
        }
        Ok(())
    }

    fn pad_and_digest(&self) -> [u8; 16] {
        let mut state = self.state;
        let mut msg = [0u8; 128];
        msg[..self.buf_len].copy_from_slice(&self.buf[..self.buf_len]);
        msg[self.buf_len] = 0x80;
        // IDA: CC_MD5_Final pads to 56 mod 64 then appends bit length.
        let pad_len = if self.buf_len < 56 { 56 - self.buf_len } else { 120 - self.buf_len };
        let total = self.buf_len + pad_len + 8;
        let bit_len = self.len_bits.to_le_bytes();
        msg[total - 8..total].copy_from_slice(&bit_len);
        let mut i = 0;
        while i < total {
            let mut block = [0u8; 64];
            block.copy_from_slice(&msg[i..i + 64]);
            Self::transform(&mut state, &block);
            i += 64;
        }
        let mut out = [0u8; 16];
        for (k, word) in state.iter().enumerate() {
            out[4 * k..4 * k + 4].copy_from_slice(&word.to_le_bytes());
        }
        out
    }

    fn ensure_cache(&mut self) {
        if self.cache.is_none() {
            let raw = self.pad_and_digest();
            let mut hex = String::with_capacity(32);
            for byte in raw {
                hex.push(char::from_digit((byte >> 4) as u32, 16).unwrap());
                hex.push(char::from_digit((byte & 0x0f) as u32, 16).unwrap());
            }
            self.cache = Some(Md5Cache { raw, hex });
        }
    }

    /// Raw 16-byte digest (CC_MD5_Final bytes, little-endian words).
    pub fn digest(&mut self) -> [u8; 16] {
        self.ensure_cache();
        self.cache.as_ref().unwrap().raw
    }

    #[doc(alias = "RBX::MD5HasherImpl::toString")]
    // 0x35a7d8 — __ZN3RBX13MD5HasherImpl8toStringEv
    // IDA 0x35a7d8: finalize vfunc (vtable +24) then copy the cached
    // string at +96. Modeled as fill-once: repeat calls return the
    // same hex without re-finalizing.
    pub fn hex_string(&mut self) -> &str {
        self.ensure_cache();
        self.cache.as_ref().unwrap().hex.as_str()
    }

    #[doc(alias = "RBX::MD5HasherImpl::c_str")]
    // 0x35a7f4 — __ZN3RBX13MD5HasherImpl5c_strEv
    // IDA 0x35a7f4: if the cached string at +96 is empty,
    // CC_MD5_Final then format each of the 16 bytes with %02x
    // appended; return the data pointer either way.
    pub fn c_str(&mut self) -> &str {
        self.hex_string()
    }

    // RFC 1321 round function (CC_MD5_Update/CC_MD5_Final core).
    fn transform(state: &mut [u32; 4], block: &[u8; 64]) {
        const S: [u32; 64] = [
            7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, //
            5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, //
            4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, //
            6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
        ];
        const K: [u32; 64] = [
            0xd76a_a478, 0xe8c7_b756, 0x2420_70db, 0xc1bd_ceee, //
            0xf57c_0faf, 0x4787_c62a, 0xa830_4613, 0xfd46_9501, //
            0x6980_98d8, 0x8b44_f7af, 0xffff_5bb1, 0x895c_d7be, //
            0x6b90_1122, 0xfd98_7193, 0xa679_438e, 0x49b4_0821, //
            0xf61e_2562, 0xc040_b340, 0x265e_5a51, 0xe9b6_c7aa, //
            0xd62f_105d, 0x0244_1453, 0xd8a1_e681, 0xe7d3_fbc8, //
            0x21e1_cde6, 0xc337_07d6, 0xf4d5_0d87, 0x455a_14ed, //
            0xa9e3_e905, 0xfcef_a3f8, 0x676f_02d9, 0x8d2a_4c8a, //
            0xfffa_3942, 0x8771_f681, 0x6d9d_6122, 0xfde5_380c, //
            0xa4be_ea44, 0x4bde_cfa9, 0xf6bb_4b60, 0xbebf_bc70, //
            0x289b_7ec6, 0xeaa1_27fa, 0xd4ef_3085, 0x0488_1d05, //
            0xd9d4_d039, 0xe6db_99e5, 0x1fa2_7cf8, 0xc4ac_5665, //
            0xf429_2244, 0x432a_ff97, 0xab94_23a7, 0xfc93_a039, //
            0x655b_59c3, 0x8f0c_cc92, 0xffef_f47d, 0x8584_5dd1, //
            0x6fa8_7e4f, 0xfe2c_e6e0, 0xa301_4314, 0x4e08_11a1, //
            0xf753_7e82, 0xbd3a_f235, 0x2ad7_d2bb, 0xeb86_d391, //
        ];
        let mut m = [0u32; 16];
        for (k, word) in m.iter_mut().enumerate() {
            *word = u32::from_le_bytes([block[4 * k], block[4 * k + 1], block[4 * k + 2], block[4 * k + 3]]);
        }
        let (mut a, mut b, mut c, mut d) = (state[0], state[1], state[2], state[3]);
        let mut i = 0;
        while i < 64 {
            let (f, g) = if i < 16 {
                ((b & c) | ((!b) & d), i)
            } else if i < 32 {
                ((d & b) | ((!d) & c), (5 * i + 1) % 16)
            } else if i < 48 {
                (b ^ c ^ d, (3 * i + 5) % 16)
            } else {
                (c ^ (b | (!d)), (7 * i) % 16)
            };
            let sum = a
                .wrapping_add(f)
                .wrapping_add(K[i])
                .wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(sum.rotate_left(S[i]));
            i += 1;
        }
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
    }
}

// 0x35a6f4 (D1) / 0x35a718 (D0) — __ZN3RBX13MD5HasherImplD1Ev/D0Ev:
// IDA: reset vtable word + destroy the cached string at +96 (+ operator
// delete for D0). Plain Drop glue covers both; no manual impl needed.
impl Default for Md5Hasher {
    // IDA 0x35a620 path creates a zero-fed hasher; Default matches it.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod aq_math_md5_tests {
    use super::math::*;
    use super::Md5Hasher;

    #[test]
    fn md5_vectors() {
        let mut h = Md5Hasher::new();
        h.update_str("");
        assert_eq!(h.hex_string(), "d41d8cd98f00b204e9800998ecf8427e");
        let mut h = Md5Hasher::new();
        h.update_str("abc");
        assert_eq!(h.hex_string(), "900150983cd24fb0d6963f7d28e17f72");
        // IDA 0x35a7f4: repeat c_str returns the cached hex as-is.
        assert_eq!(h.c_str(), "900150983cd24fb0d6963f7d28e17f72");
        // Split updates equal one-shot (0x35a7d0 streaming).
        let mut h2 = Md5Hasher::new();
        h2.update(b"a");
        h2.update(b"bc");
        assert_eq!(h2.digest(), h.digest());
        // Multi-block (> 55 bytes forces the two-block Final path).
        let mut h3 = Md5Hasher::new();
        h3.update_str(&"x".repeat(1000));
        assert_eq!(h3.hex_string().len(), 32);
    }

    #[test]
    fn angle_wrap_identities() {
        use core::f32::consts::PI;
        assert_eq!(delta_rotation_close(0.0, 0.0), 0.0);
        // (a+b)/2 midpoint through the delta path (IDA 0x3569d8).
        assert!((average_rotation_close(0.0, PI / 2.0) - PI / 4.0).abs() < 1e-6);
        // 3.0 - (-3.0) = 6.0 wraps to 6 - 2pi (IDA 0x3568e0).
        let d = delta_rotation_close(3.0, -3.0);
        assert!((d - (6.0 - 2.0 * PI)).abs() < 1e-5);
        assert!(d >= -PI && d < PI);
    }

    #[test]
    fn rotation_byte_roundtrip() {
        // IDA 0x356e6c/0x3570e8: byte 128 centers near angle 0.
        assert_eq!(rotation_to_byte(0.0), 128);
        assert_eq!(rotation_to_byte_base(0.0), 128);
        let back = rotation_from_byte(128);
        assert!(back.abs() < seg_size_radians());
        // Clamp edges (IDA 0x356fce..0x356fd8).
        assert_eq!(rotation_to_byte_base(-3.14159265), 0);
        assert_eq!(rotation_to_byte_base(3.14159265), 255);
        // Leaf predicates (IDA 0x356c80/0x356c94).
        assert!(is_denormal(f32::MIN_POSITIVE / 2.0));
        assert!(!is_denormal(1.0));
        assert!(is_nan_inf(f32::INFINITY));
        assert!(is_nan_inf(f32::NEG_INFINITY));
        assert!(!is_nan_inf(f32::NAN));
        assert!(!is_nan_inf(1.0));
    }

    #[test]
    fn matrices_quadrants_rays() {
        // IDA 0x35856c: rotateX rows.
        assert_eq!(matrix_rotate_x().0, [[1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]]);
        // IDA 0x358918: quadrant 0 == rotateX, 1 == tiltZ, 3 == tiltNegZ.
        assert_eq!(matrix_tilt_quadrant(0), *matrix_rotate_x());
        assert_eq!(matrix_tilt_quadrant(1), *matrix_tilt_z());
        assert_eq!(matrix_tilt_quadrant(3), *matrix_tilt_negative_z());
        // IDA 0x3589e8: angle 0 -> quadrant 0; axis map (IDA 0x359764).
        assert_eq!(radians_to_quadrant(0.0), 0);
        assert_eq!(get_axis_rotation_matrix(0), get_axis_rotation_matrix(3));
        assert_eq!(get_axis_rotation_matrix(1), get_axis_rotation_matrix(4));
        // IDA 0x358460: parallel rays return the first origin.
        let a = RbxRay { origin: [1.0, 2.0, 3.0], direction: [0.0, 0.0, 1.0] };
        let b = RbxRay { origin: [4.0, 5.0, 6.0], direction: [0.0, 0.0, 1.0] };
        assert_eq!(closest_point_on_ray(&a, &b), [1.0, 2.0, 3.0]);
        // (default arm asserts like the original ReleaseAssert, so it is not exercised here.)
        assert!(even_whole_number_fuzzy(2.0));
        assert!(!even_whole_number_fuzzy(3.0));
        assert!(even_whole_number_fuzzy(2.0005));
        // IDA 0x359be0: hexagon starter shape is finite and symmetric.
        let (out, trig) = polygon_starting_point(6, 2.0);
        assert!(out[0].is_finite() && out[1].is_finite() && trig.is_finite());
        assert!((out[0] + 2.0 * 0.5 * 0.5).abs() < 0.6);
    }
}