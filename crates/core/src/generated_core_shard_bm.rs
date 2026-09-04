//! core shard BM — 100 core stubs EA-sorted, next uncovered after BL 0x4e5810 (strict RBX|boost|std|rbx earliest gap, after BL 0x4cdf80..0x4e5810).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x4e5810.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::MotorFeature::MotorFeature(void)")]
// 0x4e5818 — __ZN3RBX12MotorFeatureC2Ev — RBX::MotorFeature::MotorFeature(void)
pub fn stub_4e5818() {
    // IDA 0x4e5818: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::MotorFeature::render3dAdorn(RBX::Adorn *)")]
// 0x4e5a18 — __ZN3RBX12MotorFeature13render3dAdornEPNS_5AdornE — RBX::MotorFeature::render3dAdorn(RBX::Adorn *)
pub fn stub_4e5a18() {
    // IDA 0x4e5a18: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "non-virtual thunk toRBX::MotorFeature::render3dAdorn(RBX::Adorn *)")]
// 0x4e5a70 — __ZThn92_N3RBX12MotorFeature13render3dAdornEPNS_5AdornE — non-virtual thunk toRBX::MotorFeature::render3dAdorn(RBX::Adorn *)
pub fn stub_4e5a70() {
    // IDA 0x4e5a70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::VelocityMotor::VelocityMotor(void)")]
// 0x4e5c88 — __ZN3RBX13VelocityMotorC2Ev — RBX::VelocityMotor::VelocityMotor(void)
pub fn stub_4e5c88() {
    // IDA 0x4e5c88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::VelocityMotor::~VelocityMotor()")]
// 0x4e5f1c — __ZN3RBX13VelocityMotorD0Ev — RBX::VelocityMotor::~VelocityMotor()
pub fn stub_4e5f1c() {
    // IDA 0x4e5f1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::VelocityMotor::~VelocityMotor()")]
// 0x4e5fbc — __ZN3RBX13VelocityMotorD1Ev — RBX::VelocityMotor::~VelocityMotor()
pub fn stub_4e5fbc() {
    // IDA 0x4e5fbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::VelocityMotor::~VelocityMotor()")]
// 0x4e5fc0 — __ZThn32_N3RBX13VelocityMotorD0Ev — non-virtual thunk toRBX::VelocityMotor::~VelocityMotor()
pub fn stub_4e5fc0() {
    // IDA 0x4e5fc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::VelocityMotor::~VelocityMotor()")]
// 0x4e5fc8 — __ZThn36_N3RBX13VelocityMotorD0Ev — non-virtual thunk toRBX::VelocityMotor::~VelocityMotor()
pub fn stub_4e5fc8() {
    // IDA 0x4e5fc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::VelocityMotor::~VelocityMotor()")]
// 0x4e5fd0 — __ZN3RBX13VelocityMotorD2Ev — RBX::VelocityMotor::~VelocityMotor()
pub fn stub_4e5fd0() {
    // IDA 0x4e5fd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::VelocityMotor::~VelocityMotor()")]
// 0x4e6114 — __ZThn32_N3RBX13VelocityMotorD1Ev — non-virtual thunk toRBX::VelocityMotor::~VelocityMotor()
pub fn stub_4e6114() {
    // IDA 0x4e6114: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::VelocityMotor::~VelocityMotor()")]
// 0x4e611c — __ZThn36_N3RBX13VelocityMotorD1Ev — non-virtual thunk toRBX::VelocityMotor::~VelocityMotor()
pub fn stub_4e611c() {
    // IDA 0x4e611c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::VelocityMotor::setPart(int,RBX::Feature *)")]
// 0x4e6124 — __ZN3RBX13VelocityMotor7setPartEiPNS_7FeatureE — RBX::VelocityMotor::setPart(int,RBX::Feature *)
pub fn stub_4e6124() {
    // IDA 0x4e6124: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::VelocityMotor::onAncestorChanged(RBX::AncestorChanged const&)")]
// 0x4e61b0 — __ZN3RBX13VelocityMotor17onAncestorChangedERKNS_15AncestorChangedE — RBX::VelocityMotor::onAncestorChanged(RBX::AncestorChanged const&)
pub fn stub_4e61b0() {
    // IDA 0x4e61b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::VelocityMotor::onEvent_HoleAncestorChanged(void)")]
// 0x4e62ec — __ZN3RBX13VelocityMotor27onEvent_HoleAncestorChangedEv — RBX::VelocityMotor::onEvent_HoleAncestorChanged(void)
pub fn stub_4e62ec() {
    // IDA 0x4e62ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Feature::getFaceId(void)const")]
// 0x4e6dc4 — __ZNK3RBX7Feature9getFaceIdEv — RBX::Feature::getFaceId(void)const
pub fn stub_4e6dc4() {
    // IDA 0x4e6dc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Feature::getTopBottom(void)const")]
// 0x4e6dec — __ZNK3RBX7Feature12getTopBottomEv — RBX::Feature::getTopBottom(void)const
pub fn stub_4e6dec() {
    // IDA 0x4e6dec: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Feature::getLeftRight(void)const")]
// 0x4e6e14 — __ZNK3RBX7Feature12getLeftRightEv — RBX::Feature::getLeftRight(void)const
pub fn stub_4e6e14() {
    // IDA 0x4e6e14: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Feature::getInOut(void)const")]
// 0x4e6e3c — __ZNK3RBX7Feature8getInOutEv — RBX::Feature::getInOut(void)const
pub fn stub_4e6e3c() {
    // IDA 0x4e6e3c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Feature::shouldRender3dAdorn(void)const")]
// 0x4e7160 — __ZNK3RBX7Feature19shouldRender3dAdornEv — RBX::Feature::shouldRender3dAdorn(void)const
pub fn stub_4e7160() {
    // IDA 0x4e7160: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Feature::getCoordOrientation(void)const")]
// 0x4e7164 — __ZNK3RBX7Feature19getCoordOrientationEv — RBX::Feature::getCoordOrientation(void)const
pub fn stub_4e7164() {
    // IDA 0x4e7164: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "non-virtual thunk toRBX::Feature::shouldRender3dAdorn(void)const")]
// 0x4e7190 — __ZThn92_NK3RBX7Feature19shouldRender3dAdornEv — non-virtual thunk toRBX::Feature::shouldRender3dAdorn(void)const
pub fn stub_4e7190() {
    // IDA 0x4e7190: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Hole::~Hole()")]
// 0x4e7194 — __ZN3RBX4HoleD1Ev — RBX::Hole::~Hole()
pub fn stub_4e7194() {
    // IDA 0x4e7194: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Hole::~Hole()")]
// 0x4e7198 — __ZN3RBX4HoleD0Ev — RBX::Hole::~Hole()
pub fn stub_4e7198() {
    // IDA 0x4e7198: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Hole::getCoordOrientation(void)const")]
// 0x4e7248 — __ZNK3RBX4Hole19getCoordOrientationEv — RBX::Hole::getCoordOrientation(void)const
pub fn stub_4e7248() {
    // IDA 0x4e7248: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::Hole::~Hole()")]
// 0x4e724c — __ZThn32_N3RBX4HoleD1Ev — non-virtual thunk toRBX::Hole::~Hole()
pub fn stub_4e724c() {
    // IDA 0x4e724c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::Hole::~Hole()")]
// 0x4e7254 — __ZThn32_N3RBX4HoleD0Ev — non-virtual thunk toRBX::Hole::~Hole()
pub fn stub_4e7254() {
    // IDA 0x4e7254: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::Hole::~Hole()")]
// 0x4e7308 — __ZThn36_N3RBX4HoleD1Ev — non-virtual thunk toRBX::Hole::~Hole()
pub fn stub_4e7308() {
    // IDA 0x4e7308: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::Hole::~Hole()")]
// 0x4e7310 — __ZThn36_N3RBX4HoleD0Ev — non-virtual thunk toRBX::Hole::~Hole()
pub fn stub_4e7310() {
    // IDA 0x4e7310: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::MotorFeature::~MotorFeature()")]
// 0x4e73b4 — __ZN3RBX12MotorFeatureD1Ev — RBX::MotorFeature::~MotorFeature()
pub fn stub_4e73b4() {
    // IDA 0x4e73b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::MotorFeature::~MotorFeature()")]
// 0x4e73b8 — __ZN3RBX12MotorFeatureD0Ev — RBX::MotorFeature::~MotorFeature()
pub fn stub_4e73b8() {
    // IDA 0x4e73b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::MotorFeature::~MotorFeature()")]
// 0x4e7468 — __ZThn32_N3RBX12MotorFeatureD1Ev — non-virtual thunk toRBX::MotorFeature::~MotorFeature()
pub fn stub_4e7468() {
    // IDA 0x4e7468: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::MotorFeature::~MotorFeature()")]
// 0x4e7470 — __ZThn32_N3RBX12MotorFeatureD0Ev — non-virtual thunk toRBX::MotorFeature::~MotorFeature()
pub fn stub_4e7470() {
    // IDA 0x4e7470: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::MotorFeature::~MotorFeature()")]
// 0x4e7524 — __ZThn36_N3RBX12MotorFeatureD1Ev — non-virtual thunk toRBX::MotorFeature::~MotorFeature()
pub fn stub_4e7524() {
    // IDA 0x4e7524: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::MotorFeature::~MotorFeature()")]
// 0x4e752c — __ZThn36_N3RBX12MotorFeatureD0Ev — non-virtual thunk toRBX::MotorFeature::~MotorFeature()
pub fn stub_4e752c() {
    // IDA 0x4e752c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::resize(unsigned long,RBX::Feature::InOut)")]
// 0x4ebee0 — __ZNSt6vectorIN3RBX7Feature5InOutESaIS2_EE6resizeEmS2_ — std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::resize(unsigned long,RBX::Feature::InOut)
pub fn stub_4ebee0() {
    // IDA 0x4ebee0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::push_back(RBX::Feature::InOut const&)")]
// 0x4ebf14 — __ZNSt6vectorIN3RBX7Feature5InOutESaIS2_EE9push_backERKS2_ — std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::push_back(RBX::Feature::InOut const&)
pub fn stub_4ebf14() {
    // IDA 0x4ebf14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::map<RBX::Name const*,RBX::Feature::InOut,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::operator[](RBX::Name const* const&)")]
// 0x4ebf3c — __ZNSt3mapIPKN3RBX4NameENS0_7Feature5InOutESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Feature::InOut,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::operator[](RBX::Name const* const&)
pub fn stub_4ebf3c() {
    // IDA 0x4ebf3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)")]
// 0x4ebf94 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature5InOutEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)
pub fn stub_4ebf94() {
    // IDA 0x4ebf94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)")]
// 0x4ec048 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature5InOutEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)
pub fn stub_4ec048() {
    // IDA 0x4ec048: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)")]
// 0x4ec0a0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature5InOutEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)
pub fn stub_4ec0a0() {
    // IDA 0x4ec0a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::InOut*,std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>>,RBX::Feature::InOut const&)")]
// 0x4ec108 — __ZNSt6vectorIN3RBX7Feature5InOutESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::InOut*,std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>>,RBX::Feature::InOut const&)
pub fn stub_4ec108() {
    // IDA 0x4ec108: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Vector_base<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_allocate(unsigned long)")]
// 0x4ec1ec — __ZNSt12_Vector_baseIN3RBX7Feature5InOutESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_allocate(unsigned long)
pub fn stub_4ec1ec() {
    // IDA 0x4ec1ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Feature::InOut * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::InOut *,RBX::Feature::InOut *>(RBX::Feature::InOut *,RBX::Feature::InOut *,RBX::Feature::InOut *)")]
// 0x4ec204 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Feature5InOutES6_EET0_T_S8_S7_ — RBX::Feature::InOut * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::InOut *,RBX::Feature::InOut *>(RBX::Feature::InOut *,RBX::Feature::InOut *,RBX::Feature::InOut *)
pub fn stub_4ec204() {
    // IDA 0x4ec204: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::InOut*,std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>>,unsigned long,RBX::Feature::InOut const&)")]
// 0x4ec240 — __ZNSt6vectorIN3RBX7Feature5InOutESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::InOut*,std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>>,unsigned long,RBX::Feature::InOut const&)
pub fn stub_4ec240() {
    // IDA 0x4ec240: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::resize(unsigned long,RBX::Feature::LeftRight)")]
// 0x4ec3d0 — __ZNSt6vectorIN3RBX7Feature9LeftRightESaIS2_EE6resizeEmS2_ — std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::resize(unsigned long,RBX::Feature::LeftRight)
pub fn stub_4ec3d0() {
    // IDA 0x4ec3d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::push_back(RBX::Feature::LeftRight const&)")]
// 0x4ec404 — __ZNSt6vectorIN3RBX7Feature9LeftRightESaIS2_EE9push_backERKS2_ — std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::push_back(RBX::Feature::LeftRight const&)
pub fn stub_4ec404() {
    // IDA 0x4ec404: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::map<RBX::Name const*,RBX::Feature::LeftRight,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::operator[](RBX::Name const* const&)")]
// 0x4ec42c — __ZNSt3mapIPKN3RBX4NameENS0_7Feature9LeftRightESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Feature::LeftRight,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::operator[](RBX::Name const* const&)
pub fn stub_4ec42c() {
    // IDA 0x4ec42c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)")]
// 0x4ec484 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9LeftRightEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)
pub fn stub_4ec484() {
    // IDA 0x4ec484: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)")]
// 0x4ec538 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9LeftRightEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)
pub fn stub_4ec538() {
    // IDA 0x4ec538: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)")]
// 0x4ec590 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9LeftRightEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)
pub fn stub_4ec590() {
    // IDA 0x4ec590: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::LeftRight*,std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>>,RBX::Feature::LeftRight const&)")]
// 0x4ec5f8 — __ZNSt6vectorIN3RBX7Feature9LeftRightESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::LeftRight*,std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>>,RBX::Feature::LeftRight const&)
pub fn stub_4ec5f8() {
    // IDA 0x4ec5f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Vector_base<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_allocate(unsigned long)")]
// 0x4ec6dc — __ZNSt12_Vector_baseIN3RBX7Feature9LeftRightESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_allocate(unsigned long)
pub fn stub_4ec6dc() {
    // IDA 0x4ec6dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Feature::LeftRight * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::LeftRight *,RBX::Feature::LeftRight *>(RBX::Feature::LeftRight *,RBX::Feature::LeftRight *,RBX::Feature::LeftRight *)")]
// 0x4ec6f4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Feature9LeftRightES6_EET0_T_S8_S7_ — RBX::Feature::LeftRight * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::LeftRight *,RBX::Feature::LeftRight *>(RBX::Feature::LeftRight *,RBX::Feature::LeftRight *,RBX::Feature::LeftRight *)
pub fn stub_4ec6f4() {
    // IDA 0x4ec6f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::LeftRight*,std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>>,unsigned long,RBX::Feature::LeftRight const&)")]
// 0x4ec730 — __ZNSt6vectorIN3RBX7Feature9LeftRightESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::LeftRight*,std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>>,unsigned long,RBX::Feature::LeftRight const&)
pub fn stub_4ec730() {
    // IDA 0x4ec730: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::resize(unsigned long,RBX::Feature::TopBottom)")]
// 0x4ec8c0 — __ZNSt6vectorIN3RBX7Feature9TopBottomESaIS2_EE6resizeEmS2_ — std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::resize(unsigned long,RBX::Feature::TopBottom)
pub fn stub_4ec8c0() {
    // IDA 0x4ec8c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::push_back(RBX::Feature::TopBottom const&)")]
// 0x4ec8f4 — __ZNSt6vectorIN3RBX7Feature9TopBottomESaIS2_EE9push_backERKS2_ — std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::push_back(RBX::Feature::TopBottom const&)
pub fn stub_4ec8f4() {
    // IDA 0x4ec8f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::map<RBX::Name const*,RBX::Feature::TopBottom,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::operator[](RBX::Name const* const&)")]
// 0x4ec91c — __ZNSt3mapIPKN3RBX4NameENS0_7Feature9TopBottomESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::Feature::TopBottom,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::operator[](RBX::Name const* const&)
pub fn stub_4ec91c() {
    // IDA 0x4ec91c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)")]
// 0x4ec974 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9TopBottomEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)
pub fn stub_4ec974() {
    // IDA 0x4ec974: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)")]
// 0x4eca28 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9TopBottomEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)
pub fn stub_4eca28() {
    // IDA 0x4eca28: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)")]
// 0x4eca80 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9TopBottomEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)
pub fn stub_4eca80() {
    // IDA 0x4eca80: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::TopBottom*,std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>>,RBX::Feature::TopBottom const&)")]
// 0x4ecae8 — __ZNSt6vectorIN3RBX7Feature9TopBottomESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::TopBottom*,std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>>,RBX::Feature::TopBottom const&)
pub fn stub_4ecae8() {
    // IDA 0x4ecae8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Vector_base<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_allocate(unsigned long)")]
// 0x4ecbcc — __ZNSt12_Vector_baseIN3RBX7Feature9TopBottomESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_allocate(unsigned long)
pub fn stub_4ecbcc() {
    // IDA 0x4ecbcc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Feature::TopBottom * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::TopBottom *,RBX::Feature::TopBottom *>(RBX::Feature::TopBottom *,RBX::Feature::TopBottom *,RBX::Feature::TopBottom *)")]
// 0x4ecbe4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Feature9TopBottomES6_EET0_T_S8_S7_ — RBX::Feature::TopBottom * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::TopBottom *,RBX::Feature::TopBottom *>(RBX::Feature::TopBottom *,RBX::Feature::TopBottom *,RBX::Feature::TopBottom *)
pub fn stub_4ecbe4() {
    // IDA 0x4ecbe4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::TopBottom*,std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>>,unsigned long,RBX::Feature::TopBottom const&)")]
// 0x4ecc20 — __ZNSt6vectorIN3RBX7Feature9TopBottomESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::TopBottom*,std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>>,unsigned long,RBX::Feature::TopBottom const&)
pub fn stub_4ecc20() {
    // IDA 0x4ecc20: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::FileMesh::FileMesh(void)")]
// 0x4edc88 — __ZN3RBX8FileMeshC1Ev — RBX::FileMesh::FileMesh(void)
pub fn stub_4edc88() {
    // IDA 0x4edc88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::FileMesh::FileMesh(void)")]
// 0x4edc8c — __ZN3RBX8FileMeshC2Ev — RBX::FileMesh::FileMesh(void)
pub fn stub_4edc8c() {
    // IDA 0x4edc8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::FileMesh::setMeshId(RBX::MeshId const&)")]
// 0x4ede2c — __ZN3RBX8FileMesh9setMeshIdERKNS_6MeshIdE — RBX::FileMesh::setMeshId(RBX::MeshId const&)
pub fn stub_4ede2c() {
    // IDA 0x4ede2c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::FileMesh::setTextureId(RBX::TextureId const&)")]
// 0x4ede6c — __ZN3RBX8FileMesh12setTextureIdERKNS_9TextureIdE — RBX::FileMesh::setTextureId(RBX::TextureId const&)
pub fn stub_4ede6c() {
    // IDA 0x4ede6c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::FileMesh::getMeshId(void)const")]
// 0x4edeac — __ZNK3RBX8FileMesh9getMeshIdEv — RBX::FileMesh::getMeshId(void)const
pub fn stub_4edeac() {
    // IDA 0x4edeac: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::FileMesh::getTextureId(void)const")]
// 0x4eded4 — __ZNK3RBX8FileMesh12getTextureIdEv — RBX::FileMesh::getTextureId(void)const
pub fn stub_4eded4() {
    // IDA 0x4eded4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::MeshId const& rbx::any_cast<RBX::MeshId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4eeb18 — __ZN3rbx8any_castIRKN3RBX6MeshIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::MeshId const& rbx::any_cast<RBX::MeshId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4eeb18() {
    // IDA 0x4eeb18: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "RBX::FilterInvisibleNonColliding::FilterInvisibleNonColliding(void)")]
// 0x4eef38 — __ZN3RBX27FilterInvisibleNonCollidingC1Ev — RBX::FilterInvisibleNonColliding::FilterInvisibleNonColliding(void)
pub fn stub_4eef38() {
    // IDA 0x4eef38: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "RBX::FilterInvisibleNonColliding::filterResult(RBX::Primitive const*)const")]
// 0x4eef48 — __ZNK3RBX27FilterInvisibleNonColliding12filterResultEPKNS_9PrimitiveE — RBX::FilterInvisibleNonColliding::filterResult(RBX::Primitive const*)const
pub fn stub_4eef48() {
    // IDA 0x4eef48: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "RBX::PartByLocalCharacter::filterResult(RBX::Primitive const*)const")]
// 0x4ef0f4 — __ZNK3RBX20PartByLocalCharacter12filterResultEPKNS_9PrimitiveE — RBX::PartByLocalCharacter::filterResult(RBX::Primitive const*)const
pub fn stub_4ef0f4() {
    // IDA 0x4ef0f4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "RBX::UnlockedPartByLocalCharacter::filterResult(RBX::Primitive const*)const")]
// 0x4ef164 — __ZNK3RBX28UnlockedPartByLocalCharacter12filterResultEPKNS_9PrimitiveE — RBX::UnlockedPartByLocalCharacter::filterResult(RBX::Primitive const*)const
pub fn stub_4ef164() {
    // IDA 0x4ef164: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}


#[doc(alias = "RBX::FilterDescendents::filterResult(RBX::Primitive const*)const")]
// 0x4ef260 — __ZNK3RBX17FilterDescendents12filterResultEPKNS_9PrimitiveE — RBX::FilterDescendents::filterResult(RBX::Primitive const*)const
pub fn stub_4ef260() {
    // IDA 0x4ef260: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::FilterDescendentsList::filterResult(RBX::Primitive const*)const")]
// 0x4ef2a0 — __ZNK3RBX21FilterDescendentsList12filterResultEPKNS_9PrimitiveE — RBX::FilterDescendentsList::filterResult(RBX::Primitive const*)const
pub fn stub_4ef2a0() {
    // IDA 0x4ef2a0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::FilterCharacterOcclusion::FilterCharacterOcclusion(float)")]
// 0x4ef2e0 — __ZN3RBX24FilterCharacterOcclusionC1Ef — RBX::FilterCharacterOcclusion::FilterCharacterOcclusion(float)
pub fn stub_4ef2e0() {
    // IDA 0x4ef2e0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::FilterCharacterOcclusion::filterResult(RBX::Primitive const*)const")]
// 0x4ef2f4 — __ZNK3RBX24FilterCharacterOcclusion12filterResultEPKNS_9PrimitiveE — RBX::FilterCharacterOcclusion::filterResult(RBX::Primitive const*)const
pub fn stub_4ef2f4() {
    // IDA 0x4ef2f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::FilterHumanoidParts::filterResult(RBX::Primitive const*)const")]
// 0x4ef388 — __ZNK3RBX19FilterHumanoidParts12filterResultEPKNS_9PrimitiveE — RBX::FilterHumanoidParts::filterResult(RBX::Primitive const*)const
pub fn stub_4ef388() {
    // IDA 0x4ef388: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::MergedFilter::MergedFilter(RBX::HitTestFilter const*,RBX::HitTestFilter const*)")]
// 0x4ef3a4 — __ZN3RBX12MergedFilterC1EPKNS_13HitTestFilterES3_ — RBX::MergedFilter::MergedFilter(RBX::HitTestFilter const*,RBX::HitTestFilter const*)
pub fn stub_4ef3a4() {
    // IDA 0x4ef3a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::MergedFilter::filterResult(RBX::Primitive const*)const")]
// 0x4ef3b8 — __ZNK3RBX12MergedFilter12filterResultEPKNS_9PrimitiveE — RBX::MergedFilter::filterResult(RBX::Primitive const*)const
pub fn stub_4ef3b8() {
    // IDA 0x4ef3b8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::FilterInvisibleNonColliding::~FilterInvisibleNonColliding()")]
// 0x4ef424 — __ZN3RBX27FilterInvisibleNonCollidingD1Ev — RBX::FilterInvisibleNonColliding::~FilterInvisibleNonColliding()
pub fn stub_4ef424() {
    // IDA 0x4ef424: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::FilterInvisibleNonColliding::~FilterInvisibleNonColliding()")]
// 0x4ef428 — __ZN3RBX27FilterInvisibleNonCollidingD0Ev — RBX::FilterInvisibleNonColliding::~FilterInvisibleNonColliding()
pub fn stub_4ef428() {
    // IDA 0x4ef428: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::FilterDescendentsList::~FilterDescendentsList()")]
// 0x4ef42c — __ZN3RBX21FilterDescendentsListD1Ev — RBX::FilterDescendentsList::~FilterDescendentsList()
pub fn stub_4ef42c() {
    // IDA 0x4ef42c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::FilterDescendentsList::~FilterDescendentsList()")]
// 0x4ef430 — __ZN3RBX21FilterDescendentsListD0Ev — RBX::FilterDescendentsList::~FilterDescendentsList()
pub fn stub_4ef430() {
    // IDA 0x4ef430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::FilterCharacterOcclusion::~FilterCharacterOcclusion()")]
// 0x4ef434 — __ZN3RBX24FilterCharacterOcclusionD1Ev — RBX::FilterCharacterOcclusion::~FilterCharacterOcclusion()
pub fn stub_4ef434() {
    // IDA 0x4ef434: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::FilterCharacterOcclusion::~FilterCharacterOcclusion()")]
// 0x4ef438 — __ZN3RBX24FilterCharacterOcclusionD0Ev — RBX::FilterCharacterOcclusion::~FilterCharacterOcclusion()
pub fn stub_4ef438() {
    // IDA 0x4ef438: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::MergedFilter::~MergedFilter()")]
// 0x4ef43c — __ZN3RBX12MergedFilterD1Ev — RBX::MergedFilter::~MergedFilter()
pub fn stub_4ef43c() {
    // IDA 0x4ef43c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::MergedFilter::~MergedFilter()")]
// 0x4ef440 — __ZN3RBX12MergedFilterD0Ev — RBX::MergedFilter::~MergedFilter()
pub fn stub_4ef440() {
    // IDA 0x4ef440: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::FilterHumanoidParts::~FilterHumanoidParts()")]
// 0x4ef444 — __ZN3RBX19FilterHumanoidPartsD1Ev — RBX::FilterHumanoidParts::~FilterHumanoidParts()
pub fn stub_4ef444() {
    // IDA 0x4ef444: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::FilterHumanoidParts::~FilterHumanoidParts()")]
// 0x4ef448 — __ZN3RBX19FilterHumanoidPartsD0Ev — RBX::FilterHumanoidParts::~FilterHumanoidParts()
pub fn stub_4ef448() {
    // IDA 0x4ef448: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Fire::setSizeUi(float)")]
// 0x4ef7c0 — __ZN3RBX4Fire9setSizeUiEf — RBX::Fire::setSizeUi(float)
pub fn stub_4ef7c0() {
    // IDA 0x4ef7c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Fire::setHeatUi(float)")]
// 0x4ef80c — __ZN3RBX4Fire9setHeatUiEf — RBX::Fire::setHeatUi(float)
pub fn stub_4ef80c() {
    // IDA 0x4ef80c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Fire::setSize(float)")]
// 0x4ef858 — __ZN3RBX4Fire7setSizeEf — RBX::Fire::setSize(float)
pub fn stub_4ef858() {
    // IDA 0x4ef858: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Fire::setHeat(float)")]
// 0x4ef898 — __ZN3RBX4Fire7setHeatEf — RBX::Fire::setHeat(float)
pub fn stub_4ef898() {
    // IDA 0x4ef898: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Fire::Fire(void)")]
// 0x4ef8d8 — __ZN3RBX4FireC2Ev — RBX::Fire::Fire(void)
pub fn stub_4ef8d8() {
    // IDA 0x4ef8d8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "RBX::Fire::~Fire()")]
// 0x4efaf4 — __ZN3RBX4FireD0Ev — RBX::Fire::~Fire()
pub fn stub_4efaf4() {
    // IDA 0x4efaf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Fire::~Fire()")]
// 0x4efb94 — __ZN3RBX4FireD1Ev — RBX::Fire::~Fire()
pub fn stub_4efb94() {
    // IDA 0x4efb94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "non-virtual thunk toRBX::Fire::~Fire()")]
// 0x4efb98 — __ZThn32_N3RBX4FireD0Ev — non-virtual thunk toRBX::Fire::~Fire()
pub fn stub_4efb98() {
    // IDA 0x4efb98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

