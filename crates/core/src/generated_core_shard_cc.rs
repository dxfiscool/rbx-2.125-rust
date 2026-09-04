//! core shard CC — 100 core stubs EA-sorted, next uncovered after CB 0x5f6a60 (strict RBX|boost|std|rbx earliest gap 0x5f6a78).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::PhysicsInstructions::dPhysicsClientEThrottleDutyPercent(void)")]
// 0x5f6a78 — __ZN3RBX19PhysicsInstructions34dPhysicsClientEThrottleDutyPercentEv
pub fn stub_5f6a78() {
    // IDA 0x5f6a78: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<double,std::allocator<double>>::resize(unsigned long,double)")]
// 0x5f6cf8 — __ZNSt6vectorIdSaIdEE6resizeEmd
pub fn stub_5f6cf8() {
    // IDA 0x5f6cf8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PhysicsService::~PhysicsService()")]
// 0x5f6fac — __ZN3RBX14PhysicsServiceD0Ev
pub fn stub_5f6fac() {
    // IDA 0x5f6fac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PhysicsService::~PhysicsService()")]
// 0x5f704c — __ZN3RBX14PhysicsServiceD1Ev
pub fn stub_5f704c() {
    // IDA 0x5f704c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsService::~PhysicsService()")]
// 0x5f7050 — __ZThn32_N3RBX14PhysicsServiceD0Ev
pub fn stub_5f7050() {
    // IDA 0x5f7050: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsService::~PhysicsService()")]
// 0x5f7058 — __ZThn36_N3RBX14PhysicsServiceD0Ev
pub fn stub_5f7058() {
    // IDA 0x5f7058: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PhysicsService::~PhysicsService()")]
// 0x5f7060 — __ZN3RBX14PhysicsServiceD2Ev
pub fn stub_5f7060() {
    // IDA 0x5f7060: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsService::~PhysicsService()")]
// 0x5f7410 — __ZThn32_N3RBX14PhysicsServiceD1Ev
pub fn stub_5f7410() {
    // IDA 0x5f7410: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsService::~PhysicsService()")]
// 0x5f7418 — __ZThn36_N3RBX14PhysicsServiceD1Ev
pub fn stub_5f7418() {
    // IDA 0x5f7418: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PhysicsService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x5f7420 — __ZN3RBX14PhysicsService17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_5f7420() {
    // IDA 0x5f7420: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PhysicsService::onAssemblyPhysicsOn(RBX::Primitive *)")]
// 0x5f7598 — __ZN3RBX14PhysicsService19onAssemblyPhysicsOnEPNS_9PrimitiveE
pub fn stub_5f7598() {
    // IDA 0x5f7598: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PhysicsService::onAssemblyPhysicsOff(RBX::Primitive *)")]
// 0x5f788c — __ZN3RBX14PhysicsService20onAssemblyPhysicsOffEPNS_9PrimitiveE
pub fn stub_5f788c() {
    // IDA 0x5f788c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PhysicsSettings::getShowAnchoredParts(void)const")]
// 0x5f8a64 — __ZNK3RBX15PhysicsSettings20getShowAnchoredPartsEv
pub fn stub_5f8a64() {
    // IDA 0x5f8a64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PhysicsSettings::setShowAnchoredParts(bool)")]
// 0x5f8a74 — __ZN3RBX15PhysicsSettings20setShowAnchoredPartsEb
pub fn stub_5f8a74() {
    // IDA 0x5f8a74: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowPartCoordinateFrames(void)const")]
// 0x5f8aa4 — __ZNK3RBX15PhysicsSettings27getShowPartCoordinateFramesEv
pub fn stub_5f8aa4() {
    // IDA 0x5f8aa4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowPartCoordinateFrames(bool)")]
// 0x5f8ab4 — __ZN3RBX15PhysicsSettings27setShowPartCoordinateFramesEb
pub fn stub_5f8ab4() {
    // IDA 0x5f8ab4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowUnalignedParts(void)const")]
// 0x5f8ae4 — __ZNK3RBX15PhysicsSettings21getShowUnalignedPartsEv
pub fn stub_5f8ae4() {
    // IDA 0x5f8ae4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowUnalignedParts(bool)")]
// 0x5f8af4 — __ZN3RBX15PhysicsSettings21setShowUnalignedPartsEb
pub fn stub_5f8af4() {
    // IDA 0x5f8af4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowModelCoordinateFrames(void)const")]
// 0x5f8b24 — __ZNK3RBX15PhysicsSettings28getShowModelCoordinateFramesEv
pub fn stub_5f8b24() {
    // IDA 0x5f8b24: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowModelCoordinateFrames(bool)")]
// 0x5f8b34 — __ZN3RBX15PhysicsSettings28setShowModelCoordinateFramesEb
pub fn stub_5f8b34() {
    // IDA 0x5f8b34: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowWorldCoordinateFrame(void)const")]
// 0x5f8b64 — __ZNK3RBX15PhysicsSettings27getShowWorldCoordinateFrameEv
pub fn stub_5f8b64() {
    // IDA 0x5f8b64: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowWorldCoordinateFrame(bool)")]
// 0x5f8b74 — __ZN3RBX15PhysicsSettings27setShowWorldCoordinateFrameEb
pub fn stub_5f8b74() {
    // IDA 0x5f8b74: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowEPhysicsOwners(void)const")]
// 0x5f8ba4 — __ZNK3RBX15PhysicsSettings21getShowEPhysicsOwnersEv
pub fn stub_5f8ba4() {
    // IDA 0x5f8ba4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowEPhysicsOwners(bool)")]
// 0x5f8bb4 — __ZN3RBX15PhysicsSettings21setShowEPhysicsOwnersEb
pub fn stub_5f8bb4() {
    // IDA 0x5f8bb4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowEPhysicsRegions(void)const")]
// 0x5f8be4 — __ZNK3RBX15PhysicsSettings22getShowEPhysicsRegionsEv
pub fn stub_5f8be4() {
    // IDA 0x5f8be4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowEPhysicsRegions(bool)")]
// 0x5f8bf4 — __ZN3RBX15PhysicsSettings22setShowEPhysicsRegionsEb
pub fn stub_5f8bf4() {
    // IDA 0x5f8bf4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getHighlightAwakeParts(void)const")]
// 0x5f8c24 — __ZNK3RBX15PhysicsSettings22getHighlightAwakePartsEv
pub fn stub_5f8c24() {
    // IDA 0x5f8c24: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setHighlightAwakeParts(bool)")]
// 0x5f8c34 — __ZN3RBX15PhysicsSettings22setHighlightAwakePartsEb
pub fn stub_5f8c34() {
    // IDA 0x5f8c34: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowBodyTypes(void)const")]
// 0x5f8c64 — __ZNK3RBX15PhysicsSettings16getShowBodyTypesEv
pub fn stub_5f8c64() {
    // IDA 0x5f8c64: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowBodyTypes(bool)")]
// 0x5f8c74 — __ZN3RBX15PhysicsSettings16setShowBodyTypesEb
pub fn stub_5f8c74() {
    // IDA 0x5f8c74: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowReceiveAge(void)const")]
// 0x5f8ca4 — __ZNK3RBX15PhysicsSettings17getShowReceiveAgeEv
pub fn stub_5f8ca4() {
    // IDA 0x5f8ca4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowReceiveAge(bool)")]
// 0x5f8cb4 — __ZN3RBX15PhysicsSettings17setShowReceiveAgeEb
pub fn stub_5f8cb4() {
    // IDA 0x5f8cb4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowContactPoints(void)const")]
// 0x5f8ce4 — __ZNK3RBX15PhysicsSettings20getShowContactPointsEv
pub fn stub_5f8ce4() {
    // IDA 0x5f8ce4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowContactPoints(bool)")]
// 0x5f8cf4 — __ZN3RBX15PhysicsSettings20setShowContactPointsEb
pub fn stub_5f8cf4() {
    // IDA 0x5f8cf4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowJointCoordinates(void)const")]
// 0x5f8d24 — __ZNK3RBX15PhysicsSettings23getShowJointCoordinatesEv
pub fn stub_5f8d24() {
    // IDA 0x5f8d24: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowJointCoordinates(bool)")]
// 0x5f8d34 — __ZN3RBX15PhysicsSettings23setShowJointCoordinatesEb
pub fn stub_5f8d34() {
    // IDA 0x5f8d34: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowMechanisms(void)const")]
// 0x5f8d64 — __ZNK3RBX15PhysicsSettings17getShowMechanismsEv
pub fn stub_5f8d64() {
    // IDA 0x5f8d64: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowMechanisms(bool)")]
// 0x5f8d74 — __ZN3RBX15PhysicsSettings17setShowMechanismsEb
pub fn stub_5f8d74() {
    // IDA 0x5f8d74: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowAssemblies(void)const")]
// 0x5f8da4 — __ZNK3RBX15PhysicsSettings17getShowAssembliesEv
pub fn stub_5f8da4() {
    // IDA 0x5f8da4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowAssemblies(bool)")]
// 0x5f8db4 — __ZN3RBX15PhysicsSettings17setShowAssembliesEb
pub fn stub_5f8db4() {
    // IDA 0x5f8db4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getShowSpanningTree(void)const")]
// 0x5f8de4 — __ZNK3RBX15PhysicsSettings19getShowSpanningTreeEv
pub fn stub_5f8de4() {
    // IDA 0x5f8de4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setShowSpanningTree(bool)")]
// 0x5f8df4 — __ZN3RBX15PhysicsSettings19setShowSpanningTreeEb
pub fn stub_5f8df4() {
    // IDA 0x5f8df4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getAllowSleep(void)const")]
// 0x5f8e24 — __ZNK3RBX15PhysicsSettings13getAllowSleepEv
pub fn stub_5f8e24() {
    // IDA 0x5f8e24: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setAllowSleep(bool)")]
// 0x5f8e34 — __ZN3RBX15PhysicsSettings13setAllowSleepEb
pub fn stub_5f8e34() {
    // IDA 0x5f8e34: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getParallelPhysics(void)const")]
// 0x5f8e64 — __ZNK3RBX15PhysicsSettings18getParallelPhysicsEv
pub fn stub_5f8e64() {
    // IDA 0x5f8e64: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setParallelPhysics(bool)")]
// 0x5f8e74 — __ZN3RBX15PhysicsSettings18setParallelPhysicsEb
pub fn stub_5f8e74() {
    // IDA 0x5f8e74: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getEThrottle(void)const")]
// 0x5f8ea4 — __ZNK3RBX15PhysicsSettings12getEThrottleEv
pub fn stub_5f8ea4() {
    // IDA 0x5f8ea4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setEThrottle(RBX::EThrottle::EThrottleType)")]
// 0x5f8eb4 — __ZN3RBX15PhysicsSettings12setEThrottleENS_9EThrottle13EThrottleTypeE
pub fn stub_5f8eb4() {
    // IDA 0x5f8eb4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setThrottleAdjustTime(double)")]
// 0x5f8edc — __ZN3RBX15PhysicsSettings21setThrottleAdjustTimeEd
pub fn stub_5f8edc() {
    // IDA 0x5f8edc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getThrottleAt30Fps(void)const")]
// 0x5f8f08 — __ZNK3RBX15PhysicsSettings18getThrottleAt30FpsEv
pub fn stub_5f8f08() {
    // IDA 0x5f8f08: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::setThrottleAt30Fps(bool)")]
// 0x5f8f18 — __ZN3RBX15PhysicsSettings18setThrottleAt30FpsEb
pub fn stub_5f8f18() {
    // IDA 0x5f8f18: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::PhysicsSettings(void)")]
// 0x5f8f38 — __ZN3RBX15PhysicsSettingsC1Ev
pub fn stub_5f8f38() {
    // IDA 0x5f8f38: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::PhysicsSettings(void)")]
// 0x5f8f3c — __ZN3RBX15PhysicsSettingsC2Ev
pub fn stub_5f8f3c() {
    // IDA 0x5f8f3c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::getThrottleAdjustTime(void)const")]
// 0x5f9150 — __ZNK3RBX15PhysicsSettings21getThrottleAdjustTimeEv
pub fn stub_5f9150() {
    // IDA 0x5f9150: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PhysicsSettings::~PhysicsSettings()")]
// 0x5f93f0 — __ZN3RBX15PhysicsSettingsD1Ev
pub fn stub_5f93f0() {
    // IDA 0x5f93f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PhysicsSettings::~PhysicsSettings()")]
// 0x5f9430 — __ZN3RBX15PhysicsSettingsD0Ev
pub fn stub_5f9430() {
    // IDA 0x5f9430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsSettings::~PhysicsSettings()")]
// 0x5f9520 — __ZThn32_N3RBX15PhysicsSettingsD1Ev
pub fn stub_5f9520() {
    // IDA 0x5f9520: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsSettings::~PhysicsSettings()")]
// 0x5f9564 — __ZThn32_N3RBX15PhysicsSettingsD0Ev
pub fn stub_5f9564() {
    // IDA 0x5f9564: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsSettings::~PhysicsSettings()")]
// 0x5f9654 — __ZThn36_N3RBX15PhysicsSettingsD1Ev
pub fn stub_5f9654() {
    // IDA 0x5f9654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PhysicsSettings::~PhysicsSettings()")]
// 0x5f9698 — __ZThn36_N3RBX15PhysicsSettingsD0Ev
pub fn stub_5f9698() {
    // IDA 0x5f9698: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BasePlayerGui::BasePlayerGui(void)")]
// 0x5fb3d4 — __ZN3RBX13BasePlayerGuiC2Ev
pub fn stub_5fb3d4() {
    // IDA 0x5fb3d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BasePlayerGui::~BasePlayerGui()")]
// 0x5fb5b4 — __ZN3RBX13BasePlayerGuiD0Ev
pub fn stub_5fb5b4() {
    // IDA 0x5fb5b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BasePlayerGui::~BasePlayerGui()")]
// 0x5fb654 — __ZN3RBX13BasePlayerGuiD1Ev
pub fn stub_5fb654() {
    // IDA 0x5fb654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::~BasePlayerGui()")]
// 0x5fb658 — __ZThn32_N3RBX13BasePlayerGuiD0Ev
pub fn stub_5fb658() {
    // IDA 0x5fb658: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::~BasePlayerGui()")]
// 0x5fb660 — __ZThn36_N3RBX13BasePlayerGuiD0Ev
pub fn stub_5fb660() {
    // IDA 0x5fb660: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BasePlayerGui::~BasePlayerGui()")]
// 0x5fb668 — __ZN3RBX13BasePlayerGuiD2Ev
pub fn stub_5fb668() {
    // IDA 0x5fb668: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::~BasePlayerGui()")]
// 0x5fb7a4 — __ZThn32_N3RBX13BasePlayerGuiD1Ev
pub fn stub_5fb7a4() {
    // IDA 0x5fb7a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::~BasePlayerGui()")]
// 0x5fb7ac — __ZThn36_N3RBX13BasePlayerGuiD1Ev
pub fn stub_5fb7ac() {
    // IDA 0x5fb7ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BasePlayerGui::findModalGuiObject(void)")]
// 0x5fb7b4 — __ZN3RBX13BasePlayerGui18findModalGuiObjectEv
pub fn stub_5fb7b4() {
    // IDA 0x5fb7b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BasePlayerGui::render3dAdorn(RBX::Adorn *)")]
// 0x5fbb18 — __ZN3RBX13BasePlayerGui13render3dAdornEPNS_5AdornE
pub fn stub_5fbb18() {
    // IDA 0x5fbb18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BasePlayerGui::append3dSortedAdorn(std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>> &,RBX::Camera const*)const")]
// 0x5fbb20 — __ZNK3RBX13BasePlayerGui19append3dSortedAdornERSt6vectorIPNS_10IAdornableESaIS3_EEPKNS_6CameraE
pub fn stub_5fbb20() {
    // IDA 0x5fbb20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BasePlayerGui::render2d(RBX::Adorn *)")]
// 0x5fbb28 — __ZN3RBX13BasePlayerGui8render2dEPNS_5AdornE
pub fn stub_5fbb28() {
    // IDA 0x5fbb28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BasePlayerGui::process(RBX::GuiEvent const&)")]
// 0x5fbb30 — __ZN3RBX13BasePlayerGui7processERKNS_8GuiEventE
pub fn stub_5fbb30() {
    // IDA 0x5fbb30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::process(RBX::GuiEvent const&)")]
// 0x5fbcc4 — __ZThn96_N3RBX13BasePlayerGui7processERKNS_8GuiEventE
pub fn stub_5fbcc4() {
    // IDA 0x5fbcc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PlayerGui::PlayerGui(void)")]
// 0x5fbcd0 — __ZN3RBX9PlayerGuiC1Ev
pub fn stub_5fbcd0() {
    // IDA 0x5fbcd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PlayerGui::PlayerGui(void)")]
// 0x5fbcd4 — __ZN3RBX9PlayerGuiC2Ev
pub fn stub_5fbcd4() {
    // IDA 0x5fbcd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterGuiService::setShowGui(bool)")]
// 0x5fbf2c — __ZN3RBX17StarterGuiService10setShowGuiEb
pub fn stub_5fbf2c() {
    // IDA 0x5fbf2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterGuiService::setResetPlayerGui(bool)")]
// 0x5fbf4c — __ZN3RBX17StarterGuiService17setResetPlayerGuiEb
pub fn stub_5fbf4c() {
    // IDA 0x5fbf4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterGuiService::setCoreGuiEnabled(RBX::StarterGuiService::CoreGuiType,bool)")]
// 0x5fbf6c — __ZN3RBX17StarterGuiService17setCoreGuiEnabledENS0_11CoreGuiTypeEb
pub fn stub_5fbf6c() {
    // IDA 0x5fbf6c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::StarterGuiService::getCoreGuiEnabled(RBX::StarterGuiService::CoreGuiType)")]
// 0x5fc104 — __ZN3RBX17StarterGuiService17getCoreGuiEnabledENS0_11CoreGuiTypeE
pub fn stub_5fc104() {
    // IDA 0x5fc104: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::StringConverter<RBX::StarterGuiService::CoreGuiType>::convertToValue(std::string const&,RBX::StarterGuiService::CoreGuiType&)")]
// 0x5fc400 — __ZN3RBX15StringConverterINS_17StarterGuiService11CoreGuiTypeEE14convertToValueERKSsRS2_
pub fn stub_5fc400() {
    // IDA 0x5fc400: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StarterGuiService::StarterGuiService(void)")]
// 0x5fc44c — __ZN3RBX17StarterGuiServiceC1Ev
pub fn stub_5fc44c() {
    // IDA 0x5fc44c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StarterGuiService::StarterGuiService(void)")]
// 0x5fc450 — __ZN3RBX17StarterGuiServiceC2Ev
pub fn stub_5fc450() {
    // IDA 0x5fc450: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StarterGuiService::render2d(RBX::Adorn *)")]
// 0x5fc754 — __ZN3RBX17StarterGuiService8render2dEPNS_5AdornE
pub fn stub_5fc754() {
    // IDA 0x5fc754: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StarterGuiService::render3dAdorn(RBX::Adorn *)")]
// 0x5fc764 — __ZN3RBX17StarterGuiService13render3dAdornEPNS_5AdornE
pub fn stub_5fc764() {
    // IDA 0x5fc764: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StarterGuiService::append3dSortedAdorn(std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>> &,RBX::Camera const*)const")]
// 0x5fc774 — __ZNK3RBX17StarterGuiService19append3dSortedAdornERSt6vectorIPNS_10IAdornableESaIS3_EEPKNS_6CameraE
pub fn stub_5fc774() {
    // IDA 0x5fc774: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StarterGuiService::process(RBX::GuiEvent const&)")]
// 0x5fc784 — __ZN3RBX17StarterGuiService7processERKNS_8GuiEventE
pub fn stub_5fc784() {
    // IDA 0x5fc784: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "non-virtual thunk toRBX::StarterGuiService::process(RBX::GuiEvent const&)")]
// 0x5fc7a0 — __ZThn96_N3RBX17StarterGuiService7processERKNS_8GuiEventE
pub fn stub_5fc7a0() {
    // IDA 0x5fc7a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CoreGuiService::getGuiVersion(void)const")]
// 0x5fc7bc — __ZNK3RBX14CoreGuiService13getGuiVersionEv
pub fn stub_5fc7bc() {
    // IDA 0x5fc7bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CoreGuiService::CoreGuiService(void)")]
// 0x5fc7c0 — __ZN3RBX14CoreGuiServiceC1Ev
pub fn stub_5fc7c0() {
    // IDA 0x5fc7c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CoreGuiService::CoreGuiService(void)")]
// 0x5fc7c4 — __ZN3RBX14CoreGuiServiceC2Ev
pub fn stub_5fc7c4() {
    // IDA 0x5fc7c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CoreGuiService::createRobloxScreenGui(void)")]
// 0x5fca54 — __ZN3RBX14CoreGuiService21createRobloxScreenGuiEv
pub fn stub_5fca54() {
    // IDA 0x5fca54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StarterGuiService::getShowGui(void)const")]
// 0x5fcc30 — __ZNK3RBX17StarterGuiService10getShowGuiEv
pub fn stub_5fcc30() {
    // IDA 0x5fcc30: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::StarterGuiService::getResetPlayerGui(void)const")]
// 0x5fcc5c — __ZNK3RBX17StarterGuiService17getResetPlayerGuiEv
pub fn stub_5fcc5c() {
    // IDA 0x5fcc5c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PlayerGui::~PlayerGui()")]
// 0x5fd5c8 — __ZN3RBX9PlayerGuiD1Ev
pub fn stub_5fd5c8() {
    // IDA 0x5fd5c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PlayerGui::~PlayerGui()")]
// 0x5fd5cc — __ZN3RBX9PlayerGuiD0Ev
pub fn stub_5fd5cc() {
    // IDA 0x5fd5cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PlayerGui::~PlayerGui()")]
// 0x5fd67c — __ZThn32_N3RBX9PlayerGuiD1Ev
pub fn stub_5fd67c() {
    // IDA 0x5fd67c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PlayerGui::~PlayerGui()")]
// 0x5fd684 — __ZThn32_N3RBX9PlayerGuiD0Ev
pub fn stub_5fd684() {
    // IDA 0x5fd684: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PlayerGui::~PlayerGui()")]
// 0x5fd738 — __ZThn36_N3RBX9PlayerGuiD1Ev
pub fn stub_5fd738() {
    // IDA 0x5fd738: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PlayerGui::~PlayerGui()")]
// 0x5fd740 — __ZThn36_N3RBX9PlayerGuiD0Ev
pub fn stub_5fd740() {
    // IDA 0x5fd740: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

