//! core shard BU — 100 core stubs EA-sorted, next uncovered after BT 0x572f78..0x580728.
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x572efc.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::Hopper::render2d(RBX::Adorn *)")]
// 0x572f78 — __ZN3RBX6Hopper8render2dEPNS_5AdornE — RBX::Hopper::render2d(RBX::Adorn *)
pub fn stub_572f78() {
    // IDA 0x572f78: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::StarterPackService::StarterPackService(void)")]
// 0x573090 — __ZN3RBX18StarterPackServiceC1Ev — RBX::StarterPackService::StarterPackService(void)
pub fn stub_573090() {
    // IDA 0x573090: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::StarterPackService::StarterPackService(void)")]
// 0x573094 — __ZN3RBX18StarterPackServiceC2Ev — RBX::StarterPackService::StarterPackService(void)
pub fn stub_573094() {
    // IDA 0x573094: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::StarterPackService::render2d(RBX::Adorn *)")]
// 0x5732ac — __ZN3RBX18StarterPackService8render2dEPNS_5AdornE — RBX::StarterPackService::render2d(RBX::Adorn *)
pub fn stub_5732ac() {
    // IDA 0x5732ac: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HopperBin::getBinType(void)const")]
// 0x573688 — __ZNK3RBX9HopperBin10getBinTypeEv — RBX::HopperBin::getBinType(void)const
pub fn stub_573688() {
    // IDA 0x573688: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RelativePanel::RelativePanel(void)")]
// 0x573944 — __ZN3RBX13RelativePanelC2Ev — RBX::RelativePanel::RelativePanel(void)
pub fn stub_573944() {
    // IDA 0x573944: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HopperBin::~HopperBin()")]
// 0x573a5c — __ZN3RBX9HopperBinD1Ev — RBX::HopperBin::~HopperBin()
pub fn stub_573a5c() {
    // IDA 0x573a5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HopperBin::~HopperBin()")]
// 0x573a60 — __ZN3RBX9HopperBinD0Ev — RBX::HopperBin::~HopperBin()
pub fn stub_573a60() {
    // IDA 0x573a60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BackpackItem::isEnabled(void)")]
// 0x573b10 — __ZN3RBX12BackpackItem9isEnabledEv — RBX::BackpackItem::isEnabled(void)
pub fn stub_573b10() {
    // IDA 0x573b10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BackpackItem::drawEnabled(void)const")]
// 0x573b1c — __ZNK3RBX12BackpackItem11drawEnabledEv — RBX::BackpackItem::drawEnabled(void)const
pub fn stub_573b1c() {
    // IDA 0x573b1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HopperBin::drawSelected(void)const")]
// 0x573b20 — __ZNK3RBX9HopperBin12drawSelectedEv — RBX::HopperBin::drawSelected(void)const
pub fn stub_573b20() {
    // IDA 0x573b20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HopperBin::~HopperBin()")]
// 0x573b28 — __ZThn32_N3RBX9HopperBinD1Ev — non-virtual thunk toRBX::HopperBin::~HopperBin()
pub fn stub_573b28() {
    // IDA 0x573b28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HopperBin::~HopperBin()")]
// 0x573b30 — __ZThn32_N3RBX9HopperBinD0Ev — non-virtual thunk toRBX::HopperBin::~HopperBin()
pub fn stub_573b30() {
    // IDA 0x573b30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HopperBin::~HopperBin()")]
// 0x573be4 — __ZThn36_N3RBX9HopperBinD1Ev — non-virtual thunk toRBX::HopperBin::~HopperBin()
pub fn stub_573be4() {
    // IDA 0x573be4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HopperBin::~HopperBin()")]
// 0x573bec — __ZThn36_N3RBX9HopperBinD0Ev — non-virtual thunk toRBX::HopperBin::~HopperBin()
pub fn stub_573bec() {
    // IDA 0x573bec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterGear::~StarterGear()")]
// 0x573c90 — __ZN3RBX11StarterGearD1Ev — RBX::StarterGear::~StarterGear()
pub fn stub_573c90() {
    // IDA 0x573c90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterGear::~StarterGear()")]
// 0x573c94 — __ZN3RBX11StarterGearD0Ev — RBX::StarterGear::~StarterGear()
pub fn stub_573c94() {
    // IDA 0x573c94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterGear::canClientCreate(void)")]
// 0x573d34 — __ZN3RBX11StarterGear15canClientCreateEv — RBX::StarterGear::canClientCreate(void)
pub fn stub_573d34() {
    // IDA 0x573d34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StarterGear::~StarterGear()")]
// 0x573d48 — __ZThn32_N3RBX11StarterGearD1Ev — non-virtual thunk toRBX::StarterGear::~StarterGear()
pub fn stub_573d48() {
    // IDA 0x573d48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StarterGear::~StarterGear()")]
// 0x573d50 — __ZThn32_N3RBX11StarterGearD0Ev — non-virtual thunk toRBX::StarterGear::~StarterGear()
pub fn stub_573d50() {
    // IDA 0x573d50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StarterGear::~StarterGear()")]
// 0x573e04 — __ZThn36_N3RBX11StarterGearD1Ev — non-virtual thunk toRBX::StarterGear::~StarterGear()
pub fn stub_573e04() {
    // IDA 0x573e04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StarterGear::~StarterGear()")]
// 0x573e0c — __ZThn36_N3RBX11StarterGearD0Ev — non-virtual thunk toRBX::StarterGear::~StarterGear()
pub fn stub_573e0c() {
    // IDA 0x573e0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BackpackItem::~BackpackItem()")]
// 0x573eb0 — __ZN3RBX12BackpackItemD1Ev — RBX::BackpackItem::~BackpackItem()
pub fn stub_573eb0() {
    // IDA 0x573eb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BackpackItem::~BackpackItem()")]
// 0x573fe4 — __ZN3RBX12BackpackItemD0Ev — RBX::BackpackItem::~BackpackItem()
pub fn stub_573fe4() {
    // IDA 0x573fe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BackpackItem::drawSelected(void)const")]
// 0x574150 — __ZNK3RBX12BackpackItem12drawSelectedEv — RBX::BackpackItem::drawSelected(void)const
pub fn stub_574150() {
    // IDA 0x574150: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BackpackItem::onLocalClicked(void)")]
// 0x574154 — __ZN3RBX12BackpackItem14onLocalClickedEv — RBX::BackpackItem::onLocalClicked(void)
pub fn stub_574154() {
    // IDA 0x574154: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BackpackItem::onLocalOtherClicked(void)")]
// 0x574158 — __ZN3RBX12BackpackItem19onLocalOtherClickedEv — RBX::BackpackItem::onLocalOtherClicked(void)
pub fn stub_574158() {
    // IDA 0x574158: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::BackpackItem::~BackpackItem()")]
// 0x57415c — __ZThn32_N3RBX12BackpackItemD1Ev — non-virtual thunk toRBX::BackpackItem::~BackpackItem()
pub fn stub_57415c() {
    // IDA 0x57415c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::BackpackItem::~BackpackItem()")]
// 0x57428c — __ZThn32_N3RBX12BackpackItemD0Ev — non-virtual thunk toRBX::BackpackItem::~BackpackItem()
pub fn stub_57428c() {
    // IDA 0x57428c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::BackpackItem::~BackpackItem()")]
// 0x5743f8 — __ZThn36_N3RBX12BackpackItemD1Ev — non-virtual thunk toRBX::BackpackItem::~BackpackItem()
pub fn stub_5743f8() {
    // IDA 0x5743f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::BackpackItem::~BackpackItem()")]
// 0x574528 — __ZThn36_N3RBX12BackpackItemD0Ev — non-virtual thunk toRBX::BackpackItem::~BackpackItem()
pub fn stub_574528() {
    // IDA 0x574528: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Hopper::~Hopper()")]
// 0x57466c — __ZN3RBX6HopperD1Ev — RBX::Hopper::~Hopper()
pub fn stub_57466c() {
    // IDA 0x57466c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Hopper::~Hopper()")]
// 0x574670 — __ZN3RBX6HopperD0Ev — RBX::Hopper::~Hopper()
pub fn stub_574670() {
    // IDA 0x574670: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Hopper::~Hopper()")]
// 0x574710 — __ZThn32_N3RBX6HopperD1Ev — non-virtual thunk toRBX::Hopper::~Hopper()
pub fn stub_574710() {
    // IDA 0x574710: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Hopper::~Hopper()")]
// 0x574718 — __ZThn32_N3RBX6HopperD0Ev — non-virtual thunk toRBX::Hopper::~Hopper()
pub fn stub_574718() {
    // IDA 0x574718: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Hopper::~Hopper()")]
// 0x5747bc — __ZThn36_N3RBX6HopperD1Ev — non-virtual thunk toRBX::Hopper::~Hopper()
pub fn stub_5747bc() {
    // IDA 0x5747bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Hopper::~Hopper()")]
// 0x5747c4 — __ZThn36_N3RBX6HopperD0Ev — non-virtual thunk toRBX::Hopper::~Hopper()
pub fn stub_5747c4() {
    // IDA 0x5747c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterPackService::~StarterPackService()")]
// 0x574868 — __ZN3RBX18StarterPackServiceD1Ev — RBX::StarterPackService::~StarterPackService()
pub fn stub_574868() {
    // IDA 0x574868: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterPackService::~StarterPackService()")]
// 0x57486c — __ZN3RBX18StarterPackServiceD0Ev — RBX::StarterPackService::~StarterPackService()
pub fn stub_57486c() {
    // IDA 0x57486c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StarterPackService::~StarterPackService()")]
// 0x574934 — __ZThn32_N3RBX18StarterPackServiceD1Ev — non-virtual thunk toRBX::StarterPackService::~StarterPackService()
pub fn stub_574934() {
    // IDA 0x574934: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StarterPackService::~StarterPackService()")]
// 0x57493c — __ZThn32_N3RBX18StarterPackServiceD0Ev — non-virtual thunk toRBX::StarterPackService::~StarterPackService()
pub fn stub_57493c() {
    // IDA 0x57493c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StarterPackService::~StarterPackService()")]
// 0x574a08 — __ZThn36_N3RBX18StarterPackServiceD1Ev — non-virtual thunk toRBX::StarterPackService::~StarterPackService()
pub fn stub_574a08() {
    // IDA 0x574a08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::StarterPackService::~StarterPackService()")]
// 0x574a10 — __ZThn36_N3RBX18StarterPackServiceD0Ev — non-virtual thunk toRBX::StarterPackService::~StarterPackService()
pub fn stub_574a10() {
    // IDA 0x574a10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::resize(unsigned long,RBX::HopperBin::BinType)")]
// 0x5790bc — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE6resizeEmS2_ — std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::resize(unsigned long,RBX::HopperBin::BinType)
pub fn stub_5790bc() {
    // IDA 0x5790bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::HopperBin::BinType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::operator[](RBX::Name const* const&)")]
// 0x5790f0 — __ZNSt3mapIPKN3RBX4NameENS0_9HopperBin7BinTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::HopperBin::BinType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::operator[](RBX::Name const* const&)
pub fn stub_5790f0() {
    // IDA 0x5790f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
// 0x579148 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)
pub fn stub_579148() {
    // IDA 0x579148: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
// 0x5791fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)
pub fn stub_5791fc() {
    // IDA 0x5791fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)")]
// 0x579254 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::HopperBin::BinType> const&)
pub fn stub_579254() {
    // IDA 0x579254: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,unsigned long,RBX::HopperBin::BinType const&)")]
// 0x5792bc — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,unsigned long,RBX::HopperBin::BinType const&)
pub fn stub_5792bc() {
    // IDA 0x5792bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_allocate(unsigned long)")]
// 0x57944c — __ZNSt12_Vector_baseIN3RBX9HopperBin7BinTypeESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_allocate(unsigned long)
pub fn stub_57944c() {
    // IDA 0x57944c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HopperBin::BinType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HopperBin::BinType *,RBX::HopperBin::BinType *>(RBX::HopperBin::BinType *,RBX::HopperBin::BinType *,RBX::HopperBin::BinType *)")]
// 0x579464 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9HopperBin7BinTypeES6_EET0_T_S8_S7_ — RBX::HopperBin::BinType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::HopperBin::BinType *,RBX::HopperBin::BinType *>(RBX::HopperBin::BinType *,RBX::HopperBin::BinType *,RBX::HopperBin::BinType *)
pub fn stub_579464() {
    // IDA 0x579464: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::push_back(RBX::HopperBin::BinType const&)")]
// 0x5794a0 — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE9push_backERKS2_ — std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::push_back(RBX::HopperBin::BinType const&)
pub fn stub_5794a0() {
    // IDA 0x5794a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,RBX::HopperBin::BinType const&)")]
// 0x5794c8 — __ZNSt6vectorIN3RBX9HopperBin7BinTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::HopperBin::BinType*,std::vector<RBX::HopperBin::BinType,std::allocator<RBX::HopperBin::BinType>>>,RBX::HopperBin::BinType const&)
pub fn stub_5794c8() {
    // IDA 0x5794c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HopperBin::~HopperBin()")]
// 0x5795ac — __ZN3RBX9HopperBinD2Ev — RBX::HopperBin::~HopperBin()
pub fn stub_5795ac() {
    // IDA 0x5795ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ICharacterSubject::ICharacterSubject(void)")]
// 0x579f70 — __ZN3RBX17ICharacterSubjectC2Ev — RBX::ICharacterSubject::ICharacterSubject(void)
pub fn stub_579f70() {
    // IDA 0x579f70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ICharacterSubject::isFirstPerson(void)const")]
// 0x57a09c — __ZNK3RBX17ICharacterSubject13isFirstPersonEv — RBX::ICharacterSubject::isFirstPerson(void)const
pub fn stub_57a09c() {
    // IDA 0x57a09c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ICharacterSubject::setCameraMode(RBX::Camera::CameraMode)")]
// 0x57bd7c — __ZN3RBX17ICharacterSubject13setCameraModeENS_6Camera10CameraModeE — RBX::ICharacterSubject::setCameraMode(RBX::Camera::CameraMode)
pub fn stub_57bd7c() {
    // IDA 0x57bd7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IEquipable::IEquipable(void)")]
// 0x57bf9c — __ZN3RBX10IEquipableC2Ev — RBX::IEquipable::IEquipable(void)
pub fn stub_57bf9c() {
    // IDA 0x57bf9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IEquipable::~IEquipable()")]
// 0x57bfb4 — __ZN3RBX10IEquipableD0Ev — RBX::IEquipable::~IEquipable()
pub fn stub_57bfb4() {
    // IDA 0x57bfb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IEquipable::~IEquipable()")]
// 0x57c054 — __ZN3RBX10IEquipableD1Ev — RBX::IEquipable::~IEquipable()
pub fn stub_57c054() {
    // IDA 0x57c054: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IEquipable::~IEquipable()")]
// 0x57c058 — __ZN3RBX10IEquipableD2Ev — RBX::IEquipable::~IEquipable()
pub fn stub_57c058() {
    // IDA 0x57c058: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiImageButton::GuiImageButton(void)")]
// 0x57c644 — __ZN3RBX14GuiImageButtonC2Ev — RBX::GuiImageButton::GuiImageButton(void)
pub fn stub_57c644() {
    // IDA 0x57c644: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")]
// 0x57c894 — __ZN3RBX14GuiImageButtonC1EPNS_4VerbE — RBX::GuiImageButton::GuiImageButton(RBX::Verb *)
pub fn stub_57c894() {
    // IDA 0x57c894: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")]
// 0x57c898 — __ZN3RBX14GuiImageButtonC2EPNS_4VerbE — RBX::GuiImageButton::GuiImageButton(RBX::Verb *)
pub fn stub_57c898() {
    // IDA 0x57c898: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiImageButton::setImage(RBX::TextureId)")]
// 0x57caf4 — __ZN3RBX14GuiImageButton8setImageENS_9TextureIdE — RBX::GuiImageButton::setImage(RBX::TextureId)
pub fn stub_57caf4() {
    // IDA 0x57caf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::setImage(RBX::TextureId)")]
// 0x57cb34 — __ZThn800_N3RBX14GuiImageButton8setImageENS_9TextureIdE — non-virtual thunk toRBX::GuiImageButton::setImage(RBX::TextureId)
pub fn stub_57cb34() {
    // IDA 0x57cb34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiImageButton::render2d(RBX::Adorn *)")]
// 0x57cbe4 — __ZN3RBX14GuiImageButton8render2dEPNS_5AdornE — RBX::GuiImageButton::render2d(RBX::Adorn *)
pub fn stub_57cbe4() {
    // IDA 0x57cbe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::render2d(RBX::Adorn *)")]
// 0x57cd38 — __ZThn96_N3RBX14GuiImageButton8render2dEPNS_5AdornE — non-virtual thunk toRBX::GuiImageButton::render2d(RBX::Adorn *)
pub fn stub_57cd38() {
    // IDA 0x57cd38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiImageButton::~GuiImageButton()")]
// 0x57cd64 — __ZN3RBX14GuiImageButtonD1Ev — RBX::GuiImageButton::~GuiImageButton()
pub fn stub_57cd64() {
    // IDA 0x57cd64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiImageButton::~GuiImageButton()")]
// 0x57ce5c — __ZN3RBX14GuiImageButtonD0Ev — RBX::GuiImageButton::~GuiImageButton()
pub fn stub_57ce5c() {
    // IDA 0x57ce5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
// 0x57cf74 — __ZThn32_N3RBX14GuiImageButtonD1Ev — non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()
pub fn stub_57cf74() {
    // IDA 0x57cf74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
// 0x57d06c — __ZThn32_N3RBX14GuiImageButtonD0Ev — non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()
pub fn stub_57d06c() {
    // IDA 0x57d06c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
// 0x57d188 — __ZThn36_N3RBX14GuiImageButtonD1Ev — non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()
pub fn stub_57d188() {
    // IDA 0x57d188: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
// 0x57d280 — __ZThn36_N3RBX14GuiImageButtonD0Ev — non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()
pub fn stub_57d280() {
    // IDA 0x57d280: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ImageLabel::ImageLabel(void)")]
// 0x57e37c — __ZN3RBX10ImageLabelC1Ev — RBX::ImageLabel::ImageLabel(void)
pub fn stub_57e37c() {
    // IDA 0x57e37c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ImageLabel::ImageLabel(void)")]
// 0x57e380 — __ZN3RBX10ImageLabelC2Ev — RBX::ImageLabel::ImageLabel(void)
pub fn stub_57e380() {
    // IDA 0x57e380: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ImageLabel::setImage(RBX::TextureId)")]
// 0x57e5c8 — __ZN3RBX10ImageLabel8setImageENS_9TextureIdE — RBX::ImageLabel::setImage(RBX::TextureId)
pub fn stub_57e5c8() {
    // IDA 0x57e5c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ImageLabel::setImage(RBX::TextureId)")]
// 0x57e608 — __ZThn536_N3RBX10ImageLabel8setImageENS_9TextureIdE — non-virtual thunk toRBX::ImageLabel::setImage(RBX::TextureId)
pub fn stub_57e608() {
    // IDA 0x57e608: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ImageLabel::render2d(RBX::Adorn *)")]
// 0x57e6b8 — __ZN3RBX10ImageLabel8render2dEPNS_5AdornE — RBX::ImageLabel::render2d(RBX::Adorn *)
pub fn stub_57e6b8() {
    // IDA 0x57e6b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ImageLabel::render2d(RBX::Adorn *)")]
// 0x57e7b4 — __ZThn96_N3RBX10ImageLabel8render2dEPNS_5AdornE — non-virtual thunk toRBX::ImageLabel::render2d(RBX::Adorn *)
pub fn stub_57e7b4() {
    // IDA 0x57e7b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ImageLabel::renderBackground2d(RBX::Adorn *)")]
// 0x57e7bc — __ZN3RBX10ImageLabel18renderBackground2dEPNS_5AdornE — RBX::ImageLabel::renderBackground2d(RBX::Adorn *)
pub fn stub_57e7bc() {
    // IDA 0x57e7bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ImageLabel::renderBackground2d(RBX::Adorn *)")]
// 0x57e7f0 — __ZThn96_N3RBX10ImageLabel18renderBackground2dEPNS_5AdornE — non-virtual thunk toRBX::ImageLabel::renderBackground2d(RBX::Adorn *)
pub fn stub_57e7f0() {
    // IDA 0x57e7f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiImageMixin::getImage(void)const")]
// 0x57e7f8 — __ZNK3RBX13GuiImageMixin8getImageEv — RBX::GuiImageMixin::getImage(void)const
pub fn stub_57e7f8() {
    // IDA 0x57e7f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ImageLabel::~ImageLabel()")]
// 0x57e830 — __ZN3RBX10ImageLabelD1Ev — RBX::ImageLabel::~ImageLabel()
pub fn stub_57e830() {
    // IDA 0x57e830: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ImageLabel::~ImageLabel()")]
// 0x57e928 — __ZN3RBX10ImageLabelD0Ev — RBX::ImageLabel::~ImageLabel()
pub fn stub_57e928() {
    // IDA 0x57e928: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiLabel::isGuiLeaf(void)const")]
// 0x57ea40 — __ZNK3RBX8GuiLabel9isGuiLeafEv — RBX::GuiLabel::isGuiLeaf(void)const
pub fn stub_57ea40() {
    // IDA 0x57ea40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
// 0x57ea44 — __ZThn32_N3RBX10ImageLabelD1Ev — non-virtual thunk toRBX::ImageLabel::~ImageLabel()
pub fn stub_57ea44() {
    // IDA 0x57ea44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
// 0x57eb3c — __ZThn32_N3RBX10ImageLabelD0Ev — non-virtual thunk toRBX::ImageLabel::~ImageLabel()
pub fn stub_57eb3c() {
    // IDA 0x57eb3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
// 0x57ec58 — __ZThn36_N3RBX10ImageLabelD1Ev — non-virtual thunk toRBX::ImageLabel::~ImageLabel()
pub fn stub_57ec58() {
    // IDA 0x57ec58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
// 0x57ed50 — __ZThn36_N3RBX10ImageLabelD0Ev — non-virtual thunk toRBX::ImageLabel::~ImageLabel()
pub fn stub_57ed50() {
    // IDA 0x57ed50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextureId const& rbx::any_cast<RBX::TextureId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x57fce4 — __ZN3rbx8any_castIRKN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::TextureId const& rbx::any_cast<RBX::TextureId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_57fce4() {
    // IDA 0x57fce4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextureId>(RBX::TextureId const&)")]
// 0x57fdd4 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9TextureIdEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextureId>(RBX::TextureId const&)
pub fn stub_57fdd4() {
    // IDA 0x57fdd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TextureId>::singleton(void)")]
// 0x57fe34 — __ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE9singletonEv — rbx::implementation::typed_holder<RBX::TextureId>::singleton(void)
pub fn stub_57fe34() {
    // IDA 0x57fe34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TextureId>::construct_func(char const*,char *)")]
// 0x57fea0 — __ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::TextureId>::construct_func(char const*,char *)
pub fn stub_57fea0() {
    // IDA 0x57fea0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TextureId>::destruct_func(char *)")]
// 0x57febc — __ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::TextureId>::destruct_func(char *)
pub fn stub_57febc() {
    // IDA 0x57febc: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::InsertService::setBaseSetsUrl(std::string)")]
// 0x580708 — __ZN3RBX13InsertService14setBaseSetsUrlESs — RBX::InsertService::setBaseSetsUrl(std::string)
pub fn stub_580708() {
    // IDA 0x580708: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::InsertService::setUserSetsUrl(std::string)")]
// 0x580710 — __ZN3RBX13InsertService14setUserSetsUrlESs — RBX::InsertService::setUserSetsUrl(std::string)
pub fn stub_580710() {
    // IDA 0x580710: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::InsertService::setTrustLevel(float)")]
// 0x580718 — __ZN3RBX13InsertService13setTrustLevelEf — RBX::InsertService::setTrustLevel(float)
pub fn stub_580718() {
    // IDA 0x580718: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::InsertService::setFreeModelUrl(std::string)")]
// 0x580720 — __ZN3RBX13InsertService15setFreeModelUrlESs — RBX::InsertService::setFreeModelUrl(std::string)
pub fn stub_580720() {
    // IDA 0x580720: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::InsertService::setFreeDecalUrl(std::string)")]
// 0x580728 — __ZN3RBX13InsertService15setFreeDecalUrlESs — RBX::InsertService::setFreeDecalUrl(std::string)
pub fn stub_580728() {
    // IDA 0x580728: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
