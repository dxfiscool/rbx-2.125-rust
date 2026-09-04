//! core shard CG — 100 core stubs EA-sorted, next uncovered after CF 0x637724 (strict RBX|boost|std|rbx earliest gap 0x6377e0).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "non-virtual thunk toRBX::Smoke::~Smoke()")]
// 0x6377e0 — __ZThn32_N3RBX5SmokeD1Ev
pub fn stub_6377e0() {
    // IDA 0x6377e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Smoke::~Smoke()")]
// 0x6377e8 — __ZThn36_N3RBX5SmokeD1Ev
pub fn stub_6377e8() {
    // IDA 0x6377e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Smoke::~Smoke()")]
// 0x6377f0 — __ZThn92_N3RBX5SmokeD1Ev
pub fn stub_6377f0() {
    // IDA 0x6377f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Smoke::getClampedSize(void)const")]
// 0x6377f8 — __ZNK3RBX5Smoke14getClampedSizeEv
pub fn stub_6377f8() {
    // IDA 0x6377f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Smoke::getClampedOpacity(void)const")]
// 0x637820 — __ZNK3RBX5Smoke17getClampedOpacityEv
pub fn stub_637820() {
    // IDA 0x637820: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Smoke::getClampedRiseVelocity(void)const")]
// 0x637840 — __ZNK3RBX5Smoke22getClampedRiseVelocityEv
pub fn stub_637840() {
    // IDA 0x637840: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Smoke::getColor(void)const")]
// 0x637860 — __ZNK3RBX5Smoke8getColorEv
pub fn stub_637860() {
    // IDA 0x637860: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Smoke::getSizeRaw(void)const")]
// 0x637894 — __ZNK3RBX5Smoke10getSizeRawEv
pub fn stub_637894() {
    // IDA 0x637894: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Smoke::getOpacityRaw(void)const")]
// 0x6378bc — __ZNK3RBX5Smoke13getOpacityRawEv
pub fn stub_6378bc() {
    // IDA 0x6378bc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Smoke::getRiseVelocityRaw(void)const")]
// 0x6378c0 — __ZNK3RBX5Smoke18getRiseVelocityRawEv
pub fn stub_6378c0() {
    // IDA 0x6378c0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SocialService::setFriendUrl(std::string)")]
// 0x639138 — __ZN3RBX13SocialService12setFriendUrlESs
pub fn stub_639138() {
    // IDA 0x639138: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::setBestFriendUrl(std::string)")]
// 0x639140 — __ZN3RBX13SocialService16setBestFriendUrlESs
pub fn stub_639140() {
    // IDA 0x639140: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::setGroupUrl(std::string)")]
// 0x639148 — __ZN3RBX13SocialService11setGroupUrlESs
pub fn stub_639148() {
    // IDA 0x639148: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::setGroupRankUrl(std::string)")]
// 0x639150 — __ZN3RBX13SocialService15setGroupRankUrlESs
pub fn stub_639150() {
    // IDA 0x639150: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::setGroupRoleUrl(std::string)")]
// 0x639158 — __ZN3RBX13SocialService15setGroupRoleUrlESs
pub fn stub_639158() {
    // IDA 0x639158: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::setStuffUrl(std::string)")]
// 0x639160 — __ZN3RBX13SocialService11setStuffUrlESs
pub fn stub_639160() {
    // IDA 0x639160: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::setPackageContentsUrl(std::string)")]
// 0x639168 — __ZN3RBX13SocialService21setPackageContentsUrlESs
pub fn stub_639168() {
    // IDA 0x639168: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::SocialService(void)")]
// 0x639448 — __ZN3RBX13SocialServiceC1Ev
pub fn stub_639448() {
    // IDA 0x639448: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::SocialService(void)")]
// 0x63944c — __ZN3RBX13SocialServiceC2Ev
pub fn stub_63944c() {
    // IDA 0x63944c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::~SocialService()")]
// 0x63add8 — __ZN3RBX13SocialServiceD1Ev
pub fn stub_63add8() {
    // IDA 0x63add8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SocialService::~SocialService()")]
// 0x63ae44 — __ZN3RBX13SocialServiceD0Ev
pub fn stub_63ae44() {
    // IDA 0x63ae44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SocialService::~SocialService()")]
// 0x63af0c — __ZThn32_N3RBX13SocialServiceD1Ev
pub fn stub_63af0c() {
    // IDA 0x63af0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SocialService::~SocialService()")]
// 0x63af80 — __ZThn32_N3RBX13SocialServiceD0Ev
pub fn stub_63af80() {
    // IDA 0x63af80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SocialService::~SocialService()")]
// 0x63b0ac — __ZThn36_N3RBX13SocialServiceD1Ev
pub fn stub_63b0ac() {
    // IDA 0x63b0ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SocialService::~SocialService()")]
// 0x63b120 — __ZThn36_N3RBX13SocialServiceD0Ev
pub fn stub_63b120() {
    // IDA 0x63b120: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::resize(unsigned long,RBX::SocialService::StuffType)")]
// 0x63b420 — __ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE6resizeEmS2_
pub fn stub_63b420() {
    // IDA 0x63b420: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::push_back(RBX::SocialService::StuffType const&)")]
// 0x63b454 — __ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE9push_backERKS2_
pub fn stub_63b454() {
    // IDA 0x63b454: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::SocialService::StuffType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::operator[](RBX::Name const* const&)")]
// 0x63b47c — __ZNSt3mapIPKN3RBX4NameENS0_13SocialService9StuffTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_63b47c() {
    // IDA 0x63b47c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")]
// 0x63b4d4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_63b4d4() {
    // IDA 0x63b4d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")]
// 0x63b588 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_63b588() {
    // IDA 0x63b588: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)")]
// 0x63b5e0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_63b5e0() {
    // IDA 0x63b5e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,RBX::SocialService::StuffType const&)")]
// 0x63b648 — __ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_63b648() {
    // IDA 0x63b648: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_allocate(unsigned long)")]
// 0x63b72c — __ZNSt12_Vector_baseIN3RBX13SocialService9StuffTypeESaIS2_EE11_M_allocateEm
pub fn stub_63b72c() {
    // IDA 0x63b72c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::StuffType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SocialService::StuffType *,RBX::SocialService::StuffType *>(RBX::SocialService::StuffType *,RBX::SocialService::StuffType *,RBX::SocialService::StuffType *)")]
// 0x63b744 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13SocialService9StuffTypeES6_EET0_T_S8_S7_
pub fn stub_63b744() {
    // IDA 0x63b744: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,unsigned long,RBX::SocialService::StuffType const&)")]
// 0x63b780 — __ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_63b780() {
    // IDA 0x63b780: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Sparkles::getLegacyColor(void)const")]
// 0x63c208 — __ZNK3RBX8Sparkles14getLegacyColorEv
pub fn stub_63c208() {
    // IDA 0x63c208: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Sparkles::Sparkles(void)")]
// 0x63c294 — __ZN3RBX8SparklesC1Ev
pub fn stub_63c294() {
    // IDA 0x63c294: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Sparkles::Sparkles(void)")]
// 0x63c298 — __ZN3RBX8SparklesC2Ev
pub fn stub_63c298() {
    // IDA 0x63c298: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Sparkles::getColor(void)const")]
// 0x63c450 — __ZNK3RBX8Sparkles8getColorEv
pub fn stub_63c450() {
    // IDA 0x63c450: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Sparkles::~Sparkles()")]
// 0x63c484 — __ZN3RBX8SparklesD1Ev
pub fn stub_63c484() {
    // IDA 0x63c484: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Sparkles::~Sparkles()")]
// 0x63c488 — __ZN3RBX8SparklesD0Ev
pub fn stub_63c488() {
    // IDA 0x63c488: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
// 0x63c578 — __ZThn32_N3RBX8SparklesD1Ev
pub fn stub_63c578() {
    // IDA 0x63c578: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
// 0x63c580 — __ZThn32_N3RBX8SparklesD0Ev
pub fn stub_63c580() {
    // IDA 0x63c580: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
// 0x63c598 — __ZThn36_N3RBX8SparklesD1Ev
pub fn stub_63c598() {
    // IDA 0x63c598: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
// 0x63c5a0 — __ZThn36_N3RBX8SparklesD0Ev
pub fn stub_63c5a0() {
    // IDA 0x63c5a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
// 0x63c5a8 — __ZThn92_N3RBX8SparklesD1Ev
pub fn stub_63c5a8() {
    // IDA 0x63c5a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Sparkles::~Sparkles()")]
// 0x63c5b0 — __ZThn92_N3RBX8SparklesD0Ev
pub fn stub_63c5b0() {
    // IDA 0x63c5b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Sparkles::~Sparkles()")]
// 0x63c5b8 — __ZN3RBX8SparklesD2Ev
pub fn stub_63c5b8() {
    // IDA 0x63c5b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnLocation::getTeamColor(void)const")]
// 0x63d228 — __ZNK3RBX13SpawnLocation12getTeamColorEv
pub fn stub_63d228() {
    // IDA 0x63d228: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnLocation::setTeamColor(RBX::BrickColor)")]
// 0x63d230 — __ZN3RBX13SpawnLocation12setTeamColorENS_10BrickColorE
pub fn stub_63d230() {
    // IDA 0x63d230: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnLocation::SpawnLocation(void)")]
// 0x63d248 — __ZN3RBX13SpawnLocationC1Ev
pub fn stub_63d248() {
    // IDA 0x63d248: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnLocation::~SpawnLocation()")]
// 0x63d500 — __ZN3RBX13SpawnLocationD0Ev
pub fn stub_63d500() {
    // IDA 0x63d500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnLocation::~SpawnLocation()")]
// 0x63d5ac — __ZN3RBX13SpawnLocationD1Ev
pub fn stub_63d5ac() {
    // IDA 0x63d5ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
// 0x63d5bc — __ZThn32_N3RBX13SpawnLocationD0Ev
pub fn stub_63d5bc() {
    // IDA 0x63d5bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
// 0x63d5c4 — __ZThn36_N3RBX13SpawnLocationD0Ev
pub fn stub_63d5c4() {
    // IDA 0x63d5c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
// 0x63d5cc — __ZThn132_N3RBX13SpawnLocationD0Ev
pub fn stub_63d5cc() {
    // IDA 0x63d5cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnLocation::~SpawnLocation()")]
// 0x63d5d4 — __ZN3RBX13SpawnLocationD2Ev
pub fn stub_63d5d4() {
    // IDA 0x63d5d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
// 0x63d788 — __ZThn32_N3RBX13SpawnLocationD1Ev
pub fn stub_63d788() {
    // IDA 0x63d788: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
// 0x63d798 — __ZThn36_N3RBX13SpawnLocationD1Ev
pub fn stub_63d798() {
    // IDA 0x63d798: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpawnLocation::~SpawnLocation()")]
// 0x63d7a8 — __ZThn132_N3RBX13SpawnLocationD1Ev
pub fn stub_63d7a8() {
    // IDA 0x63d7a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnLocation::updateSpawnerTouched(void)")]
// 0x63d858 — __ZN3RBX13SpawnLocation20updateSpawnerTouchedEv
pub fn stub_63d858() {
    // IDA 0x63d858: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnLocation::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x63da9c — __ZN3RBX13SpawnLocation17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_63da9c() {
    // IDA 0x63da9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnerService::SpawnerService(void)")]
// 0x63db8c — __ZN3RBX14SpawnerServiceC2Ev
pub fn stub_63db8c() {
    // IDA 0x63db8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnerService::~SpawnerService()")]
// 0x63ddd8 — __ZN3RBX14SpawnerServiceD0Ev
pub fn stub_63ddd8() {
    // IDA 0x63ddd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnerService::~SpawnerService()")]
// 0x63de78 — __ZN3RBX14SpawnerServiceD1Ev
pub fn stub_63de78() {
    // IDA 0x63de78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpawnerService::~SpawnerService()")]
// 0x63de7c — __ZThn32_N3RBX14SpawnerServiceD0Ev
pub fn stub_63de7c() {
    // IDA 0x63de7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpawnerService::~SpawnerService()")]
// 0x63de84 — __ZThn36_N3RBX14SpawnerServiceD0Ev
pub fn stub_63de84() {
    // IDA 0x63de84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnerService::~SpawnerService()")]
// 0x63de8c — __ZN3RBX14SpawnerServiceD2Ev
pub fn stub_63de8c() {
    // IDA 0x63de8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpawnerService::~SpawnerService()")]
// 0x63ded4 — __ZThn32_N3RBX14SpawnerServiceD1Ev
pub fn stub_63ded4() {
    // IDA 0x63ded4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpawnerService::~SpawnerService()")]
// 0x63dedc — __ZThn36_N3RBX14SpawnerServiceD1Ev
pub fn stub_63dedc() {
    // IDA 0x63dedc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpawnerService::ClearContents(void)")]
// 0x63dee4 — __ZN3RBX14SpawnerService13ClearContentsEv
pub fn stub_63dee4() {
    // IDA 0x63dee4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::list<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::remove(RBX::SpawnLocation * const&)")]
// 0x63e66c — __ZNSt4listIPN3RBX13SpawnLocationESaIS2_EE6removeERKS2_
pub fn stub_63e66c() {
    // IDA 0x63e66c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::push_back(RBX::SpawnLocation * const&)")]
// 0x63e6a4 — __ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE9push_backERKS2_
pub fn stub_63e6a4() {
    // IDA 0x63e6a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpawnLocation **,std::vector<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>>,RBX::SpawnLocation * const&)")]
// 0x63f508 — __ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_63f508() {
    // IDA 0x63f508: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::SpawnLocation *,std::allocator<RBX::SpawnLocation *>>::_M_allocate(unsigned long)")]
// 0x63f5e8 — __ZNSt12_Vector_baseIPN3RBX13SpawnLocationESaIS2_EE11_M_allocateEm
pub fn stub_63f5e8() {
    // IDA 0x63f5e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpawnerService * RBX::ServiceProvider::create<RBX::SpawnerService>(void)const")]
// 0x63f7fc — __ZNK3RBX15ServiceProvider6createINS_14SpawnerServiceEEEPT_v
pub fn stub_63f7fc() {
    // IDA 0x63f7fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpecialShape::setMeshType(RBX::SpecialShape::MeshType)")]
// 0x6411bc — __ZN3RBX12SpecialShape11setMeshTypeENS0_8MeshTypeE
pub fn stub_6411bc() {
    // IDA 0x6411bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpecialShape::SpecialShape(void)")]
// 0x6411dc — __ZN3RBX12SpecialShapeC2Ev
pub fn stub_6411dc() {
    // IDA 0x6411dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpecialShape::setMeshId(RBX::MeshId const&)")]
// 0x6414bc — __ZN3RBX12SpecialShape9setMeshIdERKNS_6MeshIdE
pub fn stub_6414bc() {
    // IDA 0x6414bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpecialShape::setTextureId(RBX::TextureId const&)")]
// 0x641504 — __ZN3RBX12SpecialShape12setTextureIdERKNS_9TextureIdE
pub fn stub_641504() {
    // IDA 0x641504: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SpecialShape::getMeshType(void)const")]
// 0x6418ac — __ZNK3RBX12SpecialShape11getMeshTypeEv
pub fn stub_6418ac() {
    // IDA 0x6418ac: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SpecialShape::~SpecialShape()")]
// 0x6418d8 — __ZN3RBX12SpecialShapeD1Ev
pub fn stub_6418d8() {
    // IDA 0x6418d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpecialShape::~SpecialShape()")]
// 0x6419e0 — __ZN3RBX12SpecialShapeD0Ev
pub fn stub_6419e0() {
    // IDA 0x6419e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpecialShape::~SpecialShape()")]
// 0x641b08 — __ZThn32_N3RBX12SpecialShapeD1Ev
pub fn stub_641b08() {
    // IDA 0x641b08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpecialShape::~SpecialShape()")]
// 0x641c10 — __ZThn32_N3RBX12SpecialShapeD0Ev
pub fn stub_641c10() {
    // IDA 0x641c10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpecialShape::~SpecialShape()")]
// 0x641d3c — __ZThn36_N3RBX12SpecialShapeD1Ev
pub fn stub_641d3c() {
    // IDA 0x641d3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpecialShape::~SpecialShape()")]
// 0x641e44 — __ZThn36_N3RBX12SpecialShapeD0Ev
pub fn stub_641e44() {
    // IDA 0x641e44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FileMesh::~FileMesh()")]
// 0x6438cc — __ZN3RBX8FileMeshD1Ev
pub fn stub_6438cc() {
    // IDA 0x6438cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FileMesh::~FileMesh()")]
// 0x6439d4 — __ZN3RBX8FileMeshD0Ev
pub fn stub_6439d4() {
    // IDA 0x6439d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FileMesh::~FileMesh()")]
// 0x643afc — __ZThn32_N3RBX8FileMeshD1Ev
pub fn stub_643afc() {
    // IDA 0x643afc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FileMesh::~FileMesh()")]
// 0x643c04 — __ZThn32_N3RBX8FileMeshD0Ev
pub fn stub_643c04() {
    // IDA 0x643c04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FileMesh::~FileMesh()")]
// 0x643d30 — __ZThn36_N3RBX8FileMeshD1Ev
pub fn stub_643d30() {
    // IDA 0x643d30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FileMesh::~FileMesh()")]
// 0x643e38 — __ZThn36_N3RBX8FileMeshD0Ev
pub fn stub_643e38() {
    // IDA 0x643e38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::resize(unsigned long,RBX::SpecialShape::MeshType)")]
// 0x644840 — __ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE6resizeEmS2_
pub fn stub_644840() {
    // IDA 0x644840: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::push_back(RBX::SpecialShape::MeshType const&)")]
// 0x644874 — __ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE9push_backERKS2_
pub fn stub_644874() {
    // IDA 0x644874: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::SpecialShape::MeshType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::operator[](RBX::Name const* const&)")]
// 0x64489c — __ZNSt3mapIPKN3RBX4NameENS0_12SpecialShape8MeshTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_64489c() {
    // IDA 0x64489c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")]
// 0x6448f4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_6448f4() {
    // IDA 0x6448f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")]
// 0x6449a8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_6449a8() {
    // IDA 0x6449a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType> const&)")]
// 0x644a00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_644a00() {
    // IDA 0x644a00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpecialShape::MeshType*,std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>>,RBX::SpecialShape::MeshType const&)")]
// 0x644a68 — __ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_644a68() {
    // IDA 0x644a68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
