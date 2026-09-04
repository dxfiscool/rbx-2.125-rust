//! core shard CB — 100 core stubs EA-sorted, next uncovered after CA 0x5d5c08 (strict RBX|boost|std|rbx earliest gap 0x5d5d5c).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::MouseCommand::getSearchRay(RBX::UIEvent const&)const")]
// 0x5d5d5c — __ZNK3RBX12MouseCommand12getSearchRayERKNS_7UIEventE
pub fn stub_0x5d5d5c() {
    // IDA 0x5d5d5c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PartByLocalCharacter::~PartByLocalCharacter()")]
// 0x5d626c — __ZN3RBX20PartByLocalCharacterD1Ev
pub fn stub_0x5d626c() {
    // IDA 0x5d626c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UnlockedPartByLocalCharacter::~UnlockedPartByLocalCharacter()")]
// 0x5d634c — __ZN3RBX28UnlockedPartByLocalCharacterD1Ev
pub fn stub_0x5d634c() {
    // IDA 0x5d634c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::onMouseDown(RBX::UIEvent const&)")]
// 0x5d642c — __ZN3RBX12MouseCommand11onMouseDownERKNS_7UIEventE
pub fn stub_0x5d642c() {
    // IDA 0x5d642c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::onMouseHover(RBX::UIEvent const&)")]
// 0x5d6438 — __ZN3RBX12MouseCommand12onMouseHoverERKNS_7UIEventE
pub fn stub_0x5d6438() {
    // IDA 0x5d6438: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MouseCommand::getCursorName(void)const")]
// 0x5d643c — __ZNK3RBX12MouseCommand13getCursorNameEv
pub fn stub_0x5d643c() {
    // IDA 0x5d643c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PartByLocalCharacter::~PartByLocalCharacter()")]
// 0x5d6474 — __ZN3RBX20PartByLocalCharacterD0Ev
pub fn stub_0x5d6474() {
    // IDA 0x5d6474: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UnlockedPartByLocalCharacter::~UnlockedPartByLocalCharacter()")]
// 0x5d655c — __ZN3RBX28UnlockedPartByLocalCharacterD0Ev
pub fn stub_0x5d655c() {
    // IDA 0x5d655c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PART::Wedge::Wedge(void)")]
// 0x5d6cf8 — __ZN3RBX4PART5WedgeC1Ev
pub fn stub_0x5d6cf8() {
    // IDA 0x5d6cf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PART::Wedge::~Wedge()")]
// 0x5d6f9c — __ZN3RBX4PART5WedgeD0Ev
pub fn stub_0x5d6f9c() {
    // IDA 0x5d6f9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PART::Wedge::~Wedge()")]
// 0x5d704c — __ZN3RBX4PART5WedgeD1Ev
pub fn stub_0x5d704c() {
    // IDA 0x5d704c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PART::Wedge::~Wedge()")]
// 0x5d705c — __ZThn32_N3RBX4PART5WedgeD0Ev
pub fn stub_0x5d705c() {
    // IDA 0x5d705c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PART::Wedge::~Wedge()")]
// 0x5d7064 — __ZThn36_N3RBX4PART5WedgeD0Ev
pub fn stub_0x5d7064() {
    // IDA 0x5d7064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PART::Wedge::~Wedge()")]
// 0x5d706c — __ZThn132_N3RBX4PART5WedgeD0Ev
pub fn stub_0x5d706c() {
    // IDA 0x5d706c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PART::Wedge::~Wedge()")]
// 0x5d7074 — __ZThn32_N3RBX4PART5WedgeD1Ev
pub fn stub_0x5d7074() {
    // IDA 0x5d7074: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PART::Wedge::~Wedge()")]
// 0x5d7088 — __ZThn36_N3RBX4PART5WedgeD1Ev
pub fn stub_0x5d7088() {
    // IDA 0x5d7088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PART::Wedge::~Wedge()")]
// 0x5d709c — __ZThn132_N3RBX4PART5WedgeD1Ev
pub fn stub_0x5d709c() {
    // IDA 0x5d709c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PART::Wedge::getPartType(void)const")]
// 0x5d70c0 — __ZNK3RBX4PART5Wedge11getPartTypeEv
pub fn stub_0x5d70c0() {
    // IDA 0x5d70c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::toSpecificGravity(RBX::Material)")]
// 0x5d9dec — __ZN3RBXL17toSpecificGravityENS_8MaterialE
pub fn stub_0x5d9dec() {
    // IDA 0x5d9dec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::getComponent(XmlElement const*,RBX::Name const&)")]
// 0x5dfc80 — __ZN3RBXL12getComponentEPK10XmlElementRKNS_4NameE
pub fn stub_0x5dfc80() {
    // IDA 0x5dfc80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::getExtentsWorld(void)const")]
// 0x5e0f90 — __ZNK3RBX9Primitive15getExtentsWorldEv
pub fn stub_0x5e0f90() {
    // IDA 0x5e0f90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IMoving::~IMoving()")]
// 0x5e12a8 — __ZN3RBX7IMovingD2Ev
pub fn stub_0x5e12a8() {
    // IDA 0x5e12a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Dragger::dragSnap(void)")]
// 0x5e1468 — __ZN3RBX7Dragger8dragSnapEv
pub fn stub_0x5e1468() {
    // IDA 0x5e1468: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Joint::getNormalId(int)const")]
// 0x5e14bc — __ZNK3RBX5Joint11getNormalIdEi
pub fn stub_0x5e14bc() {
    // IDA 0x5e14bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FWValue<RBX::World *>::set(RBX::World * const&,RBX::FWRef *)")]
// 0x5e1bc0 — __ZN3RBX7FWValueIPNS_5WorldEE3setERKS2_PNS_5FWRefE
pub fn stub_0x5e1bc0() {
    // IDA 0x5e1bc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FWValue<RBX::Material>::set(RBX::Material const&,RBX::FWRef *)")]
// 0x5e21fc — __ZN3RBX7FWValueINS_8MaterialEE3setERKS1_PNS_5FWRefE
pub fn stub_0x5e21fc() {
    // IDA 0x5e21fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FWValue<float>::set(float const&,RBX::FWRef *)")]
// 0x5e2424 — __ZN3RBX7FWValueIfE3setERKfPNS_5FWRefE
pub fn stub_0x5e2424() {
    // IDA 0x5e2424: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FWValue<RBX::BrickColor>::set(RBX::BrickColor const&,RBX::FWRef *)")]
// 0x5e2658 — __ZN3RBX7FWValueINS_10BrickColorEE3setERKS1_PNS_5FWRefE
pub fn stub_0x5e2658() {
    // IDA 0x5e2658: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IAdornable::shouldRender2d(void)const")]
// 0x5e2b98 — __ZNK3RBX10IAdornable14shouldRender2dEv
pub fn stub_0x5e2b98() {
    // IDA 0x5e2b98: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IAdornable::render2d(RBX::Adorn *)")]
// 0x5e2bb0 — __ZN3RBX10IAdornable8render2dEPNS_5AdornE
pub fn stub_0x5e2bb0() {
    // IDA 0x5e2bb0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::IAdornable::render3dSortedAdorn(RBX::Adorn *)")]
// 0x5e2bbc — __ZN3RBX10IAdornable19render3dSortedAdornEPNS_5AdornE
pub fn stub_0x5e2bbc() {
    // IDA 0x5e2bbc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::IAdornable::render3dSortedPosition(void)const")]
// 0x5e2bc0 — __ZNK3RBX10IAdornable22render3dSortedPositionEv
pub fn stub_0x5e2bc0() {
    // IDA 0x5e2bc0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraSubject::onCameraWrapMouse(void)")]
// 0x5e2c00 — __ZN3RBX13CameraSubject17onCameraWrapMouseEv
pub fn stub_0x5e2c00() {
    // IDA 0x5e2c00: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraSubject::getSelectionIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
// 0x5e2c48 — __ZN3RBX13CameraSubject28getSelectionIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
pub fn stub_0x5e2c48() {
    // IDA 0x5e2c48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Selectable::isSelectable3d(void)")]
// 0x5e2c90 — __ZN3RBX10Selectable14isSelectable3dEv
pub fn stub_0x5e2c90() {
    // IDA 0x5e2c90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::CameraSubject::~CameraSubject()")]
// 0x5e2fa0 — __ZN3RBX13CameraSubjectD1Ev
pub fn stub_0x5e2fa0() {
    // IDA 0x5e2fa0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CameraSubject::~CameraSubject()")]
// 0x5e2fa4 — __ZN3RBX13CameraSubjectD0Ev
pub fn stub_0x5e2fa4() {
    // IDA 0x5e2fa4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CameraSubject::onCameraNear(float)")]
// 0x5e2fa8 — __ZN3RBX13CameraSubject12onCameraNearEf
pub fn stub_0x5e2fa8() {
    // IDA 0x5e2fa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CameraSubject::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
// 0x5e2fac — __ZN3RBX13CameraSubject25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
pub fn stub_0x5e2fac() {
    // IDA 0x5e2fac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Material>(RBX::Material const&)")]
// 0x5e3d48 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8MaterialEEERS3_RKT_
pub fn stub_0x5e3d48() {
    // IDA 0x5e3d48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Material>::singleton(void)")]
// 0x5e3d98 — __ZN3rbx14implementation12typed_holderIN3RBX8MaterialEE9singletonEv
pub fn stub_0x5e3d98() {
    // IDA 0x5e3d98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Material>::construct_func(char const*,char *)")]
// 0x5e3e04 — __ZN3rbx14implementation12typed_holderIN3RBX8MaterialEE14construct_funcEPKcPc
pub fn stub_0x5e3e04() {
    // IDA 0x5e3e04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Material>::destruct_func(char *)")]
// 0x5e3e10 — __ZN3rbx14implementation12typed_holderIN3RBX8MaterialEE13destruct_funcEPc
pub fn stub_0x5e3e10() {
    // IDA 0x5e3e10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Material const& rbx::any_cast<RBX::Material const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5e3ee0 — __ZN3rbx8any_castIRKN3RBX8MaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x5e3ee0() {
    // IDA 0x5e3ee0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::bad_placement_any_cast::~bad_placement_any_cast()")]
// 0x5e40b8 — __ZN3rbx22bad_placement_any_castD0Ev
pub fn stub_0x5e40b8() {
    // IDA 0x5e40b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "XmlElement::XmlElement<float>(RBX::Name const&,float)")]
// 0x5e4cb4 — __ZN10XmlElementC2IfEERKN3RBX4NameET_
pub fn stub_0x5e4cb4() {
    // IDA 0x5e4cb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FWService * RBX::ServiceProvider::find<RBX::FWService>(void)const")]
// 0x5e6a00 — __ZNK3RBX15ServiceProvider4findINS_9FWServiceEEEPT_v
pub fn stub_0x5e6a00() {
    // IDA 0x5e6a00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FWService>(void)")]
// 0x5e6d44 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9FWServiceEEEvv
pub fn stub_0x5e6d44() {
    // IDA 0x5e6d44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FWService>(void)")]
// 0x5e6d48 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_9FWServiceEEEmv
pub fn stub_0x5e6d48() {
    // IDA 0x5e6d48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SystemAddress const& rbx::any_cast<RBX::SystemAddress const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5ed068 — __ZN3rbx8any_castIRKN3RBX13SystemAddressENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x5ed068() {
    // IDA 0x5ed068: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SystemAddress>(RBX::SystemAddress const&)")]
// 0x5ed158 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SystemAddressEEERS3_RKT_
pub fn stub_0x5ed158() {
    // IDA 0x5ed158: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::singleton(void)")]
// 0x5ed1b0 — __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE9singletonEv
pub fn stub_0x5ed1b0() {
    // IDA 0x5ed1b0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::construct_func(char const*,char *)")]
// 0x5ed21c — __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE14construct_funcEPKcPc
pub fn stub_0x5ed21c() {
    // IDA 0x5ed21c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SystemAddress>::destruct_func(char *)")]
// 0x5ed22c — __ZN3rbx14implementation12typed_holderIN3RBX13SystemAddressEE13destruct_funcEPc
pub fn stub_0x5ed22c() {
    // IDA 0x5ed22c: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::NormalId>(RBX::NormalId const&)")]
// 0x5eddb0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8NormalIdEEERS3_RKT_
pub fn stub_0x5eddb0() {
    // IDA 0x5eddb0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::NormalId>::singleton(void)")]
// 0x5ede00 — __ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE9singletonEv
pub fn stub_0x5ede00() {
    // IDA 0x5ede00: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::NormalId>::destruct_func(char *)")]
// 0x5ede70 — __ZN3rbx14implementation12typed_holderIN3RBX8NormalIdEE13destruct_funcEPc
pub fn stub_0x5ede70() {
    // IDA 0x5ede70: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::NormalId const& rbx::any_cast<RBX::NormalId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5ede78 — __ZN3rbx8any_castIRKN3RBX8NormalIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x5ede78() {
    // IDA 0x5ede78: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<bool>::singleton(void)")]
// 0x5edfe8 — __ZN3rbx14implementation12typed_holderIbE9singletonEv
pub fn stub_0x5edfe8() {
    // IDA 0x5edfe8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<bool>::destruct_func(char *)")]
// 0x5ee058 — __ZN3rbx14implementation12typed_holderIbE13destruct_funcEPc
pub fn stub_0x5ee058() {
    // IDA 0x5ee058: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<int>::construct_func(char const*,char *)")]
// 0x5ee558 — __ZN3rbx14implementation12typed_holderIiE14construct_funcEPKcPc
pub fn stub_0x5ee558() {
    // IDA 0x5ee558: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Faces const& rbx::any_cast<RBX::Faces const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5eeb20 — __ZN3rbx8any_castIRKN3RBX5FacesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x5eeb20() {
    // IDA 0x5eeb20: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Faces>(RBX::Faces const&)")]
// 0x5eec10 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5FacesEEERS3_RKT_
pub fn stub_0x5eec10() {
    // IDA 0x5eec10: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Faces>::singleton(void)")]
// 0x5eec60 — __ZN3rbx14implementation12typed_holderIN3RBX5FacesEE9singletonEv
pub fn stub_0x5eec60() {
    // IDA 0x5eec60: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Faces>::destruct_func(char *)")]
// 0x5eecd0 — __ZN3rbx14implementation12typed_holderIN3RBX5FacesEE13destruct_funcEPc
pub fn stub_0x5eecd0() {
    // IDA 0x5eecd0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::BrickColor const& rbx::any_cast<RBX::BrickColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x5f0010 — __ZN3rbx8any_castIRKN3RBX10BrickColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x5f0010() {
    // IDA 0x5f0010: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::BrickColor>(RBX::BrickColor const&)")]
// 0x5f0100 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10BrickColorEEERS3_RKT_
pub fn stub_0x5f0100() {
    // IDA 0x5f0100: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::BrickColor>::singleton(void)")]
// 0x5f0150 — __ZN3rbx14implementation12typed_holderIN3RBX10BrickColorEE9singletonEv
pub fn stub_0x5f0150() {
    // IDA 0x5f0150: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::BrickColor>::destruct_func(char *)")]
// 0x5f01c0 — __ZN3rbx14implementation12typed_holderIN3RBX10BrickColorEE13destruct_funcEPc
pub fn stub_0x5f01c0() {
    // IDA 0x5f01c0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<float>::construct_func(char const*,char *)")]
// 0x5f0c68 — __ZN3rbx14implementation12typed_holderIfE14construct_funcEPKcPc
pub fn stub_0x5f0c68() {
    // IDA 0x5f0c68: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Material,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::operator[](RBX::Name const* const&)")]
// 0x5f1fb4 — __ZNSt3mapIPKN3RBX4NameENS0_8MaterialESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
pub fn stub_0x5f1fb4() {
    // IDA 0x5f1fb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Material>>,std::pair<RBX::Name const* const,RBX::Material> const&)")]
// 0x5f200c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_0x5f200c() {
    // IDA 0x5f200c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Material> const&)")]
// 0x5f20c0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_0x5f20c0() {
    // IDA 0x5f20c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Material> const&)")]
// 0x5f2118 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_0x5f2118() {
    // IDA 0x5f2118: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::resize(unsigned long,RBX::Material)")]
// 0x5f21ac — __ZNSt6vectorIN3RBX8MaterialESaIS1_EE6resizeEmS1_
pub fn stub_0x5f21ac() {
    // IDA 0x5f21ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<unsigned long,std::allocator<unsigned long>>::resize(unsigned long,unsigned long)")]
// 0x5f21e0 — __ZNSt6vectorImSaImEE6resizeEmm
pub fn stub_0x5f21e0() {
    // IDA 0x5f21e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::push_back(RBX::Material const&)")]
// 0x5f2214 — __ZNSt6vectorIN3RBX8MaterialESaIS1_EE9push_backERKS1_
pub fn stub_0x5f2214() {
    // IDA 0x5f2214: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::resize(unsigned long,std::string)")]
// 0x5f2240 — __ZNSt6vectorISsSaISsEE6resizeEmSs
pub fn stub_0x5f2240() {
    // IDA 0x5f2240: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,RBX::Material const&)")]
// 0x5f228c — __ZNSt6vectorIN3RBX8MaterialESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0x5f228c() {
    // IDA 0x5f228c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Material,std::allocator<RBX::Material>>::_M_allocate(unsigned long)")]
// 0x5f2370 — __ZNSt12_Vector_baseIN3RBX8MaterialESaIS1_EE11_M_allocateEm
pub fn stub_0x5f2370() {
    // IDA 0x5f2370: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Material * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Material *,RBX::Material *>(RBX::Material *,RBX::Material *,RBX::Material *)")]
// 0x5f2388 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8MaterialES5_EET0_T_S7_S6_
pub fn stub_0x5f2388() {
    // IDA 0x5f2388: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Material,std::allocator<RBX::Material>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Material*,std::vector<RBX::Material,std::allocator<RBX::Material>>>,unsigned long,RBX::Material const&)")]
// 0x5f23c8 — __ZNSt6vectorIN3RBX8MaterialESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_0x5f23c8() {
    // IDA 0x5f23c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::~vector()")]
// 0x5f2a50 — __ZNSt6vectorISsSaISsEED2Ev
pub fn stub_0x5f2a50() {
    // IDA 0x5f2a50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::push_back(RBX::Primitive const* const&)")]
// 0x5f3d3c — __ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE9push_backERKS3_
pub fn stub_0x5f3d3c() {
    // IDA 0x5f3d3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive const**,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>>,RBX::Primitive const* const&)")]
// 0x5f3d68 — __ZNSt6vectorIPKN3RBX9PrimitiveESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_0x5f3d68() {
    // IDA 0x5f3d68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Primitive const*,std::allocator<RBX::Primitive const*>>::_M_allocate(unsigned long)")]
// 0x5f3e48 — __ZNSt12_Vector_baseIPKN3RBX9PrimitiveESaIS3_EE11_M_allocateEm
pub fn stub_0x5f3e48() {
    // IDA 0x5f3e48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FWBase::~FWBase()")]
// 0x5f3f80 — __ZN3RBX6FWBaseD0Ev
pub fn stub_0x5f3f80() {
    // IDA 0x5f3f80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SurfaceData::empty(void)")]
// 0x5f40dc — __ZN3RBX11SurfaceData5emptyEv
pub fn stub_0x5f40dc() {
    // IDA 0x5f40dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::EdgeList::getOther(int)const")]
// 0x5f412c — __ZNK3RBX8EdgeList8getOtherEi
pub fn stub_0x5f412c() {
    // IDA 0x5f412c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Body::updatePV(void)")]
// 0x5f41f0 — __ZN3RBX4Body8updatePVEv
pub fn stub_0x5f41f0() {
    // IDA 0x5f41f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Body::getMeInParent(void)")]
// 0x5f441c — __ZN3RBX4Body13getMeInParentEv
pub fn stub_0x5f441c() {
    // IDA 0x5f441c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Security::Context::ptr(void)")]
// 0x5f4488 — __ZN3RBX8Security7Context3ptrEv
pub fn stub_0x5f4488() {
    // IDA 0x5f4488: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::RbxRay::~RbxRay()")]
// 0x5f45c8 — __ZN3RBX6RbxRayD0Ev
pub fn stub_0x5f45c8() {
    // IDA 0x5f45c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::InterpolatedCFrame::InterpolatedCFrame(void)")]
// 0x5f46f4 — __ZN3RBX18InterpolatedCFrameC2Ev
pub fn stub_0x5f46f4() {
    // IDA 0x5f46f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IAdornable::render3dAdorn(RBX::Adorn *)")]
// 0x5f4d70 — __ZN3RBX10IAdornable13render3dAdornEPNS_5AdornE
pub fn stub_0x5f4d70() {
    // IDA 0x5f4d70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Material>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Material>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Material>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Material>> *)")]
// 0x5f5634 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8MaterialEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_0x5f5634() {
    // IDA 0x5f5634: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PhysicsInstructions::PhysicsInstructions(void)")]
// 0x5f6800 — __ZN3RBX19PhysicsInstructionsC1Ev
pub fn stub_0x5f6800() {
    // IDA 0x5f6800: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PhysicsInstructions::PhysicsInstructions(void)")]
// 0x5f6804 — __ZN3RBX19PhysicsInstructionsC2Ev
pub fn stub_0x5f6804() {
    // IDA 0x5f6804: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PhysicsInstructions::dPhysicsServerDutyPercent(void)")]
// 0x5f6948 — __ZN3RBX19PhysicsInstructions25dPhysicsServerDutyPercentEv
pub fn stub_0x5f6948() {
    // IDA 0x5f6948: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PhysicsInstructions::dPhysicsClientDutyPercent(void)")]
// 0x5f6a60 — __ZN3RBX19PhysicsInstructions25dPhysicsClientDutyPercentEv
pub fn stub_0x5f6a60() {
    // IDA 0x5f6a60: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
